use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    view! { cx,
        <div>
            <button
                // define an event listener with on:
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }
                class:red=move || count() % 2 == 1
            >
                "Click me: "
                {count}
            </button>
            <progress
                max="50"
                value=double_count
                />
            <p>
                "Double Count: "
                {double_count}
            </p>
        </div>
    }
}
