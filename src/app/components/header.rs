use leptos::prelude::*;
use leptos::*;
use leptos_router::hooks::use_location;
const INPUT_STYLE: &str = "border-b-0 border-[#7734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";
const INPUT_STYLE_SELECTED: &str =
    "border-b-2 border-[#9734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";

#[component]
pub fn Header() -> impl IntoView {
    let location = use_location();
    let path = location.pathname;
    view! {
        <div class="flex fixed top-0 items-center px-20 pt-8 mx-auto w-full h-12 align-center">
            <nav class="flex flex-row w-full h-12 max-w-[52rem]">
                <div class=move || get_style_from_url(path.get().as_str(), "/")>
                    <a href="/">"Dashboard"</a>
                </div>
                <div class=move || get_style_from_url(location.pathname.get().as_str(), "/team")>
                    <a href="/team">"Team"</a>
                </div>
            </nav>
        </div>
    }
}

fn get_style_from_url(url: &str, match_url: &str) -> &'static str {
    if url == match_url {
        INPUT_STYLE_SELECTED
    } else {
        INPUT_STYLE
    }
}
