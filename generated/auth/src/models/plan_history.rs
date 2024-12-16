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
pub struct PlanHistory {
    #[serde(rename = "plan_id")]
    pub plan_id: String,
    /// Registration date
    #[serde(rename = "plan_applied_at")]
    pub plan_applied_at: i32,
    #[serde(rename = "tax_rate_id", skip_serializing_if = "Option::is_none")]
    pub tax_rate_id: Option<String>,
    #[serde(rename = "proration_behavior", skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<crate::models::ProrationBehavior>,
    /// If you have a stripe linkage,  you can set whether to delete pay-as-you-go items when changing plans. When you change plan, you can remove all pay-as-you-go items included in your current subscription to stop being billed based on pay-as-you-go items. The recorded usage is cleared immediately. Since it cannot be restored, please note that plan change reservations with delete_usage set to true cannot be canceled. 
    #[serde(rename = "delete_usage", skip_serializing_if = "Option::is_none")]
    pub delete_usage: Option<bool>,
}

impl PlanHistory {
    pub fn new(plan_id: String, plan_applied_at: i32) -> PlanHistory {
        PlanHistory {
            plan_id,
            plan_applied_at,
            tax_rate_id: None,
            proration_behavior: None,
            delete_usage: None,
        }
    }
}


