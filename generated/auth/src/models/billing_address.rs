/*
 * SaaSus Auth API Schema
 *
 * スキーマ
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingAddress {
    /// 住所の通りの名前や番地を含めた部分  Street address, apartment or suite number. 
    #[serde(rename = "street")]
    pub street: String,
    /// 住所の市区町村  City, district, suburb, town, or village. 
    #[serde(rename = "city")]
    pub city: String,
    /// 住所の都道府県または州  State name or abbreviation. 
    #[serde(rename = "state")]
    pub state: String,
    /// 住所の国を ISO 3166-1 alpha-2 コードで指定します。  Country of the address using ISO 3166-1 alpha-2 code. 
    #[serde(rename = "country")]
    pub country: String,
    /// 建物名・部屋番号などの住所に関する追加情報  Additional information about the address, such as a building name, floor, or department name. 
    #[serde(rename = "additional_address_info", skip_serializing_if = "Option::is_none")]
    pub additional_address_info: Option<String>,
    /// 郵便番号  ZIP or postal code. 
    #[serde(rename = "postal_code")]
    pub postal_code: String,
}

impl BillingAddress {
    pub fn new(street: String, city: String, state: String, country: String, postal_code: String) -> BillingAddress {
        BillingAddress {
            street,
            city,
            state,
            country,
            additional_address_info: None,
            postal_code,
        }
    }
}


