use crate::app::components::{AddPersonModal, Header};
use leptos::prelude::*;
use leptos::*; // required for ElementChild trait
#[component]
pub fn Teampage() -> impl IntoView {
    const ADD_BUTTON_STYLE: &str = "bg-[#9634e7] px-8 py-2 rounded text-white
        transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

    let (if_show_modal, set_if_show_modal) = create_signal(false);

    // for showing/animating the toast message
    // let (if_show_toast, set_if_show_toast) = create_signal(false);
    /*   let (toast_message, set_toast_message) = create_signal(ToastMessage::new()); */
    let on_click = move |_| {
        set_if_show_modal(!if_show_modal());
    };

    view! {
        <div class="overflow-x-hidden min-h-screen bg-gray-900">
            <Header />
            <div class="justify-center items-center mx-auto w-full max-w-[64rem] align-center">

                <div class="mt-20" style="margin-top: 4rem;">
                    <div class="flex flex-col justify-center items-center mx-auto w-full text-white z-25">

                        <Show when=move || { if_show_modal() }>
                            <AddPersonModal set_if_show_modal />

                        </Show>
                        <div class="flex flex-row w-full max-w-[52rem]">
                            <div class="pr-4 mt-4 text-xl">"Memebers"</div>
                            <hr class="pt-4 pr-4 pl-4 mt-8 mr-4 w-full max-w-[48rem]" />
                            <button on:click=on_click class=ADD_BUTTON_STYLE>
                                "Add"
                            </button>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    }
}
