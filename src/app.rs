use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use transmission_rpc::{
    types::{Torrent, TorrentGetField, Torrents},
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
pub async fn get_torrents() -> Result<Vec<MyTorrent>, ServerFnError> {
    logging::log!("Getting torrents");
    let mut client = TransClient::new(
        Url::parse("http://plutonium:9091/transmission/rpc").expect("Couldn't parse url"),
    );
    let response = client
        .torrent_get(Some(vec![TorrentGetField::Id, TorrentGetField::Name]), None)
        .await;
    match response {
        Ok(_) => logging::log!("Yay!"),
        Err(_) => logging::error!("Oh no!"),
    }
    // println!("{:?}", response);
    Ok(response
        .unwrap()
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
        <Stylesheet id="leptos" href="/pkg/transmission-leptos-axum.css"/>

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

#[component]
fn TorrentCount() -> impl IntoView {
    let torrent_count = create_resource(|| (), |_| async move { get_torrents().await });

    view! {
        <Suspense fallback=move || view! { "Loading..." }>
            <p>Count: {move || torrent_count.get().map(|c| c.unwrap().len())}</p>
        </Suspense>
    }

    // view! { <button on:click=on_click>"Click Me: " {move || torrent_count}</button> }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <h1>"Welcome to Leptos!"</h1>

        <TorrentCount/>
    }
}
