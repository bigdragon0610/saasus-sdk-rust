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

pub struct BasicInfoApiClient<C: hyper::client::connect::Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> BasicInfoApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> BasicInfoApiClient<C> {
        BasicInfoApiClient {
            configuration,
        }
    }
}

pub trait BasicInfoApi {
    fn find_notification_messages(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::NotificationMessages, Error>>>>;
    fn get_basic_info(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::BasicInfo, Error>>>>;
    fn get_customize_page_settings(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::CustomizePageSettings, Error>>>>;
    fn get_customize_pages(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::CustomizePages, Error>>>>;
    fn update_basic_info(&self, update_basic_info_param: Option<crate::models::UpdateBasicInfoParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn update_customize_page_settings(&self, update_customize_page_settings_param: Option<crate::models::UpdateCustomizePageSettingsParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn update_customize_pages(&self, update_customize_pages_param: Option<crate::models::UpdateCustomizePagesParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn update_notification_messages(&self, update_notification_messages_param: Option<crate::models::UpdateNotificationMessagesParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
}

impl<C: hyper::client::connect::Connect>BasicInfoApi for BasicInfoApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn find_notification_messages(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::NotificationMessages, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/notification-messages".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_basic_info(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::BasicInfo, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/basic-info".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_customize_page_settings(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::CustomizePageSettings, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/customize-page-settings".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_customize_pages(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::CustomizePages, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/customize-pages".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_basic_info(&self, update_basic_info_param: Option<crate::models::UpdateBasicInfoParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/basic-info".to_string())
        ;
        req = req.with_body_param(update_basic_info_param);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_customize_page_settings(&self, update_customize_page_settings_param: Option<crate::models::UpdateCustomizePageSettingsParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::PATCH, "/customize-page-settings".to_string())
        ;
        req = req.with_body_param(update_customize_page_settings_param);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_customize_pages(&self, update_customize_pages_param: Option<crate::models::UpdateCustomizePagesParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::PATCH, "/customize-pages".to_string())
        ;
        req = req.with_body_param(update_customize_pages_param);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_notification_messages(&self, update_notification_messages_param: Option<crate::models::UpdateNotificationMessagesParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/notification-messages".to_string())
        ;
        req = req.with_body_param(update_notification_messages_param);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}
