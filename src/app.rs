use crate::{
    activity_list::ActivityList,
    auth::*,
    error_template::{AppError, ErrorTemplate},
    fit_upload::FitUploadForm,
    heartrate_summary_chart::HeartrateSummaryChart,
    training_load_chart::TrainingLoadChart,
    workout_schedule::WorkoutCalendar,
};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, TimeZone};
use leptos::{html::Label, *};
use leptos_meta::*;
use leptos_router::*;

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
        use sqlx::PgPool;

        pub fn pool() -> Result<PgPool, ServerFnError> {
           use_context::<PgPool>()
                .ok_or_else(|| ServerFnError::new("Pool missing."))
        }

        pub fn auth() -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>()
                .ok_or_else(|| ServerFnError::new("Auth session missing."))
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    let (show_upload, set_show_upload) = create_signal(false);
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let signup = create_server_action::<Signup>();
    let user = create_blocking_resource(
        move || {
            (
                login.version().get(),
                logout.version().get(),
                signup.version().get(),
            )
        },
        move |_| async move {
            let user = get_user().await.unwrap_or(None);
            user.is_some()
        },
    );
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/toedirs.css"/>

        // sets the document title
        <Title text="Welcome to Toedi"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Routes>
                <Route
                    path="/home"
                    view=move || {
                        view! {
                            <Suspense fallback=|| ()>
                                {move || match user.get() {
                                    Some(false) => view! { <Home/> }.into_view(),
                                    Some(true) => view! { <Redirect path="/"/> }.into_view(),
                                    None => ().into_view(),
                                }}

                            </Suspense>
                        }
                    }
                >

                    <Route path="/signup" view=move || view! { <Signup action=signup/> }/>
                    <Route path="/login" view=move || view! { <Login action=login/> }/>
                    <Route path="" view=Landing/>
                </Route>
                <Route
                    path="/"
                    view=move || {
                        view! {
                            <Suspense fallback=|| ()>
                                {move || match user.get() {
                                    Some(true) => {
                                        view! {
                                            <nav class="teal lighten-2">
                                                <div class="nav-wrapper">
                                                    <a href="#" class="brand-logo">
                                                        Toedi
                                                    </a>
                                                    <ul id="nav-mobile" class="right hide-on-med-and-down">
                                                        <li>

                                                            <A href="/" class="">
                                                                Overview
                                                            </A>
                                                        </li>
                                                        <li>

                                                            <A href="/activities" class="">
                                                                Activities
                                                            </A>
                                                        </li>
                                                        <li>

                                                            <A href="/calendar" class="">
                                                                Calendar
                                                            </A>
                                                        </li>
                                                        <li>
                                                            <a
                                                                class="waves-effect waves-light btn"
                                                                on:click=move |_| { set_show_upload.update(|v| *v = !*v) }
                                                            >
                                                                Upload
                                                                <i class="material-symbols-rounded right">upload</i>
                                                            </a>
                                                        </li>
                                                        <li>
                                                            <ActionForm action=logout>
                                                                <button type="submit" class="btn-flat waves-effect">
                                                                    "Log Out"
                                                                </button>
                                                            </ActionForm>
                                                        </li>
                                                    </ul>
                                                </div>
                                            </nav>
                                            <main>
                                                <Outlet/>
                                                <FitUploadForm show=show_upload show_set=set_show_upload/>
                                            </main>
                                        }
                                            .into_view()
                                    }
                                    Some(false) => view! { <Redirect path="/home"/> }.into_view(),
                                    None => ().into_view(),
                                }}

                            </Suspense>
                        }
                    }
                >

                    <Route path="" view=Overview/>

                    <Route path="/activities" view=ActivityList/>
                    <Route path="/calendar" view=WorkoutCalendar/>

                </Route>
            </Routes>
        </Router>
    }
}

#[component]
fn Overview() -> impl IntoView {
    //overview page
    let from_date = create_rw_signal(Some(
        (Local::now() - Duration::days(120))
            .date_naive()
            .format("%Y-%m-%d")
            .to_string(),
    ));
    let to_date = create_rw_signal(Some(
        Local::now().date_naive().format("%Y-%m-%d").to_string(),
    ));
    let from_memo = create_memo(move |_| {
        from_date().and_then(|d| {
            NaiveDate::parse_from_str(d.as_str(), "%Y-%m-%d")
                .map(|d| {
                    Local
                        .from_local_datetime(&d.and_hms_opt(0, 0, 0).unwrap())
                        .unwrap()
                })
                .ok()
        })
    });
    let to_memo = create_memo(move |_| {
        to_date().and_then(|d| {
            NaiveDate::parse_from_str(d.as_str(), "%Y-%m-%d")
                .map(|d| {
                    Local
                        .from_local_datetime(&d.and_hms_opt(0, 0, 0).unwrap())
                        .unwrap()
                })
                .ok()
        })
    });
    view! {
        <div class="container">
            <div class="row">
                <div class="col s6">
                    <input
                        type="date"
                        value=from_date
                        on:change=move |ev| {
                            from_date
                                .update(|v| {
                                    *v = Some(event_target_value(&ev));
                                })
                        }
                    />

                    <label for="from_date">From</label>
                </div>
                <div class="col s6">
                    <input
                        type="date"
                        value=to_date
                        on:change=move |ev| {
                            to_date
                                .update(|v| {
                                    *v = Some(event_target_value(&ev));
                                })
                        }
                    />

                    <label for="to_date">To</label>
                </div>
            </div>
            <div class="row">
                <div class="col s12 m6 l4 p-1">
                    <div class="card">
                        <div class="card-content teal white-text">
                            <span class="card-title">Hearrate Zones</span>
                            <HeartrateSummaryChart from=from_memo to=to_memo/>
                        </div>
                    </div>
                </div>
                <div class="col s12 m6 l4 p-1">
                    <div class="card">
                        <div class="card-content teal white-text">
                            <span class="card-title">Training LoadChart</span>
                            <TrainingLoadChart from=from_memo to=to_memo/>
                        </div>
                    </div>
                </div>
                <div class="col s12 m6 l4 p-1">
                    <div class="card-panel teal">Fitness & Fatigue</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    let name_ref = create_node_ref::<Label>();
    let passwd_ref = create_node_ref::<Label>();
    view! {
        <ActionForm action=action>
            <div class="row">
                <div class="col s12">
                    <h1>"Log In"</h1>
                </div>
            </div>
            <div class="row">
                <div class="input-field col s12">
                    <input
                        type="text"
                        placeholder="User ID"
                        maxlength="32"
                        name="username"
                        id="username"
                        on:focusin=move |_| {
                            name_ref.get_untracked().unwrap().classes("active");
                        }

                        on:focusout=move |ev| {
                            if event_target_value(&ev).len() == 0 {
                                name_ref.get_untracked().unwrap().class_list().remove_1("active");
                            }
                        }
                    />

                    <label ref=name_ref for="username">
                        "User ID:"
                    </label>
                </div>
            </div>
            <div class="row">
                <div class="input-field col s12">
                    <input
                        type="password"
                        placeholder="Password"
                        name="password"
                        on:focusin=move |_| {
                            passwd_ref.get_untracked().unwrap().classes("active");
                        }

                        on:focusout=move |ev| {
                            if event_target_value(&ev).len() == 0 {
                                passwd_ref.get_untracked().unwrap().class_list().remove_1("active");
                            }
                        }
                    />

                    <label ref=passwd_ref for="password">
                        "Password:"
                    </label>
                </div>
            </div>
            <div class="row">
                <div class="col s12">
                    <label>
                        <input type="checkbox" name="remember"/>
                        <span>"Remember me?"</span>
                    </label>
                </div>
            </div>
            <div class="row">
                <div class="col s12">
                    <button type="submit" class="btn waves-effect waves-light">
                        "Log In"
                    </button>
                    <A href="/home/signup" class="waves-effect waves-light grey-darken-2 btn">
                        Signup
                    </A>
                </div>
            </div>
        </ActionForm>
    }
}

#[component]
fn Signup(action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    let name_ref = create_node_ref::<Label>();
    let passwd_ref = create_node_ref::<Label>();
    let passwd2_ref = create_node_ref::<Label>();
    view! {
        <ActionForm action=action>
            <div class="row">
                <div class="col s12">
                    <h1>"Sign Up"</h1>
                </div>
            </div>
            <div class="row">
                <div class="input-field col s12">
                    <input
                        type="text"
                        placeholder="User ID"
                        maxlength="32"
                        name="username"
                        id="username"
                        on:focusin=move |_| {
                            let _ = name_ref.get_untracked().unwrap().classes("active");
                        }

                        on:focusout=move |ev| {
                            if event_target_value(&ev).len() == 0 {
                                let _ = name_ref
                                    .get_untracked()
                                    .unwrap()
                                    .class_list()
                                    .remove_1("active");
                            }
                        }
                    />

                    <label ref=name_ref for="username">
                        "User ID:"
                    </label>
                </div>
            </div>
            <div class="row">
                <div class="input-field col s12">
                    <input
                        type="password"
                        placeholder="Password"
                        name="password"
                        id="password"
                        on:focusin=move |_| {
                            let _ = passwd_ref.get_untracked().unwrap().classes("active");
                        }

                        on:focusout=move |ev| {
                            if event_target_value(&ev).len() == 0 {
                                let _ = passwd_ref
                                    .get_untracked()
                                    .unwrap()
                                    .class_list()
                                    .remove_1("active");
                            }
                        }
                    />

                    <label ref=passwd_ref for="password">
                        "Password:"
                    </label>
                </div>
            </div>
            <div class="row">
                <div class="input-field col s12">
                    <input
                        type="password"
                        placeholder="Password again"
                        name="password_confirmation"
                        id="password_confirmation"
                        on:focusin=move |_| {
                            let _ = passwd2_ref.get_untracked().unwrap().classes("active");
                        }

                        on:focusout=move |ev| {
                            if event_target_value(&ev).len() == 0 {
                                let _ = passwd2_ref
                                    .get_untracked()
                                    .unwrap()
                                    .class_list()
                                    .remove_1("active");
                            }
                        }
                    />

                    <label ref=passwd2_ref for="password_confirmation">
                        "Confirm Password:"
                    </label>
                </div>
            </div>
            <div class="row">
                <div class="col s12">
                    <label>
                        <input type="checkbox" name="remember"/>
                        <span>"Remember me?"</span>
                    </label>
                </div>
            </div>

            <div class="row">
                <div class="col s12">
                    <button type="submit" class="btn waves-effect waves-light">
                        "Sign Up"
                    </button>
                    <A href="/home/login" class="btn waves-effect waves-light grey-darken-2">
                        Login
                    </A>
                </div>
            </div>
        </ActionForm>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <nav class="teal lighten-2">
            <div class="nav-wrapper">
                <a href="#" class="brand-logo">
                    Toedi
                </a>
                <ul id="nav-mobile" class="right hide-on-med-and-down">
                    <li>
                        <A href="/home/login" exact=true>
                            Login
                        </A>
                    </li>
                    <li>
                        <A href="/home/signup" exact=true>
                            Signup
                        </A>
                    </li>
                </ul>
            </div>
        </nav>
        <main>
            <div class="container">
                <Outlet/>
            </div>
        </main>
    }
}
#[component]
fn Landing() -> impl IntoView {
    view! {
        <div class="row">
            <div class="col s12">
                <h1>Welcome to Toedi</h1>
            </div>
        </div>
        <div class="row">
            <div class="col s4">
                <div class="card m-1 small">
                    <div class="card-content">
                        <span class="card-title">Track your Training</span>
                        <p>
                            Always stay on top of your training effort with easy to read charts and metrics
                        </p>
                    </div>
                </div>
            </div>
            <div class="col s4">

                <div class="card m-1 small">
                    <div class="card-content">
                        <span class="card-title">Based on Science</span>
                        <p>
                            "Based on newest scientific research, presented in a transparent way. We don't just make up numbers and we explain exactly how our metrics are calculated"
                        </p>
                    </div>
                </div>
            </div>
            <div class="col s4">

                <div class="card m-1 small">
                    <div class="card-content">
                        <span class="card-title">Open Source</span>
                        <p>Fully Open-Source code, made by users for users</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
