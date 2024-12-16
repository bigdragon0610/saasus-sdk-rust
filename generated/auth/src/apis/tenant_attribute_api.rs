/*
 * SaaSus Auth API Schema
 *
 * Schema
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

pub struct TenantAttributeApiClient<C: hyper::client::connect::Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> TenantAttributeApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TenantAttributeApiClient<C> {
        TenantAttributeApiClient {
            configuration,
        }
    }
}

pub trait TenantAttributeApi {
    fn create_tenant_attribute(&self, body: Option<crate::models::Attribute>) -> Pin<Box<dyn Future<Output = Result<crate::models::Attribute, Error>>>>;
    fn delete_tenant_attribute(&self, attribute_name: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn get_tenant_attributes(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::TenantAttributes, Error>>>>;
}

impl<C: hyper::client::connect::Connect>TenantAttributeApi for TenantAttributeApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn create_tenant_attribute(&self, body: Option<crate::models::Attribute>) -> Pin<Box<dyn Future<Output = Result<crate::models::Attribute, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/tenant-attributes".to_string())
        ;
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_tenant_attribute(&self, attribute_name: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/tenant-attributes/{attribute_name}".to_string())
        ;
        req = req.with_path_param("attribute_name".to_string(), attribute_name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_tenant_attributes(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::TenantAttributes, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/tenant-attributes".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}
