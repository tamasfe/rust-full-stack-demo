use crate::{error::ClientError, grpc_response::GrpcResponse};
use bytes::{Bytes, BytesMut};
use http::{
    header::{ACCEPT, CONTENT_TYPE},
    HeaderValue, Request, Response,
};
use http_body::{combinators::UnsyncBoxBody, Body};
use reqwest::IntoUrl;
use std::{future::Future, pin::Pin};
use tonic::body::BoxBody;
use tower::Service;
use url::Url;

/// `grpc-web` based transport layer for `tonic` clients
#[derive(Debug, Clone)]
pub struct GrpcWebClient {
    http: reqwest::Client,
    base_url: Url,
}

impl GrpcWebClient {
    /// Creates a new client
    pub fn new(address: impl IntoUrl) -> Self {
        Self::new_with_client(address, reqwest::ClientBuilder::new()).unwrap()
    }

    pub fn new_with_client(
        address: impl IntoUrl,
        client: reqwest::ClientBuilder,
    ) -> Result<Self, reqwest::Error> {
        Ok(Self {
            http: client.build()?,
            base_url: address.into_url()?,
        })
    }
}

impl Service<Request<BoxBody>> for GrpcWebClient {
    type Response = Response<UnsyncBoxBody<Bytes, ClientError>>;

    type Error = ClientError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<BoxBody>) -> Self::Future {
        Box::pin(request(self.http.clone(), self.base_url.clone(), req))
    }
}

async fn request(
    http: reqwest::Client,
    mut url: Url,
    req: Request<BoxBody>,
) -> Result<Response<UnsyncBoxBody<Bytes, ClientError>>, ClientError> {
    url = url.join(&req.uri().to_string())?;

    let mut builder = http.post(url);

    for (header_name, header_value) in req.headers().iter() {
        if header_name.as_str() == "content-type" {
            continue;
        }
        builder = builder.header(header_name.as_str(), header_value.to_str()?);
    }

    builder = builder
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/grpc-web+proto"),
        )
        .header(
            ACCEPT,
            HeaderValue::from_static("application/grpc-web+proto"),
        )
        .header("x-grpc-web", HeaderValue::from_static("1"));

    let body = req.into_body().data().await;
    if let Some(body) = body {
        builder = builder.body(body?);
    }

    let response = builder.send().await?;

    let mut result = Response::builder();
    result = result.status(response.status());

    for (header_name, header_value) in response.headers().iter() {
        result = result.header(header_name.as_str(), header_value.to_str()?);
    }

    let content_type = match response.headers().get(CONTENT_TYPE) {
        None => Err(ClientError::MissingContentTypeHeader),
        Some(content_type) => content_type.to_str().map_err(Into::into),
    }?
    .to_owned();

    let bytes = BytesMut::from(response.bytes().await?.as_ref());
    let body = UnsyncBoxBody::new(GrpcResponse::new(bytes, &content_type)?);

    result.body(body).map_err(Into::into)
}
