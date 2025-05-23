use spotify_web_api::{
    AsyncSpotify,
    api::{AsyncQuery as _, artists::GetArtist},
    auth::scopes,
    model::Artist,
};
use std::{
    env,
    io::{self, Write},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;

    let mut spotify = AsyncSpotify::with_authorization_code_pkce(
        client_id,
        "http://127.0.0.1:8888/callback",
        scopes::user_details(),
    )?;

    let user_auth_url = spotify.user_authorization_url();

    println!("\nUser Authorization URL:\n\n{user_auth_url}");
    println!("\nPlease paste the full URL you were redirected to after authorization:\n");
    io::stdout().flush()?;

    let mut redirect_url = String::new();
    io::stdin().read_line(&mut redirect_url)?;

    let redirect_url = redirect_url.trim();

    spotify
        .request_token_from_redirect_url(redirect_url)
        .await?;

    let artist: Artist = GetArtist::from("0559tR6WyukLWH68JIGBuC")
        .query_async(&spotify)
        .await?;

    println!("\n{} -> {}\n", artist.name, artist.external_urls.spotify);

    Ok(())
}
