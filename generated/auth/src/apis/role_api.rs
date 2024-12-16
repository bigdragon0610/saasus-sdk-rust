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

pub struct RoleApiClient<C: hyper::client::connect::Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> RoleApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> RoleApiClient<C> {
        RoleApiClient {
            configuration,
        }
    }
}

pub trait RoleApi {
    fn create_role(&self, body: Option<crate::models::Role>) -> Pin<Box<dyn Future<Output = Result<crate::models::Role, Error>>>>;
    fn delete_role(&self, role_name: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn get_roles(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::Roles, Error>>>>;
}

impl<C: hyper::client::connect::Connect>RoleApi for RoleApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn create_role(&self, body: Option<crate::models::Role>) -> Pin<Box<dyn Future<Output = Result<crate::models::Role, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/roles".to_string())
        ;
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_role(&self, role_name: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/roles/{role_name}".to_string())
        ;
        req = req.with_path_param("role_name".to_string(), role_name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_roles(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::Roles, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/roles".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}
