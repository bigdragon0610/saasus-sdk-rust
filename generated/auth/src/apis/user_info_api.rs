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

pub struct UserInfoApiClient<C: hyper::client::connect::Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> UserInfoApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> UserInfoApiClient<C> {
        UserInfoApiClient {
            configuration,
        }
    }
}

pub trait UserInfoApi {
    fn get_user_info(&self, token: &str) -> Pin<Box<dyn Future<Output = Result<crate::models::UserInfo, Error>>>>;
    fn get_user_info_by_email(&self, email: &str) -> Pin<Box<dyn Future<Output = Result<crate::models::UserInfo, Error>>>>;
}

impl<C: hyper::client::connect::Connect>UserInfoApi for UserInfoApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn get_user_info(&self, token: &str) -> Pin<Box<dyn Future<Output = Result<crate::models::UserInfo, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/userinfo".to_string())
        ;
        req = req.with_query_param("token".to_string(), token.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_user_info_by_email(&self, email: &str) -> Pin<Box<dyn Future<Output = Result<crate::models::UserInfo, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/userinfo/search/email".to_string())
        ;
        req = req.with_query_param("email".to_string(), email.to_string());

        req.execute(self.configuration.borrow())
    }

}
