use http;
use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Header(hyper::http::header::InvalidHeaderValue),
    Http(http::Error),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    UriError(http::uri::InvalidUri),
}

#[derive(Debug)]
pub struct ApiError {
    pub code: hyper::StatusCode,
    pub body: hyper::body::Body,
}

impl From<(hyper::StatusCode, hyper::body::Body)> for Error {
    fn from(e: (hyper::StatusCode, hyper::body::Body)) -> Self {
        Error::Api(ApiError {
            code: e.0,
            body: e.1,
        })
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        return Error::Http(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod auth_info_api;
pub use self::auth_info_api::{ AuthInfoApi, AuthInfoApiClient };
mod basic_info_api;
pub use self::basic_info_api::{ BasicInfoApi, BasicInfoApiClient };
mod credential_api;
pub use self::credential_api::{ CredentialApi, CredentialApiClient };
mod env_api;
pub use self::env_api::{ EnvApi, EnvApiClient };
mod error_api;
pub use self::error_api::{ ErrorApi, ErrorApiClient };
mod invitation_api;
pub use self::invitation_api::{ InvitationApi, InvitationApiClient };
mod role_api;
pub use self::role_api::{ RoleApi, RoleApiClient };
mod saas_user_api;
pub use self::saas_user_api::{ SaasUserApi, SaasUserApiClient };
mod saasus_tenant_api;
pub use self::saasus_tenant_api::{ SaasusTenantApi, SaasusTenantApiClient };
mod tenant_api;
pub use self::tenant_api::{ TenantApi, TenantApiClient };
mod tenant_attribute_api;
pub use self::tenant_attribute_api::{ TenantAttributeApi, TenantAttributeApiClient };
mod tenant_user_api;
pub use self::tenant_user_api::{ TenantUserApi, TenantUserApiClient };
mod user_attribute_api;
pub use self::user_attribute_api::{ UserAttributeApi, UserAttributeApiClient };
mod user_info_api;
pub use self::user_info_api::{ UserInfoApi, UserInfoApiClient };

pub mod configuration;
pub mod client;
