/*
 * SaaSus Auth API Schema
 *
 * スキーマ
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    /// 属性名(attribute name)
    #[serde(rename = "attribute_name")]
    pub attribute_name: String,
    /// 表示名(display name)
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "attribute_type")]
    pub attribute_type: crate::models::AttributeType,
}

impl Attribute {
    pub fn new(attribute_name: String, display_name: String, attribute_type: crate::models::AttributeType) -> Attribute {
        Attribute {
            attribute_name,
            display_name,
            attribute_type,
        }
    }
}


