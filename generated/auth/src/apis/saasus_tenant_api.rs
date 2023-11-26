/*
 * SaaSus Auth API Schema
 *
 * スキーマ
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

pub struct SaasusTenantApiClient<C: hyper::client::connect::Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> SaasusTenantApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SaasusTenantApiClient<C> {
        SaasusTenantApiClient {
            configuration,
        }
    }
}

pub trait SaasusTenantApi {
    fn create_api_key(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn delete_api_key(&self, api_key: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn get_api_keys(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::ApiKeys, Error>>>>;
    fn get_client_secret(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::ClientSecret, Error>>>>;
    fn get_saas_id(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::SaasId, Error>>>>;
    fn update_client_secret(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn update_saas_id(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
}

impl<C: hyper::client::connect::Connect>SaasusTenantApi for SaasusTenantApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn create_api_key(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/apikeys".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_api_key(&self, api_key: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/apikeys/{api_key}".to_string())
        ;
        req = req.with_path_param("api_key".to_string(), api_key.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_api_keys(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::ApiKeys, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/apikeys".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_client_secret(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::ClientSecret, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/client-secret".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_saas_id(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::SaasId, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/saasid".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_client_secret(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::PATCH, "/client-secret".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_saas_id(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::PATCH, "/saasid".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}
