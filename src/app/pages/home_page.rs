use leptos::prelude::*;
use leptos::*; // required for ElementChild trait

#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <div class="bg-gray-900 overflow-x-hide">
            <div class="justify-center items-center mx-auto w-full max-w-[64rem] align-center">
                "Home page here"
            </div>
        </div>
    }
}
