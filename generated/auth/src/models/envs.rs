/*
 * SaaSus Auth API Schema
 *
 * Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Envs : env list



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Envs {
    #[serde(rename = "envs")]
    pub envs: Vec<crate::models::Env>,
}

impl Envs {
    /// env list
    pub fn new(envs: Vec<crate::models::Env>) -> Envs {
        Envs {
            envs,
        }
    }
}


