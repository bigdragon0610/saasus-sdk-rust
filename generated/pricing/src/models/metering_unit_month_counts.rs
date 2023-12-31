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
pub struct MeteringUnitMonthCounts {
    #[serde(rename = "counts")]
    pub counts: Vec<crate::models::MeteringUnitMonthCount>,
}

impl MeteringUnitMonthCounts {
    pub fn new(counts: Vec<crate::models::MeteringUnitMonthCount>) -> MeteringUnitMonthCounts {
        MeteringUnitMonthCounts {
            counts,
        }
    }
}


