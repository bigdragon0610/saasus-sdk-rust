/*
 * SaaSus Billing API Schema
 *
 * SaaSus Billing API Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStripeInfoParam {
    /// secret key
    #[serde(rename = "secret_key")]
    pub secret_key: String,
}

impl UpdateStripeInfoParam {
    pub fn new(secret_key: String) -> UpdateStripeInfoParam {
        UpdateStripeInfoParam {
            secret_key,
        }
    }
}


