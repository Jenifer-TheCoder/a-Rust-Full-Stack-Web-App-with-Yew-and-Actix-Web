use yew::prelude::*;
use yew::functional::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;

#[function_component(App)]
fn app() -> Html {
    let message = use_state(|| "Loading...".to_string());

    {
        let message = message.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let resp = Request::get("http://127.0.0.1:9005/api/message")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                message.set(resp);
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <h1>{ "Rust Full-Stack App" }</h1>
            <p>{ &*message }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}