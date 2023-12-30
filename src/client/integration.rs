use hyper::Client;
use hyper_tls::HttpsConnector;
use integration::apis::{client::APIClient, configuration::Configuration};

pub fn create_client() -> APIClient {
    let connector = HttpsConnector::new();
    let client = Client::builder().build(connector);
    let configuration = Configuration::new(client);
    APIClient::new(configuration)
}
