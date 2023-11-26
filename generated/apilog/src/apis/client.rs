use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    api_log_api: Box<dyn crate::apis::ApiLogApi>,
    error_api: Box<dyn crate::apis::ErrorApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Rc::new(configuration);

        APIClient {
            api_log_api: Box::new(crate::apis::ApiLogApiClient::new(rc.clone())),
            error_api: Box::new(crate::apis::ErrorApiClient::new(rc.clone())),
        }
    }

    pub fn api_log_api(&self) -> &dyn crate::apis::ApiLogApi{
        self.api_log_api.as_ref()
    }

    pub fn error_api(&self) -> &dyn crate::apis::ErrorApi{
        self.error_api.as_ref()
    }

}
