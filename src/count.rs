use leptos::*;

stylance::import_crate_style!(css, "src/count.module.scss");

#[component]
pub(crate) fn Count(count: ReadSignal<u32>) -> impl IntoView {
    view! {
        <p class=css::style>
            <strong>"Reactive: "</strong>
            {move || count.get()}
        </p>
        <p class=css::style>
            <strong>"Reactive shorthand: "</strong>
            {count}
        </p>
    }
}
