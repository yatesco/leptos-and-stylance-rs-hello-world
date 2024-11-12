use crate::count::Count;
use leptos::*;
stylance::import_crate_style!(css, "src/app.module.scss");

#[component]
pub(crate) fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
        class = css::style
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <Count count = count/>
    }
}
