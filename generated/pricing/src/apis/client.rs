use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    error_api: Box<dyn crate::apis::ErrorApi>,
    metering_api: Box<dyn crate::apis::MeteringApi>,
    pricing_menus_api: Box<dyn crate::apis::PricingMenusApi>,
    pricing_plans_api: Box<dyn crate::apis::PricingPlansApi>,
    pricing_units_api: Box<dyn crate::apis::PricingUnitsApi>,
    tax_rate_api: Box<dyn crate::apis::TaxRateApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Rc::new(configuration);

        APIClient {
            error_api: Box::new(crate::apis::ErrorApiClient::new(rc.clone())),
            metering_api: Box::new(crate::apis::MeteringApiClient::new(rc.clone())),
            pricing_menus_api: Box::new(crate::apis::PricingMenusApiClient::new(rc.clone())),
            pricing_plans_api: Box::new(crate::apis::PricingPlansApiClient::new(rc.clone())),
            pricing_units_api: Box::new(crate::apis::PricingUnitsApiClient::new(rc.clone())),
            tax_rate_api: Box::new(crate::apis::TaxRateApiClient::new(rc.clone())),
        }
    }

    pub fn error_api(&self) -> &dyn crate::apis::ErrorApi{
        self.error_api.as_ref()
    }

    pub fn metering_api(&self) -> &dyn crate::apis::MeteringApi{
        self.metering_api.as_ref()
    }

    pub fn pricing_menus_api(&self) -> &dyn crate::apis::PricingMenusApi{
        self.pricing_menus_api.as_ref()
    }

    pub fn pricing_plans_api(&self) -> &dyn crate::apis::PricingPlansApi{
        self.pricing_plans_api.as_ref()
    }

    pub fn pricing_units_api(&self) -> &dyn crate::apis::PricingUnitsApi{
        self.pricing_units_api.as_ref()
    }

    pub fn tax_rate_api(&self) -> &dyn crate::apis::TaxRateApi{
        self.tax_rate_api.as_ref()
    }

}
