use leptos::prelude::AddAnyAttr;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
            <NavDrawer>
        <NavCategory value="area">
            <NavCategoryItem slot icon=icondata::AiAreaChartOutlined>
                "Area Chart"
            </NavCategoryItem>
            <NavSubItem value="target">
                "Target"
            </NavSubItem>
            <NavSubItem value="above">
                "Above"
            </NavSubItem>
            <NavSubItem value="below">
                "Below"
            </NavSubItem>
        </NavCategory>
        <NavCategory value="pie">
            <NavCategoryItem slot icon=icondata::AiPieChartOutlined>
                "Pie Chart"
            </NavCategoryItem>
            <NavSubItem value="pie-target">
                "Pie Target"
            </NavSubItem>
            <NavSubItem value="pin-above">
                "Pin Above"
            </NavSubItem>
            <NavSubItem value="pin-below">
                "Pin Below"
            </NavSubItem>
        </NavCategory>
        <NavItem
            icon=icondata::AiGithubOutlined
            value="github"
            href="https://github.com/thaw-ui/thaw"
            attr:target="_blank"
        >
            "Github"
        </NavItem>
        <NavItem icon=icondata::AiChromeOutlined value="chrome">
            "Chrome"
        </NavItem>
    </NavDrawer>
    }
    // let (name, set_name) = create_signal(String::new());
    // let (greet_msg, set_greet_msg) = create_signal(String::new());
    //
    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };
    //
    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }
    //
    //         let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };
    //
    // view! {
    //     <main class="container">
    //         <div class="row">
    //             <a href="https://tauri.app" target="_blank">
    //                 <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
    //             </a>
    //             <a href="https://docs.rs/leptos/" target="_blank">
    //                 <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
    //             </a>
    //         </div>
    //
    //         <p>"Click on the Tauri and Leptos logos to learn more."</p>
    //
    //         <form class="row" on:submit=greet>
    //             <input
    //                 id="greet-input"
    //                 placeholder="Enter a name..."
    //                 on:input=update_name
    //             />
    //             <button type="submit">"Greet"</button>
    //         </form>
    //
    //         <p><b>{ move || greet_msg.get() }</b></p>
    //     </main>
    // }
}
