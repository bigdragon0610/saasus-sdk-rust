use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    auth_info_api: Box<dyn crate::apis::AuthInfoApi>,
    basic_info_api: Box<dyn crate::apis::BasicInfoApi>,
    credential_api: Box<dyn crate::apis::CredentialApi>,
    env_api: Box<dyn crate::apis::EnvApi>,
    error_api: Box<dyn crate::apis::ErrorApi>,
    invitation_api: Box<dyn crate::apis::InvitationApi>,
    role_api: Box<dyn crate::apis::RoleApi>,
    saas_user_api: Box<dyn crate::apis::SaasUserApi>,
    tenant_api: Box<dyn crate::apis::TenantApi>,
    tenant_attribute_api: Box<dyn crate::apis::TenantAttributeApi>,
    tenant_user_api: Box<dyn crate::apis::TenantUserApi>,
    user_attribute_api: Box<dyn crate::apis::UserAttributeApi>,
    user_info_api: Box<dyn crate::apis::UserInfoApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Rc::new(configuration);

        APIClient {
            auth_info_api: Box::new(crate::apis::AuthInfoApiClient::new(rc.clone())),
            basic_info_api: Box::new(crate::apis::BasicInfoApiClient::new(rc.clone())),
            credential_api: Box::new(crate::apis::CredentialApiClient::new(rc.clone())),
            env_api: Box::new(crate::apis::EnvApiClient::new(rc.clone())),
            error_api: Box::new(crate::apis::ErrorApiClient::new(rc.clone())),
            invitation_api: Box::new(crate::apis::InvitationApiClient::new(rc.clone())),
            role_api: Box::new(crate::apis::RoleApiClient::new(rc.clone())),
            saas_user_api: Box::new(crate::apis::SaasUserApiClient::new(rc.clone())),
            tenant_api: Box::new(crate::apis::TenantApiClient::new(rc.clone())),
            tenant_attribute_api: Box::new(crate::apis::TenantAttributeApiClient::new(rc.clone())),
            tenant_user_api: Box::new(crate::apis::TenantUserApiClient::new(rc.clone())),
            user_attribute_api: Box::new(crate::apis::UserAttributeApiClient::new(rc.clone())),
            user_info_api: Box::new(crate::apis::UserInfoApiClient::new(rc.clone())),
        }
    }

    pub fn auth_info_api(&self) -> &dyn crate::apis::AuthInfoApi{
        self.auth_info_api.as_ref()
    }

    pub fn basic_info_api(&self) -> &dyn crate::apis::BasicInfoApi{
        self.basic_info_api.as_ref()
    }

    pub fn credential_api(&self) -> &dyn crate::apis::CredentialApi{
        self.credential_api.as_ref()
    }

    pub fn env_api(&self) -> &dyn crate::apis::EnvApi{
        self.env_api.as_ref()
    }

    pub fn error_api(&self) -> &dyn crate::apis::ErrorApi{
        self.error_api.as_ref()
    }

    pub fn invitation_api(&self) -> &dyn crate::apis::InvitationApi{
        self.invitation_api.as_ref()
    }

    pub fn role_api(&self) -> &dyn crate::apis::RoleApi{
        self.role_api.as_ref()
    }

    pub fn saas_user_api(&self) -> &dyn crate::apis::SaasUserApi{
        self.saas_user_api.as_ref()
    }

    pub fn tenant_api(&self) -> &dyn crate::apis::TenantApi{
        self.tenant_api.as_ref()
    }

    pub fn tenant_attribute_api(&self) -> &dyn crate::apis::TenantAttributeApi{
        self.tenant_attribute_api.as_ref()
    }

    pub fn tenant_user_api(&self) -> &dyn crate::apis::TenantUserApi{
        self.tenant_user_api.as_ref()
    }

    pub fn user_attribute_api(&self) -> &dyn crate::apis::UserAttributeApi{
        self.user_attribute_api.as_ref()
    }

    pub fn user_info_api(&self) -> &dyn crate::apis::UserInfoApi{
        self.user_info_api.as_ref()
    }

}
