/*
 * SaaSus Communication API Schema
 *
 * SaaSus Communication API Schema
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

pub struct FeedbackApiClient<C: hyper::client::connect::Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> FeedbackApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> FeedbackApiClient<C> {
        FeedbackApiClient {
            configuration,
        }
    }
}

pub trait FeedbackApi {
    fn create_feedback(&self, create_feedback_param: Option<crate::models::CreateFeedbackParam>) -> Pin<Box<dyn Future<Output = Result<crate::models::Feedback, Error>>>>;
    fn create_feedback_comment(&self, feedback_id: &str, create_feedback_comment_param: Option<crate::models::CreateFeedbackCommentParam>) -> Pin<Box<dyn Future<Output = Result<crate::models::Comment, Error>>>>;
    fn create_vote_user(&self, feedback_id: &str, create_vote_user_param: Option<crate::models::CreateVoteUserParam>) -> Pin<Box<dyn Future<Output = Result<crate::models::Votes, Error>>>>;
    fn delete_feedback(&self, feedback_id: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn delete_feedback_comment(&self, feedback_id: &str, comment_id: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn delete_vote_for_feedback(&self, feedback_id: &str, user_id: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn get_feedback(&self, feedback_id: &str) -> Pin<Box<dyn Future<Output = Result<crate::models::Feedback, Error>>>>;
    fn get_feedback_comment(&self, feedback_id: &str, comment_id: &str) -> Pin<Box<dyn Future<Output = Result<crate::models::Comment, Error>>>>;
    fn get_feedbacks(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::Feedbacks, Error>>>>;
    fn update_feedback(&self, feedback_id: &str, update_feedback_param: Option<crate::models::UpdateFeedbackParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn update_feedback_comment(&self, feedback_id: &str, comment_id: &str, update_feedback_comment_param: Option<crate::models::UpdateFeedbackCommentParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn update_feedback_status(&self, feedback_id: &str, update_feedback_status_param: Option<crate::models::UpdateFeedbackStatusParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
}

impl<C: hyper::client::connect::Connect>FeedbackApi for FeedbackApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn create_feedback(&self, create_feedback_param: Option<crate::models::CreateFeedbackParam>) -> Pin<Box<dyn Future<Output = Result<crate::models::Feedback, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/feedbacks".to_string())
        ;
        req = req.with_body_param(create_feedback_param);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn create_feedback_comment(&self, feedback_id: &str, create_feedback_comment_param: Option<crate::models::CreateFeedbackCommentParam>) -> Pin<Box<dyn Future<Output = Result<crate::models::Comment, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/feedbacks/{feedback_id}/comments".to_string())
        ;
        req = req.with_path_param("feedback_id".to_string(), feedback_id.to_string());
        req = req.with_body_param(create_feedback_comment_param);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn create_vote_user(&self, feedback_id: &str, create_vote_user_param: Option<crate::models::CreateVoteUserParam>) -> Pin<Box<dyn Future<Output = Result<crate::models::Votes, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/feedbacks/{feedback_id}/votes/users".to_string())
        ;
        req = req.with_path_param("feedback_id".to_string(), feedback_id.to_string());
        req = req.with_body_param(create_vote_user_param);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_feedback(&self, feedback_id: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/feedbacks/{feedback_id}".to_string())
        ;
        req = req.with_path_param("feedback_id".to_string(), feedback_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_feedback_comment(&self, feedback_id: &str, comment_id: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/feedbacks/{feedback_id}/comments/{comment_id}".to_string())
        ;
        req = req.with_path_param("feedback_id".to_string(), feedback_id.to_string());
        req = req.with_path_param("comment_id".to_string(), comment_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_vote_for_feedback(&self, feedback_id: &str, user_id: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/feedbacks/{feedback_id}/votes/users/{user_id}".to_string())
        ;
        req = req.with_path_param("feedback_id".to_string(), feedback_id.to_string());
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_feedback(&self, feedback_id: &str) -> Pin<Box<dyn Future<Output = Result<crate::models::Feedback, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/feedbacks/{feedback_id}".to_string())
        ;
        req = req.with_path_param("feedback_id".to_string(), feedback_id.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_feedback_comment(&self, feedback_id: &str, comment_id: &str) -> Pin<Box<dyn Future<Output = Result<crate::models::Comment, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/feedbacks/{feedback_id}/comments/{comment_id}".to_string())
        ;
        req = req.with_path_param("feedback_id".to_string(), feedback_id.to_string());
        req = req.with_path_param("comment_id".to_string(), comment_id.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_feedbacks(&self, ) -> Pin<Box<dyn Future<Output = Result<crate::models::Feedbacks, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/feedbacks".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_feedback(&self, feedback_id: &str, update_feedback_param: Option<crate::models::UpdateFeedbackParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::PATCH, "/feedbacks/{feedback_id}".to_string())
        ;
        req = req.with_path_param("feedback_id".to_string(), feedback_id.to_string());
        req = req.with_body_param(update_feedback_param);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_feedback_comment(&self, feedback_id: &str, comment_id: &str, update_feedback_comment_param: Option<crate::models::UpdateFeedbackCommentParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::PATCH, "/feedbacks/{feedback_id}/comments/{comment_id}".to_string())
        ;
        req = req.with_path_param("feedback_id".to_string(), feedback_id.to_string());
        req = req.with_path_param("comment_id".to_string(), comment_id.to_string());
        req = req.with_body_param(update_feedback_comment_param);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_feedback_status(&self, feedback_id: &str, update_feedback_status_param: Option<crate::models::UpdateFeedbackStatusParam>) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::PATCH, "/feedbacks/{feedback_id}/status".to_string())
        ;
        req = req.with_path_param("feedback_id".to_string(), feedback_id.to_string());
        req = req.with_body_param(update_feedback_status_param);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}
