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
pub struct SavePricingPlanParam {
    /// Pricing plan name
    #[serde(rename = "name")]
    pub name: String,
    /// Pricing plan display name
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Pricing plan description
    #[serde(rename = "description")]
    pub description: String,
    /// Menu ID to be added to the pricing plan
    #[serde(rename = "menu_ids")]
    pub menu_ids: Vec<String>,
}

impl SavePricingPlanParam {
    pub fn new(name: String, display_name: String, description: String, menu_ids: Vec<String>) -> SavePricingPlanParam {
        SavePricingPlanParam {
            name,
            display_name,
            description,
            menu_ids,
        }
    }
}


