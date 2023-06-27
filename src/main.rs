use js_sys::{Array, Function, Object, Promise, Reflect};
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use yew::prelude::*;

pub struct Metamask {
    provider: Option<Object>,
}

impl Metamask {
    pub fn connect(&mut self) {
        let window = web_sys::window().unwrap();
        self.provider = window.get("ethereum");
        if let Some(ref provider) = self.provider {
            web_sys::console::log_1(&"connected, hehe".into());
            self.get_accounts();
        } else {
            web_sys::console::log_1(&"failed to connect".into());
        }
    }

    pub fn get_accounts(&self) {
        if let Some(ref provider) = self.provider {
            let request = Reflect::get(provider, &JsValue::from("request"))
                .unwrap()
                .dyn_into::<Function>()
                .unwrap();

            let payload = Object::new();

            Reflect::set(&payload, &JsValue::from("method"), &JsValue::from("eth_requestAccounts")).unwrap();

            let promise = Promise::from(request.call1(&JsValue::null(), &payload).unwrap());

            spawn_local(async move {
                let accounts = JsFuture::from(promise).await.unwrap();
                web_sys::console::log_1(&accounts);
            });
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
