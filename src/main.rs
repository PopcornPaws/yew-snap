use js_sys::{Array, Object};
use wasm_bindgen::JsValue;
use yew::prelude::*;


pub struct Metamask {
    provider: Option<Object>,
}

impl Metamask {
    pub fn connect(&mut self) {
        let window = web_sys::window().unwrap();
        self.provider = window.get("ethereum");
        if let Some(ref provider) = self.provider {
            web_sys::console::log_1(&provider.to_string().into());
        } else {
            web_sys::console::log_1(&"failed to connect".into());
        }
    }
}

pub enum Msg {
    Connect,
}

impl Component for Metamask {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { provider: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Connect => {
                self.connect();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let button_msg = if self.provider.is_some() {
            "Connected"
        } else {
            "Connect Metamask"
        };
        html! {
            <div>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Connect)}>
                    { button_msg }
                </button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Metamask>::new().render();
}
