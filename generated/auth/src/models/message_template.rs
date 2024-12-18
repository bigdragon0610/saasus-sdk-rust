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
pub struct MessageTemplate {
    /// Title
    #[serde(rename = "subject")]
    pub subject: String,
    /// Message
    #[serde(rename = "message")]
    pub message: String,
}

impl MessageTemplate {
    pub fn new(subject: String, message: String) -> MessageTemplate {
        MessageTemplate {
            subject,
            message,
        }
    }
}


