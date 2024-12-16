/*
 * SaaSus Auth API Schema
 *
 * Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignUpWithAwsMarketplaceParam {
    /// Email Address
    #[serde(rename = "email")]
    pub email: String,
    /// Registration Token
    #[serde(rename = "registration_token")]
    pub registration_token: String,
}

impl SignUpWithAwsMarketplaceParam {
    pub fn new(email: String, registration_token: String) -> SignUpWithAwsMarketplaceParam {
        SignUpWithAwsMarketplaceParam {
            email,
            registration_token,
        }
    }
}


