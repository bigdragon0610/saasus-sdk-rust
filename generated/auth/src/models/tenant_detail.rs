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
pub struct TenantDetail {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "plan_id", skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "billing_info", skip_serializing_if = "Option::is_none")]
    pub billing_info: Option<Box<crate::models::BillingInfo>>,
    /// テナント名(tenant name)
    #[serde(rename = "name")]
    pub name: String,
    /// 属性情報(attribute info)
    #[serde(rename = "attributes")]
    pub attributes: ::std::collections::HashMap<String, serde_json::Value>,
    /// 事務管理部門スタッフメールアドレス(administrative staff email address)
    #[serde(rename = "back_office_staff_email")]
    pub back_office_staff_email: String,
    #[serde(rename = "next_plan_id", skip_serializing_if = "Option::is_none")]
    pub next_plan_id: Option<String>,
    /// 次回料金プラン開始日時（stripe連携時、当月月初の0時（UTC）を指定すると当月月初開始のサブスクリプションを作成できます。ex. 2023年1月の場合は、1672531200 ） (Next billing plan start time (When using stripe, you can create a subscription that starts at the beginning of the current month by specifying 00:00 (UTC) at the beginning of the current month. Ex. 1672531200 for January 2023.)) 
    #[serde(rename = "using_next_plan_from", skip_serializing_if = "Option::is_none")]
    pub using_next_plan_from: Option<i32>,
    #[serde(rename = "next_plan_tax_rate_id", skip_serializing_if = "Option::is_none")]
    pub next_plan_tax_rate_id: Option<String>,
    #[serde(rename = "proration_behavior", skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<crate::models::ProrationBehavior>,
    /// stripe連携している場合で、プラン変更時に従量課金アイテムを削除するか設定できます。 プラン変更した場合に、現在のサブスクリプションに含まれる従量課金アイテムを全て削除して、従量課金アイテムに基づく請求の発生を止めることができます。 即時に記録している使用量がクリアされます。それらは復元できないため、delete_usageをtrueにしたプラン変更予約は取り消しできません。  If you have a stripe linkage,  you can set whether to delete pay-as-you-go items when changing plans. When you change plan, you can remove all pay-as-you-go items included in your current subscription to stop being billed based on pay-as-you-go items. The recorded usage is cleared immediately. Since it cannot be restored, please note that plan change reservations with delete_usage set to true cannot be canceled. 
    #[serde(rename = "delete_usage", skip_serializing_if = "Option::is_none")]
    pub delete_usage: Option<bool>,
    /// 料金プラン履歴
    #[serde(rename = "plan_histories")]
    pub plan_histories: Vec<crate::models::PlanHistory>,
    /// 現在のプランの開始日時(current plan period start)
    #[serde(rename = "current_plan_period_start", skip_serializing_if = "Option::is_none")]
    pub current_plan_period_start: Option<i32>,
    /// 現在のプランの終了日時(current plan period end)
    #[serde(rename = "current_plan_period_end", skip_serializing_if = "Option::is_none")]
    pub current_plan_period_end: Option<i32>,
}

impl TenantDetail {
    pub fn new(id: String, name: String, attributes: ::std::collections::HashMap<String, serde_json::Value>, back_office_staff_email: String, plan_histories: Vec<crate::models::PlanHistory>) -> TenantDetail {
        TenantDetail {
            id,
            plan_id: None,
            billing_info: None,
            name,
            attributes,
            back_office_staff_email,
            next_plan_id: None,
            using_next_plan_from: None,
            next_plan_tax_rate_id: None,
            proration_behavior: None,
            delete_usage: None,
            plan_histories,
            current_plan_period_start: None,
            current_plan_period_end: None,
        }
    }
}


