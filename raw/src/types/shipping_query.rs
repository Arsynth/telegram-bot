use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress
}