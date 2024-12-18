/*
 * SaaSus Pricing API Schema
 *
 * SaaSus Pricing API Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingUsageUnit {
    /// Upper limit
    #[serde(rename = "upper_count")]
    pub upper_count: i32,
    /// Amount per usage
    #[serde(rename = "unit_amount")]
    pub unit_amount: i32,
    /// Metering unit name
    #[serde(rename = "metering_unit_name")]
    pub metering_unit_name: String,
    #[serde(rename = "aggregate_usage", skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<crate::models::AggregateUsage>,
    /// Name
    #[serde(rename = "name")]
    pub name: String,
    /// Display Name
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Description
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::UnitType,
    #[serde(rename = "currency")]
    pub currency: crate::models::Currency,
    /// Universally Unique Identifier
    #[serde(rename = "id")]
    pub id: String,
    /// Universally Unique Identifier
    #[serde(rename = "metering_unit_id")]
    pub metering_unit_id: String,
    #[serde(rename = "recurring_interval")]
    pub recurring_interval: crate::models::RecurringInterval,
    #[serde(rename = "used")]
    pub used: bool,
}

impl PricingUsageUnit {
    pub fn new(upper_count: i32, unit_amount: i32, metering_unit_name: String, name: String, display_name: String, description: String, r#type: crate::models::UnitType, currency: crate::models::Currency, id: String, metering_unit_id: String, recurring_interval: crate::models::RecurringInterval, used: bool) -> PricingUsageUnit {
        PricingUsageUnit {
            upper_count,
            unit_amount,
            metering_unit_name,
            aggregate_usage: None,
            name,
            display_name,
            description,
            r#type,
            currency,
            id,
            metering_unit_id,
            recurring_interval,
            used,
        }
    }
}


