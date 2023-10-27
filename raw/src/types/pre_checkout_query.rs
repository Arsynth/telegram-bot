use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). 
    /// For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, 
    /// it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>
}