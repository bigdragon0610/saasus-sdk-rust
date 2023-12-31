/*
 * SaaSus Communication API Schema
 *
 * SaaSus Communication API Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFeedbackStatusParam {
    #[serde(rename = "status")]
    pub status: i32,
}

impl UpdateFeedbackStatusParam {
    pub fn new(status: i32) -> UpdateFeedbackStatusParam {
        UpdateFeedbackStatusParam {
            status,
        }
    }
}


