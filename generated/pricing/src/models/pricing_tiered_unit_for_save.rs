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
pub struct PricingTieredUnitForSave {
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
    #[serde(rename = "tiers")]
    pub tiers: Vec<crate::models::PricingTier>,
    /// Upper limit
    #[serde(rename = "upper_count")]
    pub upper_count: i32,
    /// Metering unit name
    #[serde(rename = "metering_unit_name")]
    pub metering_unit_name: String,
    #[serde(rename = "aggregate_usage", skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<crate::models::AggregateUsage>,
}

impl PricingTieredUnitForSave {
    pub fn new(name: String, display_name: String, description: String, r#type: crate::models::UnitType, currency: crate::models::Currency, tiers: Vec<crate::models::PricingTier>, upper_count: i32, metering_unit_name: String) -> PricingTieredUnitForSave {
        PricingTieredUnitForSave {
            name,
            display_name,
            description,
            r#type,
            currency,
            tiers,
            upper_count,
            metering_unit_name,
            aggregate_usage: None,
        }
    }
}


