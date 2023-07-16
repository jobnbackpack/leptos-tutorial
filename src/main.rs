use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn ProgressBar(
    cx: Scope,
    /// The maximum value of the progress bar
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { cx,
        <progress
            max=max
            value=progress
        />
    }
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
            // .into() converts `ReadSignal` to `Signal`
            <ProgressBar progress=count max=100 />
            // use `Signal::derive()` to wrap a derived signal
            <ProgressBar progress=Signal::derive(cx, double_count) />
            <p>
                "Double Count: "
                {double_count}
            </p>
        </div>
    }
}
