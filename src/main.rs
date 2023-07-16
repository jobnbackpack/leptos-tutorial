use leptos::{ev::MouseEvent, *};

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
fn DynList(cx: Scope, #[prop(default = 3)] initial_length: usize) -> impl IntoView {
    let mut next_counter_id: usize = initial_length;

    let initial_counters: Vec<(usize, (ReadSignal<usize>, WriteSignal<usize>))> = (0
        ..initial_length)
        .map(|id: usize| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(cx, initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(cx, next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };

    view! { cx,
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    view=move |cx, (id, (count, set_count))| {
                        view! { cx,
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    let values = vec![0, 1, 2];

    let length = 3;
    // create a list of N signals
    let counters = (1..=length).map(|idx| create_signal(cx, idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! { cx,
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n +=1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view(cx);

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

            // this will just render "012"
            <p>values to string: {values.clone()}</p>

            // or we can wrap them in a list
            <ul>
                {values.into_iter()
                    .map(|n| view! {cx, <li>{n}</li>})
                    .collect_view(cx)}
            </ul>

            <h2>static counter list</h2>
            <ul>{counter_buttons}</ul>

            <h2>dynamic counter list</h2>
            <DynList initial_length=5/>
        </div>
    }
}
