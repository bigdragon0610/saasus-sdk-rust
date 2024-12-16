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
pub struct PricingFixedUnitForSave {
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
    /// Price
    #[serde(rename = "unit_amount")]
    pub unit_amount: i32,
    #[serde(rename = "recurring_interval")]
    pub recurring_interval: crate::models::RecurringInterval,
}

impl PricingFixedUnitForSave {
    pub fn new(name: String, display_name: String, description: String, r#type: crate::models::UnitType, currency: crate::models::Currency, unit_amount: i32, recurring_interval: crate::models::RecurringInterval) -> PricingFixedUnitForSave {
        PricingFixedUnitForSave {
            name,
            display_name,
            description,
            r#type,
            currency,
            unit_amount,
            recurring_interval,
        }
    }
}


