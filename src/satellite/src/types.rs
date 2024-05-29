use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CandidType)]
pub enum CurrencyType {
  #[serde(rename = "cad")]
  CAD,
  #[serde(rename = "chf")]
  CHF,
  #[serde(rename = "eur")]
  EUR,
  #[serde(rename = "gbp")]
  GBP,
  #[serde(rename = "idr")]
  IDR,
  #[serde(rename = "jpy")]
  JPY,
  #[serde(rename = "usd")]
  USD,
}

#[derive(Deserialize, Serialize)]
pub struct Note {
    pub name: String,
    pub price_cents: i32,
    pub currency: String
  }

#[derive(Deserialize, Serialize)]
 pub struct UpdateNote {
    pub name: String,
    pub price_cents: i32,
    pub currency: String,
    pub price_usd_cents: String
  }
  
#[derive(Deserialize, Serialize, CandidType)]
pub struct CurrencyData{
  pub value: f64,
  pub source: Option<String>,
  pub name: String,
  pub currency_type: CurrencyType,
  pub description: Option<String>,
  pub created_at: usize,
  pub value_str: String,
  pub symbol: String
}
