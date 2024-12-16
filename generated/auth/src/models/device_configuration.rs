/*
 * SaaSus Auth API Schema
 *
 * Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeviceConfiguration : Settings for remembering trusted devices



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfiguration {
    /// always: always remember userOptIn: user opt-in no: don't save 
    #[serde(rename = "device_remembering")]
    pub device_remembering: DeviceRememberingEnum,
}

impl DeviceConfiguration {
    /// Settings for remembering trusted devices
    pub fn new(device_remembering: DeviceRememberingEnum) -> DeviceConfiguration {
        DeviceConfiguration {
            device_remembering,
        }
    }
}

/// always: always remember userOptIn: user opt-in no: don't save 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceRememberingEnum {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "userOptIn")]
    UserOptIn,
    #[serde(rename = "no")]
    No,
}

impl Default for DeviceRememberingEnum {
    fn default() -> DeviceRememberingEnum {
        Self::Always
    }
}

