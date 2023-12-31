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
pub struct SavePricingMenuParam {
    /// メニュー名(menu name)
    #[serde(rename = "name")]
    pub name: String,
    /// メニュー表示名(menu display name)
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// メニュー説明(menu description)
    #[serde(rename = "description")]
    pub description: String,
    /// 追加するユニットID(unit id to add)
    #[serde(rename = "unit_ids")]
    pub unit_ids: Vec<String>,
}

impl SavePricingMenuParam {
    pub fn new(name: String, display_name: String, description: String, unit_ids: Vec<String>) -> SavePricingMenuParam {
        SavePricingMenuParam {
            name,
            display_name,
            description,
            unit_ids,
        }
    }
}


