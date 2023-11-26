/*
 * SaaSus AWS Marketplace API Schema
 *
 * SaaSus AWS Marketplace API Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ListingStatus {
    #[serde(rename = "no_listing")]
    NoListing,
    #[serde(rename = "first_step_working")]
    FirstStepWorking,
    #[serde(rename = "first_step_completed")]
    FirstStepCompleted,
    #[serde(rename = "second_step_working")]
    SecondStepWorking,
    #[serde(rename = "second_step_plan_created")]
    SecondStepPlanCreated,
    #[serde(rename = "second_step_completed")]
    SecondStepCompleted,
    #[serde(rename = "third_step_working")]
    ThirdStepWorking,
    #[serde(rename = "third_step_completed")]
    ThirdStepCompleted,
    #[serde(rename = "limited")]
    Limited,
    #[serde(rename = "restricted")]
    Restricted,
    #[serde(rename = "public")]
    Public,

}

impl ToString for ListingStatus {
    fn to_string(&self) -> String {
        match self {
            Self::NoListing => String::from("no_listing"),
            Self::FirstStepWorking => String::from("first_step_working"),
            Self::FirstStepCompleted => String::from("first_step_completed"),
            Self::SecondStepWorking => String::from("second_step_working"),
            Self::SecondStepPlanCreated => String::from("second_step_plan_created"),
            Self::SecondStepCompleted => String::from("second_step_completed"),
            Self::ThirdStepWorking => String::from("third_step_working"),
            Self::ThirdStepCompleted => String::from("third_step_completed"),
            Self::Limited => String::from("limited"),
            Self::Restricted => String::from("restricted"),
            Self::Public => String::from("public"),
        }
    }
}

impl Default for ListingStatus {
    fn default() -> ListingStatus {
        Self::NoListing
    }
}




