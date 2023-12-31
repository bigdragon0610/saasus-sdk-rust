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
    /// 上限値(upper limit)
    #[serde(rename = "upper_count")]
    pub upper_count: i32,
    /// 使用量あたりの金額(amount per usage)
    #[serde(rename = "unit_amount")]
    pub unit_amount: i32,
    #[serde(rename = "metering_unit_name")]
    pub metering_unit_name: String,
    #[serde(rename = "aggregate_usage", skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<crate::models::AggregateUsage>,
    /// 名前(name)
    #[serde(rename = "name")]
    pub name: String,
    /// 表示名(display name)
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// 説明(description)
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::UnitType,
    #[serde(rename = "currency")]
    pub currency: crate::models::Currency,
    #[serde(rename = "id")]
    pub id: String,
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


