/*
 * SaaSus Auth API Schema
 *
 * Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudFormationLaunchStackLink {
    #[serde(rename = "link")]
    pub link: String,
}

impl CloudFormationLaunchStackLink {
    pub fn new(link: String) -> CloudFormationLaunchStackLink {
        CloudFormationLaunchStackLink {
            link,
        }
    }
}


