use awsmarketplace::apis::{client::APIClient, configuration::Configuration};
use hyper::Client;
use hyper_tls::HttpsConnector;

pub fn create_client() -> APIClient {
    let connector = HttpsConnector::new();
    let client = Client::builder().build(connector);
    let configuration = Configuration::new(client);
    APIClient::new(configuration)
}
