use std::vec;

use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use transmission_rpc::{
    types::{Torrent, TorrentGetField},
    TransClient,
};
use url::Url;

#[server(PortTest, "/api", "GetJson")]
pub async fn port_test() -> Result<String, ServerFnError> {
    logging::log!("Executing port test");
    let mut client = TransClient::new(
        Url::parse("http://plutonium:9091/transmission/rpc").expect("Couldn't parse url"),
    );
    let response = client.port_test().await;
    match response {
        Ok(_) => logging::log!("Yay!"),
        Err(_) => logging::error!("Oh no!"),
    }
    println!("{:?}", response);
    Ok(format!("Response ok? {:?}", response.is_ok()))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyTorrent {
    pub id: Option<i64>,
    pub name: Option<String>,
}

impl Into<MyTorrent> for &Torrent {
    fn into(self) -> MyTorrent {
        MyTorrent {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

#[server(GetTorrents, "/api", "GetJson")]
pub async fn get_torrents() -> Result<Vec<MyTorrent>, ServerFnError<String>> {
    logging::log!("Getting torrents");
    let mut client = TransClient::new(
        Url::parse("http://plutonium:9091/transmission/rpc").expect("Couldn't parse url"),
    );
    let response = client
        .torrent_get(Some(vec![TorrentGetField::Id, TorrentGetField::Name]), None)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    println!("{:?}", response.arguments.torrents.first());
    // match response {
    //     Ok(_) => logging::log!("Yay!"),
    //     Err(_) => logging::error!("Oh no!"),
    // }
    // println!("{:?}", response);
    Ok(response
        .arguments
        .torrents
        .iter()
        .map(|t| t.into())
        .collect())
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/transmission-leptos.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

use stylers::style_str;

#[component]
fn TorrentCount() -> impl IntoView {
    let (filter, set_filter) = create_signal("".to_string());

    let torrents = create_resource(|| (), |_| async move { get_torrents().await });

    let sorted = move || {
        torrents()
            .map(|r| match r {
                Ok(v) => {
                    let mut v = v.clone();
                    v.sort_by(|a, b| a.name.partial_cmp(&b.name).expect("Couldn't sort"));
                    v
                }
                Err(_) => vec![],
            })
            .unwrap_or(vec![])
    };

    let filtered = move || {
        sorted()
            .into_iter()
            .filter(|t| {
                t.name
                    .clone()
                    .unwrap_or("".to_string())
                    .to_lowercase()
                    .contains(&filter().to_lowercase())
            })
            .collect::<Vec<MyTorrent>>()
    };

    let (class_name, style_val) = style_str! {
        section {

        }
        section div {
            padding: var(--size-gap);
            text-align: left;
            border-bottom: 1px solid gray;
        }
    };

    view! { class=class_name,
        <Suspense fallback=move || view! { "Loading..." }>
            <style>{style_val}</style>
            <input on:input=move |ev| set_filter(event_target_value(&ev)) prop:value=filter/>
            <section>
                <For each=filtered key=move |t| (t.name.clone(), class_name) let:t>
                    <div>{t.name.clone()}</div>
                </For>
            </section>
        </Suspense>
    }

    // view! { <button on:click=on_click>"Click Me: " {move || torrent_count}</button> }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! { <TorrentCount/> }
}
