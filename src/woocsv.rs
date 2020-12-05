use wasm_bindgen::__rt::std::error::Error;
use csv::{ReaderBuilder};
use derive_builder::Builder;
use anyhow::{Context, Result};
use wasm_bindgen::__rt::core::fmt::{Display, Formatter};
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen::__rt::core::num::ParseFloatError;

pub fn parse_csv(data: &str) -> Result<InputData> {
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
    pub order_total: Price,
    pub delivery: String,
    pub payment_gateway: String,
    pub shipping_address_line_1: String,
    pub shipping_address_line_2: String,
    pub shipping_postcode: String,
    pub order_date: String,
    pub billing_phone_number: String,
    pub packages: Vec<Vec<OrderItem>>,
}

#[derive(Clone, Builder)]
pub struct DeliveryDetail {
    pub name: &'static str,
    pub data: String,
    pub highlight: bool,
}

impl OrderDetails {
    pub fn delivery_details(&self, i: usize) -> Box<[DeliveryDetail]> {
        let show_package_number = self.packages.len() > 1;
        let show_totals = i + 1 == self.packages.len();
        let mut details = Vec::new();

        if show_totals {
            details.append(&mut vec![
                    DeliveryDetail{ name: "Consegna", data: self.delivery.clone(), highlight: false },
                    DeliveryDetail{ name: "Metodo  Pagamento", data: self.payment_gateway.clone(), highlight: false },
                    DeliveryDetail{ name: "Totale", data: format!("{}€", self.order_total.display), highlight: false },
            ]);
        }

        if show_package_number {
            details.push(DeliveryDetail{ name: "", data: format!("{} COLLO {} DI {}", self.package_name(i), i+1, self.packages.len()), highlight: true});
        }

        details.into_boxed_slice()
    }

    fn package_name(&self, index: usize) -> &str {
        if index == 0 {
            ""
        } else {
            &self.customer_name
        }
    }
}

#[derive(Clone, Builder)]
pub struct OrderItem {
    pub product_name: String,
    pub quantity: u32,
    pub item_price: f32,
}

#[derive(Clone, Builder)]
pub struct SummaryRow {
    pub product_name: String,
    pub total_quantity: u32,
}

impl InputData {
    pub fn labels(&self) -> Result<Vec<OrderDetails>> {
        use itertools::Itertools;

        let mut result = Vec::new();
        for (order_id, rows) in &self.data.iter().group_by(|row| row.order_id) {
            let rows = rows.collect::<Vec<&WooCommerceRow>>();
            let row: &WooCommerceRow = rows[0];
            let mut order_details = OrderDetails {
                order_id,
                customer_name: row.customer_name.clone(),
                shipping_address_line_1: row.shipping_address_line_1.clone(),
                shipping_address_line_2: row.shipping_address_line_2.clone(),
                shipping_postcode: row.shipping_postcode.clone(),
                billing_phone_number: row.billing_phone_number.clone(),
                payment_gateway: row.payment_gateway.clone(),
                order_date: row.order_date.clone(),
                order_total: Price::parse(&row.order_total)?,
                delivery: Self::map_shipping_to_delivery(row.order_shipping),
                packages: Vec::new(),
            };
            let num_packages: i32 = { let val = order_details.order_total.value;
                if val <= 40.0 {
                    1
                } else if val <= 70.0 {
                    2
                } else {
                    3
                }
            };
            let num_rows = rows.len() as i32;
            let (mut items_per_package, remainder) = (num_rows / num_packages, num_rows % num_packages);
            if remainder > 0 {
                items_per_package += 1;
            }

            for p in &rows.into_iter()
                .sorted_by_key(|r| &r.product_name)
                .chunks(items_per_package as usize)
            {
                let mut package_items = Vec::new();
                for o in p.into_iter() {
                    package_items.push(OrderItem {
                        quantity: o.quantity,
                        product_name: o.product_name.clone(),
                        item_price: o.item_price.parse().with_context(|| format!("Invalid price: {}", o.item_price))?
                    })
                }
                order_details.packages.push(package_items);
            }
            result.push(order_details);
        }

        Ok(result)
    }

    pub fn summary(&self) -> Vec<(String, u32)> {
        use itertools::Itertools;

        let mut result: HashMap<String, u32> = HashMap::new();
        for row in &self.data {
            *result.entry(row.product_name.clone()).or_insert(0) += 1;
        }
        result.into_iter()
            .sorted_by(|t1, t2| t1.0.cmp(&t2.0))
            .collect()
    }

    fn map_shipping_to_delivery(order_shipping: f32) -> String {
        if order_shipping == 0.0 {
            "local pick up".to_owned()
        } else {
            format!("{} €", order_shipping)
        }
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

#[derive(Clone)]
pub struct Price {
    pub value: f32,
    pub display: String,
}

impl Price {
    fn parse(s: &str) -> Result<Self, ParseFloatError> {
        use core::str::FromStr;
        Ok(Price {
            display: s.to_string(),
            value: f32::from_str(&s.replace(',', "."))?,
        })
    }
}

#[cfg(test)]
const DATA: &str = include_str!("data.csv");

#[test]
fn test_parse_csv() {
    let parsed = parse_csv(DATA).unwrap();
    let data = &parsed.data;
    assert_eq!(data.len(), 9);
    assert_eq!(data[0].order_id, 5358);
    assert_eq!(data[8].item_price, "3.5");

    let labels = parsed.labels().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].order_id, 5358);
    assert_eq!(labels[0].packages[0].len(), 2);
    assert_eq!(labels[0].packages[1].len(), 2);
    assert_eq!(&labels[0].delivery, "5 €");
    assert_eq!(labels[0].packages[1][1].product_name, r#"SELEZIONE B "IL VEGETARIANO""#);
    assert_eq!(labels[0].packages[1][1].item_price, 40.0);
    assert_eq!(labels[0].packages[1][1].quantity, 1);
    assert_eq!(labels[0].packages[1][0].product_name, r#"GALLETTO VALLE SPLUGA ALLE ERBE DI MONTAGNA 500 g"#);
    assert_eq!(labels[0].packages[1][0].item_price, 4.6);
    assert_eq!(labels[0].packages[1][0].quantity, 1);

    assert_eq!(labels[1].order_id, 5357);
    assert_eq!(&labels[1].delivery, "local pick up");
    assert_eq!(labels[1].packages[0].len(), 3);
    assert_eq!(labels[1].packages[1].len(), 2);

    let summary = parsed.summary();
    assert_eq!(summary.len(), 8);
    assert_eq!(summary.iter().find(|(key, _)| key == r#"SELEZIONE B "IL VEGETARIANO""#).unwrap().1, 2);
    let second = &summary[1];
    assert_eq!(second.0, "CARNE TRITA DI MANZO PER RAGU' E POLPETTE 500 g");
}

#[test]
fn test_parse_empty_data() {
    let parsed = parse_csv("").unwrap();
    assert_eq!(parsed.data.len(), 0);
}

#[cfg(test)]
const BIG_DATA: &str = include_str!("data-big.csv");

#[test]
fn test_multiple_packages() {
    let parsed = parse_csv(BIG_DATA).unwrap().labels().unwrap();
    let must_have_3_packages = &parsed[3];
    assert_eq!(must_have_3_packages.packages.len(), 3);
}

