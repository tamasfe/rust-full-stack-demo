use rpc::example::example_service_client::ExampleServiceClient;
use tonic_web_client::GrpcWebClient;

#[derive(Debug, Clone)]
pub struct Api {
    pub example: ExampleServiceClient<GrpcWebClient>,
}

impl Api {
    pub fn new() -> Self {
        Self {
            example: ExampleServiceClient::new(GrpcWebClient::new("http://127.0.0.1:8081")),
        }
    }
}
