use gloo_timers::future::TimeoutFuture;
use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_meta::*;
use uuid::Uuid;

fn main() {
    mount_to_body(|cx| view! { cx, <div class="container"><App/></div> })
}

async fn add_todo(text: &str) -> Uuid {
    _ = text;
    TimeoutFuture::new(10_000).await;
    Uuid::new_v4()
}

#[component]
fn AsyncAction(cx: Scope) -> impl IntoView {
    let add_todo = create_action(cx, |input: &String| {
        let input = input.to_owned();
        async move { add_todo(&input).await }
    });

    let submitted = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();

    let input_ref = create_node_ref::<Input>(cx);
    view! {cx,
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                add_todo.dispatch(input.value());
            }
        >
            <label>
                "What do you need to do?"
                <input type="text"
                    node_ref=input_ref
                />
            </label>
            <button type="submit">"Add Todo"</button>
        </form>
        <p>{move || pending().then(|| "Loading...")}</p>
        <p>
            "Submitted: "
            <code>{move || format!("{:#?}", submitted())}</code>
        </p>
        <p>
            "Pending: "
            <code>{move || format!("{:#?}", pending())}</code>
        </p>
        <p>
            "Todo ID: "
            <code>{move || format!("{:#?}", todo_id())}</code>
        </p>
    }
}

async fn load_data() -> String {
    TimeoutFuture::new(10_000).await;
    "hello world".to_string()
}

#[component]
fn AsyncResource(cx: Scope) -> impl IntoView {
    let async_data = create_resource(
        cx,
        || (),
        |_| async move {
            log!("loading data from API");
            load_data().await
        },
    );
    view! { cx,
        <h2>"My Async Data"</h2>
        {move || match async_data.read(cx) {
            None => view! { cx, <p>"Loading..."</p>}.into_view(cx),
            Some(data) => view! {cx, <p>"Server returned: " {data}</p> }.into_view(cx)
        }}
    }
}

#[component]
fn TakesChildren<F, IV>(cx: Scope, render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {cx,
        <h2>"Render Prop"</h2>
        {render_prop()}
        <h2>"Render Children"</h2>
        {children(cx)}
    }
}

#[component]
fn NumericInput(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! { cx,
        <h2>"Error Handling"</h2>
        <label>
            "Type a number (or something that's not a number)"
            <input on:input=on_input/>
            <ErrorBoundary
                fallback=|cx, errors| view! { cx,
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {move || errors.get().into_iter().map(|(_,e)| view! {cx, <li> {e.to_string()}</li>}).collect_view(cx)
                            }
                        </ul>
                    </div>
                    }>
        <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}

#[component]
fn ControlledForm(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Controlled".to_string());

    view! { cx,
        <input type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn UncontrolledForm(cx: Scope) -> impl IntoView {
    let (firstname, set_firstname) = create_signal(cx, "Uncontrolled".to_string());

    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();
        set_firstname(value);
    };

    view! { cx,
        <form on:submit=on_submit>
            <input type="text"
                value=firstname
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {firstname}</p>
    }
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
    provide_meta_context(cx);
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    let values = vec![0, 1, 2];

    let length = 3;
    // create a list of N signals
    let counters = (1..=length).map(|idx| create_signal(cx, idx));
    let formatter = |_| format!("lol — Leptos Online");

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
        <main>
        <Title formatter/>
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
            <p>
            {move || if count() % 2 == 1 {
                "Odd"
            } else {
                "Even"
            }}
            </p>
            <Show
                when=move || { count() > 10 }
                fallback=|cx| view! {cx, <p>"I'm still smaller than 10"</p>}
            >
                <strong>"I'm bigger than 10"</strong>
            </Show>

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

            <h2>Controlled Form:</h2>
            <ControlledForm />

            <h2>Uncontrolled Form:</h2>
            <UncontrolledForm />

            <NumericInput />

            <TakesChildren render_prop=|| view!{ cx, <p>"I was in the props"</p>}><p>"I'm a child"</p></TakesChildren>

            <AsyncResource />

            <AsyncAction />
        </div>
    </main>
    }
}
