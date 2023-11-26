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
pub struct Comments {
    #[serde(rename = "comments")]
    pub comments: Vec<crate::models::Comment>,
}

impl Comments {
    pub fn new(comments: Vec<crate::models::Comment>) -> Comments {
        Comments {
            comments,
        }
    }
}


