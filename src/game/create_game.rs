use sycamore::prelude::*;
use crate::client::create_client;

#[component]
pub async fn CreateGame<G: Html>(cx: Scope<'_>) -> View<G> {
    let body = create_game().await.unwrap_or_default();

    view! { cx,
        p {
            (body)
        }
    }
}

async fn create_game() -> Result<String, Box<dyn std::error::Error>>  {
    let client = create_client();

    let res = client
        .from("games")
        .insert(r#"[{}]"#)
        .execute()
        .await?;

    let body = res.text().await?;

    Ok(body)
}