use yew::prelude::*;
use crate::woocsv::{parse_csv, WooCommerceRow, WooCommerceRowBuilder, InputData, OrderDetails, OrderDetailsBuilder, OrderItem, OrderItemBuilder};
use wasm_bindgen::__rt::std::error::Error;

#[derive(Debug)]
pub enum Msg {
    UpdateCsv(String),
}

pub struct Gui {
    link: ComponentLink<Self>,
    input_data: Option<InputData>,
    error: Option<Box<dyn Error>>,
}

impl Component for Gui {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Gui { link, input_data: None, error: None }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateCsv(data) => {
                stdweb::console!(log, "Received update csv:", &data);
                let parsed = parse_csv(data);
                match parsed {
                    Ok(data) => self.input_data = Some(data),
                    Err(e) => self.error = Some(e),
                }
            }
            x => { stdweb::console!(log, "Unknown message", format!("{:?}", x)); }
        };
        true
    }

    fn view(&self) -> Html {
        use yew::InputData;
        let empty = html! {<div/>};
        html! {
            <div>
                <div>{"Copy-paste your woocommerce CSV into the textarea below:"}</div>
                <textarea
                    rows="30" cols="120"
                    oninput=self.link.callback(|e: InputData| Msg::UpdateCsv(e.value))
                />
                {
                    self.input_data.as_ref().map(|d| html!{
                    <div>
                        <h2>{format!("Raw Data ({} rows)", d.data.len())}</h2>
                        <div>{d.view()}</div>
                        <h2>{"Labels"}</h2>
                        <div>
                            <table>
                            {
                                d.labels().map(|labels| {
                                    labels.iter().map(|label| label.view()).collect::<Html>()
                                }).unwrap_or_else(|e| {
                                    html! {
                                        <div class="error">{e.to_string()}</div>
                                    }
                                })
                            }
                            </table>
                        </div>
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
                <td>{self.order_id}</td><td>{self.customer_name.clone()}</td><td>{self.product_name.clone()}</td>
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
            <tr>
                <td>{self.order_id}</td>
                <td>
                    <table>
                    {
                        self.products.iter().map(|product| { product.view() }).collect::<Html>()
                    }
                    </table>
                </td>
            </tr>
        }
    }
}

impl Properties for OrderItem {
    type Builder = OrderItemBuilder;

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
}
impl Component for OrderItem {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html!{
            <tr>
                <td>{self.quantity}</td>
                <td>{self.product_name.clone()}</td>
                <td>{format!("{:.02}", self.item_price)}</td>
            </tr>
        }
    }
}