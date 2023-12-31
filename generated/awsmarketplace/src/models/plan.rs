/*
 * SaaSus AWS Marketplace API Schema
 *
 * SaaSus AWS Marketplace API Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[serde(rename = "plan_id")]
    pub plan_id: String,
    #[serde(rename = "plan_name")]
    pub plan_name: String,
}

impl Plan {
    pub fn new(plan_id: String, plan_name: String) -> Plan {
        Plan {
            plan_id,
            plan_name,
        }
    }
}


