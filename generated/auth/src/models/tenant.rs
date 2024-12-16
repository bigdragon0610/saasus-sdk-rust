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
pub struct Tenant {
    /// tenant name
    #[serde(rename = "name")]
    pub name: String,
    /// attribute info
    #[serde(rename = "attributes")]
    pub attributes: ::std::collections::HashMap<String, serde_json::Value>,
    /// administrative staff email address
    #[serde(rename = "back_office_staff_email")]
    pub back_office_staff_email: String,
    #[serde(rename = "next_plan_id", skip_serializing_if = "Option::is_none")]
    pub next_plan_id: Option<String>,
    /// Next billing plan start time (When using stripe, you can create a subscription that starts at the beginning of the current month by specifying 00:00 (UTC) at the beginning of the current month. Ex. 1672531200 for January 2023.) 
    #[serde(rename = "using_next_plan_from", skip_serializing_if = "Option::is_none")]
    pub using_next_plan_from: Option<i32>,
    #[serde(rename = "next_plan_tax_rate_id", skip_serializing_if = "Option::is_none")]
    pub next_plan_tax_rate_id: Option<String>,
    #[serde(rename = "proration_behavior", skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<crate::models::ProrationBehavior>,
    /// If you have a stripe linkage,  you can set whether to delete pay-as-you-go items when changing plans. When you change plan, you can remove all pay-as-you-go items included in your current subscription to stop being billed based on pay-as-you-go items. The recorded usage is cleared immediately. Since it cannot be restored, please note that plan change reservations with delete_usage set to true cannot be canceled. 
    #[serde(rename = "delete_usage", skip_serializing_if = "Option::is_none")]
    pub delete_usage: Option<bool>,
    /// Plan History
    #[serde(rename = "plan_histories")]
    pub plan_histories: Vec<crate::models::PlanHistory>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "plan_id", skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "billing_info", skip_serializing_if = "Option::is_none")]
    pub billing_info: Option<Box<crate::models::BillingInfo>>,
}

impl Tenant {
    pub fn new(name: String, attributes: ::std::collections::HashMap<String, serde_json::Value>, back_office_staff_email: String, plan_histories: Vec<crate::models::PlanHistory>, id: String) -> Tenant {
        Tenant {
            name,
            attributes,
            back_office_staff_email,
            next_plan_id: None,
            using_next_plan_from: None,
            next_plan_tax_rate_id: None,
            proration_behavior: None,
            delete_usage: None,
            plan_histories,
            id,
            plan_id: None,
            billing_info: None,
        }
    }
}


