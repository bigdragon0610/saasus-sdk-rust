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
pub struct StripeCustomer {
    /// stripe Customer ID
    #[serde(rename = "customer_id")]
    pub customer_id: String,
    /// stripe Subscription Schedule ID
    #[serde(rename = "subscription_schedule_id")]
    pub subscription_schedule_id: String,
}

impl StripeCustomer {
    pub fn new(customer_id: String, subscription_schedule_id: String) -> StripeCustomer {
        StripeCustomer {
            customer_id,
            subscription_schedule_id,
        }
    }
}

