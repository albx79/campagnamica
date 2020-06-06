use wasm_bindgen::__rt::std::error::Error;
use csv::{ReaderBuilder};
use derive_builder::Builder;
use anyhow::{Context, Result};
use wasm_bindgen::__rt::core::fmt::{Display, Formatter};

pub fn parse_csv(data: String) -> Result<InputData> {
    let reader = ReaderBuilder::new().from_reader(data.as_bytes());
    let mut rdr = csv::Reader::from(reader);
    let mut data = Vec::new();
    for result in rdr.records() {
        let ctx = format!("{:?}", &result);
        let record = result.context(ctx)?;

        data.push(WooCommerceRow {
            order_id: record[0].parse()?,
            order_date: record[1].to_owned(),
            order_status: record[2].to_owned(),
            customer_name: record[3].to_owned(),
            order_total: record[4].to_owned(),
            order_shipping: record[5].parse().with_context(|| format!("Invalid shipping: {}", &record[5]))?,
            payment_gateway: record[6].to_owned(),
            shipping_method: record[7].to_owned(),
            shipping_address_line_1: record[8].to_owned(),
            shipping_address_line_2: record[9].to_owned(),
            shipping_postcode: record[10].to_owned(),
            billing_phone_number: record[11].to_owned(),
            _transaction_id: record[12].to_owned(),
            product_name: record[13].to_owned(),
            quantity: record[14].parse().with_context(|| format!("Invalid quantity: {}", &record[14]))?,
            item_price: record[15].to_owned(),
        });
    }
    Ok(InputData { data })
}

#[derive(Builder, Clone, Debug)]
pub struct WooCommerceRow {
    pub order_id: u32,
    pub order_date: String,
    pub order_status: String,
    pub customer_name: String,
    pub order_total: String,
    pub order_shipping: f32,
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

#[derive(Debug)]
pub struct InputData {
    pub data: Vec<WooCommerceRow>
}

#[derive(Clone, Builder)]
pub struct OrderDetails {
    pub order_id: u32,
    pub customer_name: String,
    pub order_total: String,
    pub order_shipping: f32,
    pub shipping_method: String,
    pub payment_gateway: String,
    pub shipping_address_line_1: String,
    pub shipping_address_line_2: String,
    pub shipping_postcode: String,
    pub order_date: String,
    pub billing_phone_number: String,
    pub products: Vec<OrderItem>,
}

#[derive(Clone, Builder)]
pub struct OrderItem {
    pub product_name: String,
    pub quantity: u32,
    pub item_price: f32,
}

impl InputData {
    pub fn labels(&self) -> Result<Vec<OrderDetails>> {
        use itertools::Itertools;

        let mut result = Vec::new();
        for (order_id, rows) in &self.data.iter().group_by(|row| row.order_id) {
            let rows = rows.collect::<Vec<&WooCommerceRow>>();
            let row = rows[0];
            let mut order_details = OrderDetails {
                order_id,
                customer_name: row.customer_name.clone(),
                shipping_address_line_1: row.shipping_address_line_1.clone(),
                shipping_address_line_2: row.shipping_address_line_2.clone(),
                shipping_postcode: row.shipping_postcode.clone(),
                billing_phone_number: row.billing_phone_number.clone(),
                payment_gateway: row.payment_gateway.clone(),
                order_date: row.order_date.clone(),
                order_total: row.order_total.clone(),
                order_shipping: row.order_shipping,
                shipping_method: row.shipping_method.clone(),
                products: Vec::new(),
            };
            for o in rows {
                order_details.products.push(OrderItem {
                    quantity: o.quantity,
                    product_name: o.product_name.clone(),
                    item_price: o.item_price.parse().with_context(|| format!("Invalid price: {}", o.item_price))?
                })
            }

            result.push(order_details);
        }

        Ok(result)
    }
}

#[derive(Debug)]
pub struct PriceParseError(String);
impl Display for PriceParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl Error for PriceParseError {}

#[cfg(test)]
const DATA: &str = r###""Order ID","Order Date","Order Status","Customer Name","Order Total","Order Shipping","Payment Gateway","Shipping Method","Shipping Address Line 1","Shipping Address Line 2","Shipping Zip/Postcode","Billing Phone Number",_transaction_id,"Product Name","Quantity of items purchased","Item price EXCL. tax"
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

#[test]
fn test_parse_csv() {
    let parsed = parse_csv(DATA.to_owned()).unwrap();
    let data = &parsed.data;
    assert_eq!(data.len(), 9);
    assert_eq!(data[0].order_id, 5358);
    assert_eq!(data[8].item_price, "3.5");

    let labels = parsed.labels().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].order_id, 5358);
    assert_eq!(labels[0].products.len(), 4);
    assert_eq!(labels[0].products[0].product_name, r#"SELEZIONE B "IL VEGETARIANO""#);
    assert_eq!(labels[0].products[0].item_price, 40.0);
    assert_eq!(labels[0].products[0].quantity, 1);
    assert_eq!(labels[0].products[3].product_name, r#"GALLETTO VALLE SPLUGA ALLE ERBE DI MONTAGNA 500 g"#);
    assert_eq!(labels[0].products[3].item_price, 4.6);
    assert_eq!(labels[0].products[3].quantity, 1);

    assert_eq!(labels[1].order_id, 5357);
    assert_eq!(labels[1].products.len(), 5);
}

#[test]
fn test_parse_empty_data() {
    let parsed = parse_csv("".to_owned()).unwrap();
    assert_eq!(parsed.data.len(), 0);
}

