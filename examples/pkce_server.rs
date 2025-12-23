use anyhow::{Context, Result, bail};
use spotify_web_api::{
    Spotify,
    api::{Query as _, users::GetCurrentUserProfile},
    auth::scopes,
    model::CurrentUserProfile,
};
use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const APP_NAME: &str = "My App";
const PORT: u16 = 8888;

fn main() -> Result<()> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;
    let redirect_uri = format!("http://127.0.0.1:{PORT}/callback");

    let mut spotify =
        Spotify::with_authorization_code_pkce(client_id, redirect_uri, scopes::user_details())?;

    let user_auth_url = spotify.user_authorization_url();
    let callback_url = authenticate_user(&user_auth_url)?;

    spotify.request_token_from_redirect_url(&callback_url)?;

    let user_profile: CurrentUserProfile = GetCurrentUserProfile.query(&spotify)?;

    println!("{user_profile:#?}");

    Ok(())
}

fn authenticate_user(user_auth_url: &str) -> Result<String> {
    let listener = TcpListener::bind(format!("127.0.0.1:{PORT}"))?;

    if webbrowser::open(user_auth_url).is_err() {
        println!(
            "Please navigate to the following URL to authorize the application:\n\n{user_auth_url}",
        );
    }

    for stream in listener.incoming() {
        let stream = stream.with_context(|| "Failed to accept connection")?;
        if let Some(url) = handle_connection(stream) {
            return Ok(url);
        }
    }

    bail!("Failed to start server on port {PORT}");
}

fn handle_connection(mut stream: TcpStream) -> Option<String> {
    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_err() {
        return None;
    }

    let request = String::from_utf8_lossy(&buffer);

    if let Some(request_line) = request.lines().next() {
        let mut parts = request_line.split_whitespace();
        if let Some(method) = parts.next()
            && method == "GET"
            && let Some(url) = parts.next()
        {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><html><head><title>{APP_NAME}</title></head><body><h1>{APP_NAME}</h1><p>Authorization successful. You can now close this tab and return to the application.</p></body></html>"
            );

            let _ = stream.write_all(response.as_bytes());
            if let Err(e) = stream.flush() {
                eprintln!("Error flushing stream: {e:?}");
            }

            return Some(format!("http://127.0.0.1:{PORT}{url}"));
        }
    }

    let response = format!(
        "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><html><head><title>{APP_NAME}</title></head><body><h1>{APP_NAME}</h1><p>Bad request</p></body></html>"
    );

    let _ = stream.write_all(response.as_bytes());
    if let Err(e) = stream.flush() {
        eprintln!("Error flushing stream: {e:?}");
    }

    None
}
