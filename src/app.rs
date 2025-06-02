use std::fmt::Display;

use leptos::{ev::SubmitEvent, leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize};
use uuid::{Timestamp, Uuid};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(what: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Role {
    User,
    Ai
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => {
                f.write_str("User")
            },
            Role::Ai => {
                f.write_str("Ai")
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub uuid: Uuid,
}

#[component]
pub fn App() -> impl IntoView {
    let (messages, set_messages) = signal(vec![]);

    view! {
        <div class="messages">
            <For
            each=move || messages.get()
            key=|msg: &Message| msg.uuid
            children=move |msg| {
                console_log(format!("{:?}", msg.role).as_str());

                let role_class = match msg.role {
                    Role::User => "userMessages",
                    Role::Ai => "aiMessages",
                };
                let class = format!("{} {}", role_class, "message");

                view! {
                    <div class={ class }>
                    { msg.content }
                    </div>
                }
            }
        />
        </div>

        <main class="container">
            <form class="row" on:submit=move |ev: SubmitEvent| {
                ev.prevent_default();

                let inp = ev.target()
                    .and_then(|target| target.dyn_into::<leptos::web_sys::HtmlFormElement>().ok())
                    .and_then(|form| form.elements().named_item("message"))
                    .and_then(|elem| elem.dyn_into::<leptos::web_sys::HtmlInputElement>().ok());

                if let Some(inp) = inp {
                    if !inp.value().trim().is_empty() {
                        let msg = Message {
                            role: Role::User,
                            content: inp.value().trim().to_string(),
                            uuid: Uuid::now_v7(),
                        };

                        set_messages.update(|messages| {
                            messages.push(msg.clone());
                        });

                        spawn_local(async move {
                            let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "message": msg })).unwrap();
                            let response = invoke("process_message", args).await;
                            let response: Message = serde_wasm_bindgen::from_value(response).unwrap();
                            set_messages.update(|messages| {
                                messages.push(response);
                            });
                            console_log("Message added!");
                        });
                    }

                    inp.set_value("");
                }
            }>
                <input
                    id="message"
                    placeholder="Enter a message..."
                />
                <button type="submit">"Send"</button>
            </form>
        </main>
    }
}
