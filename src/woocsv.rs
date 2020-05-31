use crate::app::{InputData};
use wasm_bindgen::__rt::std::error::Error;
use csv::{ReaderBuilder, Trim};
use derive_builder::Builder;
use yew::prelude::*;
use wasm_bindgen::__rt::core::ptr::read_volatile;

pub fn parse_csv(mut data: String) -> Result<InputData, Box<dyn Error>> {
    let reader = ReaderBuilder::new()
        // .double_quote(true)
        .from_reader(data.as_bytes());
    let mut rdr = csv::Reader::from(reader);
    let mut input_data: Vec<WooCommerceRow> = Vec::new();
    for result in rdr.records() {
        let record = result?;

        input_data.push(WooCommerceRow {
            order_id: record[0].parse()?,
            order_date: record[1].to_owned(),
            order_status: record[2].to_owned(),
            customer_name: record[3].to_owned(),
            order_total: record[4].to_owned(),
            order_shipping: record[5].parse()?,
            payment_gateway: record[6].to_owned(),
            shipping_method: record[7].to_owned(),
            shipping_address_line_1: record[8].to_owned(),
            shipping_address_line_2: record[9].to_owned(),
            shipping_postcode: record[10].to_owned(),
            billing_phone_number: record[11].to_owned(),
            _transaction_id: record[12].to_owned(),
            product_name: record[13].to_owned(),
            quantity: record[14].parse()?,
            item_price: record[15].to_owned(),
        });
    }
    Ok(InputData { data: input_data })
}

#[derive(Builder, Clone, Debug)]
pub struct WooCommerceRow {
    pub order_id: u32,
    pub order_date: String,
    pub order_status: String,
    pub customer_name: String,
    pub order_total: String,
    pub order_shipping: u32,
    pub payment_gateway: String,
    pub shipping_method: String,
    pub shipping_address_line_1: String,
    pub shipping_address_line_2: String,
    pub shipping_postcode: String,
    pub billing_phone_number: String,
    pub _transaction_id: String,
    pub product_name: String,
    pub quantity: u32,
    pub item_price: String,
}

#[test]
fn test_parse_csv() {
    let data = r###""Order ID","Order Date","Order Status","Customer Name","Order Total","Order Shipping","Payment Gateway","Shipping Method","Shipping Address Line 1","Shipping Address Line 2","Shipping Zip/Postcode","Billing Phone Number",_transaction_id,"Product Name","Quantity of items purchased","Item price EXCL. tax"
5358,2020/05/24,processing,"PERINO LUPO","57,10",5,"PayPal o carta di credito",flat_rate:1,"VIA DEI PAZZI 0","SCALA A DESTRA SECONDO PIANO",20146,3355700000,0P128552W4082524Y,"SELEZIONE B ""IL VEGETARIANO""",1,40
5358,2020/05/24,processing,"PERINO LUPO","57,10",5,"PayPal o carta di credito",flat_rate:1,"VIA DEI PAZZI 0","SCALA A DESTRA SECONDO PIANO",20146,3355700000,0P128552W4082524Y,"CARNE TRITA DI MANZO PER RAGU' E POLPETTE 500 g",1,3.5
5358,2020/05/24,processing,"PERINO LUPO","57,10",5,"PayPal o carta di credito",flat_rate:1,"VIA DEI PAZZI 0","SCALA A DESTRA SECONDO PIANO",20146,3355700000,0P128552W4082524Y,"FETTINE DI LONZA DI SUINO 500 g",1,4
5358,2020/05/24,processing,"PERINO LUPO","57,10",5,"PayPal o carta di credito",flat_rate:1,"VIA DEI PAZZI 0","SCALA A DESTRA SECONDO PIANO",20146,3355700000,0P128552W4082524Y,"GALLETTO VALLE SPLUGA ALLE ERBE DI MONTAGNA 500 g",1,4.6
5357,2020/05/24,processing,"Maria Luisa","57,90",5,"PayPal o carta di credito",flat_rate:1,"Via Da Qui 1",,20129,3332750000,5L1092726H247623G,"INSALATA VARIA 500 g",1,1.4
5357,2020/05/24,processing,"Maria Luisa","57,90",5,"PayPal o carta di credito",flat_rate:1,"Via Da Qui 1",,20129,3332750000,5L1092726H247623G,"SELEZIONE B ""IL VEGETARIANO""",1,40
5357,2020/05/24,processing,"Maria Luisa","57,90",5,"PayPal o carta di credito",flat_rate:1,"Via Da Qui 1",,20129,3332750000,5L1092726H247623G,"YOGURT DI CAPRA 500 g",1,3
5357,2020/05/24,processing,"Maria Luisa","57,90",5,"PayPal o carta di credito",flat_rate:1,"Via Da Qui 1",,20129,3332750000,5L1092726H247623G,"10 ARROSTICINI DI SUINO 300 g",1,5
5357,2020/05/24,processing,"Maria Luisa","57,90",5,"PayPal o carta di credito",flat_rate:1,"Via Da Qui 1",,20129,3332750000,5L1092726H247623G,"PANE AI CEREALI ANTICHI 500 g",1,3.5
"###;
    let parsed = parse_csv(data.to_owned()).unwrap().data;
    assert_eq!(parsed.len(), 9);
    assert_eq!(parsed[0].order_id, 5358);
    assert_eq!(parsed[8].item_price, "3.5");
}

#[test]
fn test_parse_empty_data() {
    let parsed = parse_csv("".to_owned()).unwrap();
    assert_eq!(parsed.data.len(), 0);
}