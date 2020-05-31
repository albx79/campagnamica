use crate::app::{InputData, WooCommerceRow};
use wasm_bindgen::__rt::std::error::Error;
use csv::ReaderBuilder;

pub fn parse_csv(mut data: String) -> Result<InputData, Box<dyn Error>> {
    let reader = ReaderBuilder::new().from_reader(data.as_bytes());
    let mut rdr = csv::Reader::from(reader);
    let mut input_data: Vec<WooCommerceRow> = Vec::new();
    for result in rdr.deserialize() {
        let record: WooCommerceRow = result?;
        input_data.push(record);
    }
    Ok(InputData{ data: input_data })
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