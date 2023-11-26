use http;
use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Header(hyper::http::header::InvalidHeaderValue),
    Http(http::Error),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    UriError(http::uri::InvalidUri),
}

#[derive(Debug)]
pub struct ApiError {
    pub code: hyper::StatusCode,
    pub body: hyper::body::Body,
}

impl From<(hyper::StatusCode, hyper::body::Body)> for Error {
    fn from(e: (hyper::StatusCode, hyper::body::Body)) -> Self {
        Error::Api(ApiError {
            code: e.0,
            body: e.1,
        })
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        return Error::Http(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod error_api;
pub use self::error_api::{ ErrorApi, ErrorApiClient };
mod metering_api;
pub use self::metering_api::{ MeteringApi, MeteringApiClient };
mod pricing_menus_api;
pub use self::pricing_menus_api::{ PricingMenusApi, PricingMenusApiClient };
mod pricing_plans_api;
pub use self::pricing_plans_api::{ PricingPlansApi, PricingPlansApiClient };
mod pricing_units_api;
pub use self::pricing_units_api::{ PricingUnitsApi, PricingUnitsApiClient };
mod tax_rate_api;
pub use self::tax_rate_api::{ TaxRateApi, TaxRateApiClient };

pub mod configuration;
pub mod client;
