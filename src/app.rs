use yew::prelude::*;
use crate::woocsv::{parse_csv, WooCommerceRow, WooCommerceRowBuilder, InputData, OrderDetails, OrderDetailsBuilder, OrderItem, OrderItemBuilder, parse_product_data, ProductData};
use wasm_bindgen::__rt::std::error::Error;

#[derive(Debug)]
pub enum Msg {
    UpdateCsv(String),
    UpdateProducts(String),
}

pub struct Gui {
    link: ComponentLink<Self>,
    input_data: Option<InputData>,
    product_list: Option<ProductData>,
    error: Option<Box<dyn Error>>,
}

impl Component for Gui {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Gui { link, input_data: None, product_list: None, error: None }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateCsv(data) => {
                stdweb::console!(log, "Received update csv:", &data);
                let parsed = parse_csv(&data);
                match parsed {
                    Ok(data) => {
                        self.input_data = Some(data);
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e.into());
                        self.input_data = None;
                    }
                }
            }
            Msg::UpdateProducts(products) => {
                stdweb::console!(log, "Received product list:", &products);
                let parsed = parse_product_data(&products);
                match parsed {
                    Ok(data) => {
                        self.product_list = Some(data);
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e.into());
                        self.product_list = None;
                    }
                }
            }
        };
        true
    }

    fn view(&self) -> Html {
        use yew::InputData;
        let empty = html! {<div/>};
        html! {
            <div width="100%">
                <div>{"Copy-paste your woocommerce CSV into the textarea below:"}</div>
                <textarea
                    rows="30" cols="120"
                    oninput=self.link.callback(|e: InputData| Msg::UpdateCsv(e.value))
                />
                <div>{"Copy-paste your product list into the textarea below:"}</div>
                <textarea
                    rows="30" cols="120"
                    oninput=self.link.callback(|e: InputData| Msg::UpdateProducts(e.value))
                />
                {
                    self.input_data.as_ref().map(|d| html!{
                    <div>
                        <h2>{"Labels"}</h2>
                        {
                            d.labels(&self.product_list).map(|labels| html!{
                                <div>
                                {
                                    labels.iter().map(|label| label.view()).collect::<Html>()
                                }
                                <p>{format!("Number of deliveries: {}", labels.len())}</p>
                                </div>
                            }).unwrap_or_else(|e| {
                                html! {
                                    <div class="error">{e.to_string()}</div>
                                }
                            })
                        }
                        <hr/>
                        <h2>{"Summary"}</h2>
                        <table>
                            <thead>
                                <tr>
                                    <th class="product" align="left">{"Prodotto"}</th>
                                    <th class="quantity" align="right">{"Quantità"}</th>
                                </tr>
                            </thead>
                            <tbody> {
                                d.summary().iter().map(|(prod, qty)| html! {
                                    <tr>
                                        <td>{&prod}</td>
                                        <td align="right">{format!("{}", qty)}</td>
                                    </tr>
                                }).collect::<Html>()
                            } </tbody>
                        </table>
                    </div>
                    }).unwrap_or(empty.clone())
                }
                {
                    self.error.as_ref().map(|e| html!{
                        <div class="error">{e.to_string()}</div>
                    }).unwrap_or(empty.clone())
                }
            </div>
        }
    }
}

impl Component for InputData {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        InputData { data: Vec::new() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true // should render
    }

    fn view(&self) -> Html {
        html! {
        <table>
            { self.data.iter().map(WooCommerceRow::view).collect::<Html>() }
        </table>
        }
    }
}

impl Component for WooCommerceRow {
    type Message = ();
    type Properties = WooCommerceRow;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <tr>
                <td>{self.order_id}</td><td>{&self.customer_name}</td><td>{&self.product_name}</td>
            </tr>
        }
    }
}

impl Properties for WooCommerceRow {
    type Builder = WooCommerceRowBuilder;

    fn builder() -> Self::Builder {
        WooCommerceRowBuilder::default()
    }
}

impl Properties for OrderDetails {
    type Builder = OrderDetailsBuilder;

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
}

impl Component for OrderDetails {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
         <div class="the-label">
            <table class="address" width="100%">
                <tr>
                    <td width="60%" valign="top">
                        <span>{format!("Ordine N.: {}", self.order_id)}</span><br/>
                        <span>{format!("Data: {}", self.order_date)}</span><br/>
                        <span>{format!("Tel.: {}", self.billing_phone_number)}</span><br/>
                    </td>
                    <td>
                        <b>{"Indirizzo:"}</b><br/>
                        <span>{&self.customer_name}</span><br/>
                        <span>{&self.shipping_address_line_1}</span><br/>
                        <span>{&self.shipping_address_line_2}</span><br/>
                        <span>{format!("Milano, {}", self.shipping_postcode)}</span><br/>
                        <span>{"Italia"}</span><br/>
                    </td>
                </tr>
            </table>

            <table class="order-items" width="100%">
                <thead>
                    <tr height="3vm">
                        <th class="quantity">{"Quantità"}</th>
                        <th class="product">{"Prodotto"}</th>
                        <th class="barcode">{"Codice"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        self.products.iter().map(|product| html! {
                            <tr>
                                <td class="quantity" align="center">{product.quantity}</td>
                                <td class="product"><b>{&product.product_name}</b></td>
                                <td class="barcode">{
                                    product.ean13.as_ref().map(|code| html! {
                                                    <img src={format!("https://barcode.tec-it.com/barcode.ashx?data={}&code=EAN13&dpi=96", code)}/>
                                                }).unwrap_or_else(|| html!{ <span/> })
                                    }
                                </td>

                            </tr>
                        }).collect::<Html>()
                    }
                    <tr>
                        <td align="right"><b>{"Consegna"}</b></td>
                        <td align="center">{&self.delivery}</td>
                    </tr>
                    <tr>
                        <td align="right"><b>{"Metodo Pagamento"}</b></td>
                        <td align="center">{&self.payment_gateway}</td>
                    </tr>
                    <tr>
                        <td align="right"><b>{"Totale"}</b></td>
                        <td align="center">{format!("{}€", self.order_total)}</td>
                    </tr>
                </tbody>
            </table>
            <p><br/></p>
        </div>
        }
    }
}

impl Properties for OrderItem {
    type Builder = OrderItemBuilder;

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
}

// impl Component for OrderItem {
//     type Message = ();
//     type Properties = Self;
//
//     fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
//         props
//     }
//
//     fn update(&mut self, _msg: Self::Message) -> bool {
//         true
//     }
//
//     fn view(&self) -> Html {
//         html! {
//             <tr>
//                 <td class="quantity" align="center">{self.quantity}</td>
//                 <td class="product"><b>{&self.product_name}</b></td>
//             </tr>
//         }
//     }
// }
