use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    error_api: Box<dyn crate::apis::ErrorApi>,
    stripe_api: Box<dyn crate::apis::StripeApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Rc::new(configuration);

        APIClient {
            error_api: Box::new(crate::apis::ErrorApiClient::new(rc.clone())),
            stripe_api: Box::new(crate::apis::StripeApiClient::new(rc.clone())),
        }
    }

    pub fn error_api(&self) -> &dyn crate::apis::ErrorApi{
        self.error_api.as_ref()
    }

    pub fn stripe_api(&self) -> &dyn crate::apis::StripeApi{
        self.stripe_api.as_ref()
    }

}
