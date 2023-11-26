use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    error_api: Box<dyn crate::apis::ErrorApi>,
    event_bridge_api: Box<dyn crate::apis::EventBridgeApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Rc::new(configuration);

        APIClient {
            error_api: Box::new(crate::apis::ErrorApiClient::new(rc.clone())),
            event_bridge_api: Box::new(crate::apis::EventBridgeApiClient::new(rc.clone())),
        }
    }

    pub fn error_api(&self) -> &dyn crate::apis::ErrorApi{
        self.error_api.as_ref()
    }

    pub fn event_bridge_api(&self) -> &dyn crate::apis::EventBridgeApi{
        self.event_bridge_api.as_ref()
    }

}
