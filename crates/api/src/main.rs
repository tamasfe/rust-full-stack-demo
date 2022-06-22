use async_trait::async_trait;
use rpc::{
    example::{
        example_service_server::{ExampleService, ExampleServiceServer},
        SumRequest, SumResponse,
    },
    tonic::{transport::Server, Request, Response, Status},
};
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

struct Foo;

#[async_trait]
impl ExampleService for Foo {
    async fn add_numbers(
        &self,
        request: Request<SumRequest>,
    ) -> Result<Response<SumResponse>, Status> {
        let request = request.into_inner();

        Ok(Response::new(SumResponse {
            sum: request.numbers.into_iter().sum(),
        }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::Registry::default()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let layer = tower::ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_grpc()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(tracing::Level::INFO)
                        .latency_unit(LatencyUnit::Seconds),
                ),
        )
        .into_inner();

    Server::builder()
        .accept_http1(true)
        .layer(layer)
        .add_service(tonic_web::enable(ExampleServiceServer::new(Foo)))
        .serve("0.0.0.0:8081".parse().unwrap())
        .await?;

    Ok(())
}
