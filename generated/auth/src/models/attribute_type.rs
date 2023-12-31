/*
 * SaaSus Auth API Schema
 *
 * スキーマ
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AttributeType : 型（dateはYYYY-MM-DDの形式で使用する事ができます。） (Type (date can be set to YYYY-MM-DD format.)) 

/// 型（dateはYYYY-MM-DDの形式で使用する事ができます。） (Type (date can be set to YYYY-MM-DD format.)) 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttributeType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "date")]
    Date,

}

impl ToString for AttributeType {
    fn to_string(&self) -> String {
        match self {
            Self::String => String::from("string"),
            Self::Number => String::from("number"),
            Self::Bool => String::from("bool"),
            Self::Date => String::from("date"),
        }
    }
}

impl Default for AttributeType {
    fn default() -> AttributeType {
        Self::String
    }
}




