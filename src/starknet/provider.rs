use reqwest::Url;
use starknet::providers::SequencerGatewayProvider;

pub struct StarkNetProvider {
    pub provider: SequencerGatewayProvider,
}

impl StarkNetProvider {
    pub fn new() -> Self {
        let gateway_url = dotenv::var("STARKNET_GATEWAY_URL")
            .expect("environment variable missing : STARKNET_GATEWAY_URL");

        let feeder_gateway_url = dotenv::var("STARKNET_FEEDER_GATEWAY_URL")
            .expect("environment variable missing : STARKNET_FEEDER_GATEWAY_URL");

        let provider = SequencerGatewayProvider::new(
            Url::parse(&gateway_url).unwrap(),
            Url::parse(&feeder_gateway_url).unwrap(),
        );

        Self { provider }
    }
}
