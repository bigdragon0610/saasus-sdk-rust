use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    aws_marketplace_api: Box<dyn crate::apis::AwsMarketplaceApi>,
    error_api: Box<dyn crate::apis::ErrorApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Rc::new(configuration);

        APIClient {
            aws_marketplace_api: Box::new(crate::apis::AwsMarketplaceApiClient::new(rc.clone())),
            error_api: Box::new(crate::apis::ErrorApiClient::new(rc.clone())),
        }
    }

    pub fn aws_marketplace_api(&self) -> &dyn crate::apis::AwsMarketplaceApi{
        self.aws_marketplace_api.as_ref()
    }

    pub fn error_api(&self) -> &dyn crate::apis::ErrorApi{
        self.error_api.as_ref()
    }

}
