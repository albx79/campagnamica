use anyhow::{Context, Result};
use csv::ReaderBuilder;
use derive_builder::Builder;
use wasm_bindgen::__rt::core::fmt::{Display, Formatter};
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen::__rt::std::error::Error;

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

#[derive(Debug)]
pub struct ProductData {
    pub data: Vec<ProductRow>
}

impl ProductData {
    pub fn get(&self, index: &str) -> Option<ProductRow> {
        self.data.iter().cloned().find(|row| row.product_name == index)
    }
}

#[derive(Clone, Builder)]
pub struct OrderDetails {
    pub order_id: u32,
    pub customer_name: String,
    pub order_total: String,
    pub delivery: String,
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
                order_total: row.order_total.clone(),
                delivery: Self::map_shipping_to_delivery(row.order_shipping),
                products: Vec::new(),
            };
            for o in rows.iter().sorted_by_key(|r| &r.product_name) {
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

#[cfg(test)]
const DATA: &str = r###""Order ID","Order Date","Order Status","Customer Name","Order Total","Order Shipping","Payment Gateway","Shipping Method","Shipping Address Line 1","Shipping Address Line 2","Shipping Zip/Postcode","Billing Phone Number",_transaction_id,"Product Name","Quantity of items purchased","Item price EXCL. tax"
5358,2020/05/24,processing,"PERINO LUPO","57,10",5,"PayPal o carta di credito",flat_rate:1,"VIA DEI PAZZI 0","SCALA A DESTRA SECONDO PIANO",20146,3355700000,0P128552W4082524Y,"SELEZIONE B ""IL VEGETARIANO""",1,40
5358,2020/05/24,processing,"PERINO LUPO","57,10",5,"PayPal o carta di credito",flat_rate:1,"VIA DEI PAZZI 0","SCALA A DESTRA SECONDO PIANO",20146,3355700000,0P128552W4082524Y,"CARNE TRITA DI MANZO PER RAGU' E POLPETTE 500 g",1,3.5
5358,2020/05/24,processing,"PERINO LUPO","57,10",5,"PayPal o carta di credito",flat_rate:1,"VIA DEI PAZZI 0","SCALA A DESTRA SECONDO PIANO",20146,3355700000,0P128552W4082524Y,"FETTINE DI LONZA DI SUINO 500 g",1,4
5358,2020/05/24,processing,"PERINO LUPO","57,10",5,"PayPal o carta di credito",flat_rate:1,"VIA DEI PAZZI 0","SCALA A DESTRA SECONDO PIANO",20146,3355700000,0P128552W4082524Y,"GALLETTO VALLE SPLUGA ALLE ERBE DI MONTAGNA 500 g",1,4.6
5357,2020/05/24,processing,"Maria Luisa","57,90",0,"PayPal o carta di credito",flat_rate:1,"Via Da Qui 1",,20129,3332750000,5L1092726H247623G,"INSALATA VARIA 500 g",1,1.4
5357,2020/05/24,processing,"Maria Luisa","57,90",0,"PayPal o carta di credito",flat_rate:1,"Via Da Qui 1",,20129,3332750000,5L1092726H247623G,"SELEZIONE B ""IL VEGETARIANO""",1,40
5357,2020/05/24,processing,"Maria Luisa","57,90",0,"PayPal o carta di credito",flat_rate:1,"Via Da Qui 1",,20129,3332750000,5L1092726H247623G,"YOGURT DI CAPRA 500 g",1,3
5357,2020/05/24,processing,"Maria Luisa","57,90",0,"PayPal o carta di credito",flat_rate:1,"Via Da Qui 1",,20129,3332750000,5L1092726H247623G,"10 ARROSTICINI DI SUINO 300 g",1,5
5357,2020/05/24,processing,"Maria Luisa","57,90",0,"PayPal o carta di credito",flat_rate:1,"Via Da Qui 1",,20129,3332750000,5L1092726H247623G,"PANE AI CEREALI ANTICHI 500 g",1,3.5
"###;

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
    assert_eq!(labels[0].products.len(), 4);
    assert_eq!(&labels[0].delivery, "5 €");
    assert_eq!(labels[0].products[3].product_name, r#"SELEZIONE B "IL VEGETARIANO""#);
    assert_eq!(labels[0].products[3].item_price, 40.0);
    assert_eq!(labels[0].products[3].quantity, 1);
    assert_eq!(labels[0].products[2].product_name, r#"GALLETTO VALLE SPLUGA ALLE ERBE DI MONTAGNA 500 g"#);
    assert_eq!(labels[0].products[2].item_price, 4.6);
    assert_eq!(labels[0].products[2].quantity, 1);

    assert_eq!(labels[1].order_id, 5357);
    assert_eq!(&labels[1].delivery, "local pick up");
    assert_eq!(labels[1].products.len(), 5);

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

#[derive(Debug, Clone)]
pub struct ProductRow {
    pub id: u32,
    pub category: String,
    pub provenance: String,
    pub net_weight: f32,
    pub product_type: u32,
    pub departure_code: String,
    pub product_name: String,
    pub price: String,
    pub unit: String,
    pub vat: String,
    pub department: u32,
    pub plu_code: String,
    pub ean_12_chars: String,
    pub ean_13_own: String,
    pub ean_13_vendor: String,
}

pub fn parse_product_data(data: &str) -> Result<ProductData> {
    let reader = ReaderBuilder::new().delimiter(b';').from_reader(data.as_bytes());
    let mut rdr = csv::Reader::from(reader);
    let mut data = Vec::new();
    for result in rdr.records() {
        let ctx = format!("{:?}", &result);
        eprintln!("Read: {}", ctx);
        let record = result.context(ctx)?;

        data.push(ProductRow {
            id: record[0].parse()?,
            category: record[1].to_owned(),
            provenance: record[2].to_owned(),
            net_weight: record[3].parse()?,
            product_type: record[4].parse()?,
            departure_code: record[5].to_owned(),
            product_name: record[6].to_owned(),
            price: record[7].to_owned(),
            unit: record[8].to_owned(),
            vat: record[9].to_owned(),
            department: record[10].parse()?,
            plu_code: record[11].to_owned(),
            ean_12_chars: record[12].to_owned(),
            ean_13_own: record[13].to_owned(),
            ean_13_vendor: record[14].to_owned(),
        });
    }
    Ok(ProductData{data})
}

#[cfg(test)]
const PRODUCT_DATA: &str = r###"Progressivo;Categoria;Provenienza;Preincartato al KG;Tipo Articolo;Codice di partenza;Prodotto;Prezzo;Unita;Iva;Reparto fiscale;Codice PLU Olivetti;12 caratteri EAN;EAN 13 proprio;EAN 13 fornitore
1;FORMAGGI;agricolo;0;5;50001;Mozzarella BIO 350 gr;4,50;pezzo;4%;1;50001;;;2130001004009
2;PRODOTTI MANIPOLATI O TRASFORMATI;agricolo;0;5;;PANE CER ANTICH 500 G;3,50;pezzo;4%;1;50002;;;2130002003704
3;CEREALI;agricolo;0;5;;RISO 1 KG;2,50;pezzo;4%;1;50003;;;2130003002508
4;ORTAGGI;agricolo;0;5;;FINOCCHI 1 KG;2,50;pezzo;4%;1;50006;;;2130006001409
5;FRUTTA;agricolo;0;5;;ARANCE 2 KG;3,50;pezzo;4%;1;50007;;;2130007004003
6;ORTAGGI;agricolo;0;5;;CICORINO 500 G;1,30;pezzo;4%;1;50008;;;2130008001308
7;FRUTTA;agricolo;0;5;;KIWI 1 KG;3,50;pezzo;4%;1;50010;;;2130010003000
8;FRUTTA;agricolo;0;5;;FRAGOLE 500 G;2,50;pezzo;4%;1;50011;;;2130011003009
9;ORTAGGI;agricolo;0;5;;sc-CAROTE 500 G;1,00;pezzo;4%;1;50012;;;2109042020040
10;ORTAGGI;agricolo;0;5;;DATTERINO 500 g;2,50;pezzo;4%;1;50014;;;2109042020033
11;FORMAGGI;agricolo;0;5;;FORM TOMA LATCRU 300G;3,90;pezzo;4%;1;50015;;;2130015003203
12;FORMAGGI;agricolo;0;5;;FORM CAPRINO 200 G;3,50;pezzo;4%;1;50016;;;2130016003004
13;ORTAGGI;agricolo;0;5;;ZUCCHINE 500 Gs;1,50;pezzo;4%;1;50017;;;2130017003003
14;ORTAGGI;agricolo;0;5;;CIMA DI RAPA 500 g;1,50;pezzo;4%;1;50018;;;2130018002500
15;FORMAGGI;agricolo;0;5;;MOZ LATT BUFALA 250 G;3,50;pezzo;4%;1;50019;;;2130019003506
16;FORMAGGI;agricolo;0;5;;GRANA 30 MESI 300 G;3,80;pezzo;4%;1;50025;;;2130025003804
17;CARNI E SALUMI;agricolo;0;5;;PROSCIUT. COTTO 200 G;3,70;pezzo;10%;2;50004;;;2130004003702
18;CARNI E SALUMI;agricolo;0;5;;PETTO DI POLLO 700 G;7,00;pezzo;10%;2;50005;;;2130005007006
19;DERIVATI ANIMALI;agricolo;0;5;;UOVA 6;2,40;pezzo;10%;2;50009;;;2130009002403
20;CARNI E SALUMI;agricolo;0;5;;HAMBURGER 4 (500 G);6,00;pezzo;10%;2;50020;;;2130020006008
21;PESCI E MOLLUSCHI;agricolo;0;5;;STORIONE FILETTO 450G;5,00;pezzo;10%;2;50021;;;2130021007004
22;CARNI E SALUMI;agricolo;0;5;;GALLET VALLESPL 500 G;4,00;pezzo;10%;2;50022;;;2130022004002
23;CARNI E SALUMI;agricolo;0;5;;PROSC CRUDO 200 G;5,00;pezzo;10%;2;50023;;;2130023005008
24;CARNI E SALUMI;agricolo;0;5;;SALAME PICCOLO 200 G;4,00;pezzo;10%;2;50024;;;2109042020170
"###;

#[cfg(test)]
#[test]
fn test_parse_product_data() {
    let parsed = parse_product_data(PRODUCT_DATA).unwrap();
    assert_eq!(parsed.data[0].id, 1);
    assert_eq!(parsed.data[23].id, 24);
    assert_eq!(parsed.data[0].product_name, "Mozzarella BIO 350 gr");
    assert_eq!(parsed.data[0].ean_13_vendor, "2130001004009");

    assert_eq!(&parsed.get("Mozzarella BIO 350 gr").unwrap().ean_13_vendor, "2130001004009");
}
