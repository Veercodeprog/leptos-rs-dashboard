use leptos::prelude::*;
use leptos::*; // required for ElementChild trait

use leptos_router::*;

const INPUT_STYLE: &str = "border-b-0 border-[#7734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";

const INPUT_STYLE_SELECTED: &str =
    "border b-2 border-[#9734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";

#[component]
pub fn Header()-> impl IntoView{
    let (current_path: ReadSignal<String>, set_current_path: WriteSignal<String> ) = create_signal(String::new());
    create_effect(move |_| {
        let router_context: Option<RouterContext> = use_context::<RouterContext>();
        match router_context{
            Some(route_context: RouterContext)=> {
                let path: String = route_context.pathname().get();
                set_current_path(path);
            },
            None=>{
                set_current_path(String::from("/"));
            }
        }
    });
    view!{
        <div class="flex fixed top-0 items-center px-20 pt-8 mx-auto w-full h-12 align-center">
<nav class="flex w-full h-12 felx-row max-w-[52rem]">
<div class={move || get_style_from_url(&current_path, "/")}>
    <a href= "/">"Dashboard"</a>
        </div>
 <div class={move || get_style_from_url(&current_path, "/team")}>
    <a href= "/">"Team"</a>
        </div>

        </nav>
        </div>
    }
}
