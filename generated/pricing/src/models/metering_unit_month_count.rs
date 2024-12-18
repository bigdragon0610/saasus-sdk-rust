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
pub struct MeteringUnitMonthCount {
    /// Metering unit name
    #[serde(rename = "metering_unit_name")]
    pub metering_unit_name: String,
    /// Month
    #[serde(rename = "month")]
    pub month: String,
    /// Count
    #[serde(rename = "count")]
    pub count: i32,
}

impl MeteringUnitMonthCount {
    pub fn new(metering_unit_name: String, month: String, count: i32) -> MeteringUnitMonthCount {
        MeteringUnitMonthCount {
            metering_unit_name,
            month,
            count,
        }
    }
}


