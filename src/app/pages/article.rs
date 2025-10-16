use crate::app::components::Header;
use crate::app::utils::fetch_all_categories;
use leptos::prelude::*;
use leptos::*; // required for ElementChild trait

#[component]
pub fn Article() -> impl IntoView {
    // Use Resource (SSR-friendly) and name it `articles`
    let articles = LocalResource::new(|| async { fetch_all_categories().await });

    view! {
        <div class="overflow-x-hidden min-h-screen bg-gray-900">
            <div class="justify-center items-center mx-auto w-full text-white max-w-[64rem] align-center">
                <Header />

            </div>
        </div>
    }
}
