/*
 * SaaSus Pricing API Schema
 *
 * SaaSus Pricing API Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateMeteringUnitTimestampCountMethod : 更新方法(update method) add: 加算(addition) sub: 減算(subtraction) direct: 上書き(overwrite) 

/// 更新方法(update method) add: 加算(addition) sub: 減算(subtraction) direct: 上書き(overwrite) 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UpdateMeteringUnitTimestampCountMethod {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "sub")]
    Sub,
    #[serde(rename = "direct")]
    Direct,

}

impl ToString for UpdateMeteringUnitTimestampCountMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Add => String::from("add"),
            Self::Sub => String::from("sub"),
            Self::Direct => String::from("direct"),
        }
    }
}

impl Default for UpdateMeteringUnitTimestampCountMethod {
    fn default() -> UpdateMeteringUnitTimestampCountMethod {
        Self::Add
    }
}




