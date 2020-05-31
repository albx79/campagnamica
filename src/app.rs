use yew::prelude::*;
use floating_bar::r64;
use stdweb::traits::INode;
use stdweb::web::event::InputEvent;
use crate::woocsv::{parse_csv, WooCommerceRow, WooCommerceRowBuilder};

pub enum Msg {
    UpdateCsv(String),
    ProcessCsv,
    PopulateInputData(InputData),
}

pub struct Gui {
    link: ComponentLink<Self>,
    textarea: NodeRef,
    text: String,
    input_data: Option<InputData>,
}

impl Component for Gui {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Gui{link, textarea: NodeRef::default(), text: "".to_owned(), input_data: None}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ProcessCsv => {
                let parsed = parse_csv(self.text.clone()).expect("failed to parse");
                self.link.send_message( Msg::PopulateInputData(parsed));
            },
            Msg::UpdateCsv(data) => {
                self.text = data;
            }
            _ => ()

        };
        false
    }

    fn view(&self) -> Html {
        use yew::InputData;
        let elem: Option<()> = Some(());
        html! {
            <div>
                <div>{"Copy-paste your woocommerce CSV into the textarea below:"}</div>
                <textarea
                    ref=self.textarea.clone() rows="40" cols="100"
                    oninput=self.link.callback(|e: InputData| Msg::UpdateCsv(e.value))
                />
                <div><button onclick=self.link.callback(|_| Msg::ProcessCsv)>{"Process"}</button></div>
                <div> {
                    self.input_data.as_ref().map(|d| d.view()).unwrap_or(html!{<div/>})
                }</div>
            </div>
        }
    }
}

impl Component for InputData {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        InputData{ data: Vec::new() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true // should render
    }

    // fn change(&mut self, _props: Self::Properties) -> bool {
    //     true
    // }

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

pub struct OrderDetails {
    order_id: u32,
    products: Vec<OrderItem>,
}

pub struct OrderItem {
    product_name: String,
    quantity: u32,
    item_price: r64,
}

pub struct InputData {
    pub data: Vec<WooCommerceRow>
}
