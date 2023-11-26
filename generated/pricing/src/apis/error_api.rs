/*
 * SaaSus Pricing API Schema
 *
 * SaaSus Pricing API Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::pin::Pin;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct ErrorApiClient<C: hyper::client::connect::Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> ErrorApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ErrorApiClient<C> {
        ErrorApiClient {
            configuration,
        }
    }
}

pub trait ErrorApi {
    fn return_internal_server_error(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
}

impl<C: hyper::client::connect::Connect>ErrorApi for ErrorApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn return_internal_server_error(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/errors/internal-server-error".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}
