# Spotify Web API

[![Build Status](https://img.shields.io/github/actions/workflow/status/ry-sev/spotify_web_api/ci.yml?branch=main)](https://github.com/ry-sev/spotify_web_api/actions)

A wrapper for the [Spotify Web API](https://developer.spotify.com/documentation/web-api) written in Rust.

> Spotify Web API enables the creation of applications that can interact with Spotify's streaming service, such as retrieving content metadata, getting recommendations, creating and managing playlists, or controlling playback.

## Adding as a Dependency

You can add this to your project by running the command

```bash
cargo add spotify_web_api
```

## Examples

There are more examples in the [examples](https://github.com/ry-sev/spotify_web_api/tree/main/examples) folder.

### Client Credentials

```rust
use spotify_web_api::{
    api::{artists::GetArtist, Query as _},
    model::Artist,
    Spotify,
};

fn main() -> anyhow::Result<()> {
    let spotify = Spotify::with_client_credentials("client_id", "client_secret")?;

    spotify.request_token()?;

    let artist: Artist = GetArtist::from("0559tR6WyukLWH68JIGBuC").query(&spotify)?;

    println!("{artist:#?}");

    Ok(())
}
```

### Authorization Code with PKCE

```rust
use spotify_web_api::{
	api::{users::GetCurrentUserProfile, AsyncQuery as _},
    auth::scopes,
    model::CurrentUserProfile,
    AsyncSpotify,
};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut spotify = AsyncSpotify::with_authorization_code_pkce(
        "client_id",
        "http://127.0.0.1:8888/callback",
        scopes::all(),
    )?;

    let user_auth_url = spotify.user_authorization_url();

    println!("User Authorization URL:\n\n{user_auth_url}");
    println!("Please paste the full URL you were redirected to after authorization:\n");
    io::stdout().flush()?;

    let mut redirect_url = String::new();
    io::stdin().read_line(&mut redirect_url)?;
    let redirect_url = redirect_url.trim();

    spotify.request_token_from_redirect_url(redirect_url).await?;

    let user_profile: CurrentUserProfile = GetCurrentUserProfile.query_async(&spotify).await?;

    println!("{user_profile:#?}");

    Ok(())
}
```

### Which OAuth flow should I use? ([source](https://developer.spotify.com/documentation/web-api/concepts/authorization))

Choosing one flow over the rest depends on the application you are building:

- In scenarios where storing the client secret is not safe (e.g. desktop, mobile apps or JavaScript web apps running in the browser), you can use the [authorization code with PKCE](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow), as it provides protection against attacks where the authorization code may be intercepted.
- For some applications running on the backend, such as CLIs or daemons, the system authenticates and authorizes the app rather than a user. For these scenarios, [Client credentials](https://developer.spotify.com/documentation/web-api/tutorials/client-credentials-flow) is the typical choice. This flow does not include user authorization, so only endpoints that do not request user information (e.g. user profile data) can be accessed.

The following table summarizes the flows' behaviors:

| Flow | Access User Resources | Requires Secret Key (Server-Side) | Access Token Refresh |
| :--- | :--- | :--- | :--- |
| Authorization code with PKCE | Yes | No | Yes |
| Client credentials | No | Yes | No |

## API

Supported endpoints are organized under the [`api`](https://github.com/ry-sev/spotify_web_api/blob/main/src/api.rs) module. To interact with an endpoint, you can use either the `Query` or `AsyncQuery` [traits](https://github.com/ry-sev/spotify_web_api/blob/main/src/api/query.rs).
- `Query` is designed for blocking code, making it ideal for synchronous workflows or environments where asynchronous execution is unnecessary or not supported. Opt for this when simplicity is key, such as in single-threaded environments or scripts where blocking is acceptable.
- `AsyncQuery` is intended for asynchronous code and integrates seamlessly with an asynchronous runtime of your choice, such as `Tokio` or `async-std`. This approach is particularly useful when working in environments that benefit from non-blocking operations. Use this trait when building applications that require high concurrency or when interacting with other asynchronous code.

There are additional helpers to handle different cases:
- [`api::ignore`](https://github.com/ry-sev/spotify_web_api/blob/main/src/api/ignore.rs): Ignore the Spotify response (useful for POST or PUT endpoints).
- [`api::paged`](https://github.com/ry-sev/spotify_web_api/blob/main/src/api/paged/all_at_once.rs): Fetch results that are paginated.
- [`api::raw`](https://github.com/ry-sev/spotify_web_api/blob/main/src/api/raw.rs): Return the raw data from Spotify instead of deserializing into a structure.

You're not restricted to the predefined endpoints; you can define your own by implementing the [`Endpoint`](https://github.com/ry-sev/spotify_web_api/blob/main/src/api/endpoint.rs) trait. [See example](https://github.com/ry-sev/spotify_web_api/blob/main/examples/creds_custom_endpoint.rs).

All endpoints return data types chosen by the caller, provided these types implement `serde`'s `Deserialize` trait. The library offers predefined structs in the [`model`](https://github.com/ry-sev/spotify_web_api/blob/main/src/model.rs) module, but you are free to use your own structs by implementing the `Deserialize` trait. This flexibility is particularly useful when a custom data structure better suits the your needs or when avoiding the overhead of deserializing the entire response is desirable. [See example](https://github.com/ry-sev/spotify_web_api/blob/main/examples/creds_custom_model.rs).

## Feature Flags

A set of [feature flags](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section) are available to customize the data models. **These are enabled by default**, but you can disable them to reduce the size of the compiled library or to avoid unnecessary data in your application.
- `markets` - Enables the `available_markets` field in various models, such as [`Track`](https://github.com/ry-sev/spotify_web_api/blob/main/src/model/tracks.rs#L41). This field contains a list of markets where the content is available.
- `page_items` - Enables the field in various models that contain paginated items, such as the `tracks` field in [`Playlist`](https://github.com/ry-sev/spotify_web_api/blob/main/src/model/playlists.rs#L49).

## Implemented Endpoints

Format: `[x]` `[Title]` `[Method]` `[Endpoint]` `[Spotify Docs]`

### Albums

- [x] Get Album `GET` `/albums/{id}` [get-an-album](https://developer.spotify.com/documentation/web-api/reference/get-an-album)
- [x] Get Several Albums `GET` `/albums` [get-multiple-albums](https://developer.spotify.com/documentation/web-api/reference/get-multiple-albums)
- [x] Get Album Tracks `GET` `/albums/{id}/tracks` [get-an-albums-tracks](https://developer.spotify.com/documentation/web-api/reference/get-an-albums-tracks)
- [X] Get User's Saved Albums `GET` `me/albums` [get-users-saved-albums](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-albums)
- [X] Save Albums for Current User `PUT` `me/albums` [save-albums-user](https://developer.spotify.com/documentation/web-api/reference/save-albums-user)
- [X] Remove User's Saved Albums `DELETE` `me/albums` [remove-albums-user](https://developer.spotify.com/documentation/web-api/reference/remove-albums-user)
- [x] Check User's Saved Albums `GET` `me/albums/contains` [check-users-saved-albums](https://developer.spotify.com/documentation/web-api/reference/check-users-saved-albums)
- [X] Get New Releases `GET` `/browse/new-releases` [get-new-releases](https://developer.spotify.com/documentation/web-api/reference/get-new-releases)

### Artists

- [x] Get Artist `GET` `/artists/{id}` [get-an-artist](https://developer.spotify.com/documentation/web-api/reference/get-an-artist)
- [X] Get Several Artists `GET` `/artists` [get-multiple-artists](https://developer.spotify.com/documentation/web-api/reference/get-multiple-artists)
- [X] Get Artist's Albums `GET` `/artists/{id}/albums` [get-an-artists-albums](https://developer.spotify.com/documentation/web-api/reference/get-an-artists-albums)
- [X] Get Artist's Top Tracks `GET` `/artists/{id}/top-tracks` [get-an-artists-top-tracks](https://developer.spotify.com/documentation/web-api/reference/get-an-artists-top-tracks)

### Audiobooks

- [X] Get an Audiobook `GET` `/audiobooks/{id}` [get-an-audiobook](https://developer.spotify.com/documentation/web-api/reference/get-an-audiobook)
- [X] Get Several Audiobooks `GET` `/audiobooks` [get-multiple-audiobooks](https://developer.spotify.com/documentation/web-api/reference/get-multiple-audiobooks)
- [X] Get Audiobook Chapters `GET` `/audiobooks/{id}/chapters` [get-audiobook-chapters](https://developer.spotify.com/documentation/web-api/reference/get-audiobook-chapters)
- [X] Get User's Saved Audiobooks `GET` `me/audiobooks` [get-users-saved-audiobooks](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-audiobooks)
- [X] Save Audiobooks for Current User `PUT` `me/audiobooks` [save-audiobooks-user](https://developer.spotify.com/documentation/web-api/reference/save-audiobooks-user)
- [X] Remove User's Saved Audiobooks `DELETE` `me/audiobooks` [remove-audiobooks-user](https://developer.spotify.com/documentation/web-api/reference/remove-audiobooks-user)
- [X] Check User's Saved Audiobooks `GET` `me/audiobooks/contains` [check-users-saved-audiobooks](https://developer.spotify.com/documentation/web-api/reference/check-users-saved-audiobooks)

### Categories

- [X] Get Several Browse Categories `GET` `/browse/categories` [get-categories](https://developer.spotify.com/documentation/web-api/reference/get-categories)
- [X] Get Single Browse Category `GET` `/browse/categories/{category_id}` [get-a-category](https://developer.spotify.com/documentation/web-api/reference/get-a-category)

### Chapters

- [X] Get a Chapter `GET` `/chapters/{id}` [get-a-chapter](https://developer.spotify.com/documentation/web-api/reference/get-a-chapter)
- [X] Get Several Chapters `GET` `/chapters` [get-several-chapters](https://developer.spotify.com/documentation/web-api/reference/get-several-chapters)

### Episodes

- [X] Get Episode `GET` `/episodes/{id}` [get-an-episode](https://developer.spotify.com/documentation/web-api/reference/get-an-episode)
- [X] Get Several Episodes `GET` `/episodes` [get-multiple-episodes](https://developer.spotify.com/documentation/web-api/reference/get-multiple-episodes)
- [X] Get User's Saved Episodes `GET` `me/episodes` [get-users-saved-episodes](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-episodes)
- [X] Save Episodes for Current User `PUT` `me/episodes` [save-episodes-user](https://developer.spotify.com/documentation/web-api/reference/save-episodes-user)
- [X] Remove User's Saved Episodes `DELETE` `me/episodes` [remove-episodes-user](https://developer.spotify.com/documentation/web-api/reference/remove-episodes-user)
- [X] Check User's Saved Episodes `GET` `me/episodes/contains` [check-users-saved-episodes](https://developer.spotify.com/documentation/web-api/reference/check-users-saved-episodes)

### Genres

- [X] Get Available Genre Seeds `GET` `/recommendations/available-genre-seeds` [get-recommendation-genres](https://developer.spotify.com/documentation/web-api/reference/get-recommendation-genres)

### Markets

- [X] Get Available Markets `GET` `/markets` [get-available-markets](https://developer.spotify.com/documentation/web-api/reference/get-available-markets)

### Player

- [X] Get Playback State `GET` `/me/player` [get-information-about-the-users-current-playback](https://developer.spotify.com/documentation/web-api/reference/get-information-about-the-users-current-playback)
- [X] Transfer Playback `PUT` `/me/player` [transfer-a-users-playback](https://developer.spotify.com/documentation/web-api/reference/transfer-a-users-playback)
- [X] Get Available Devices `GET` `/me/player/devices` [get-a-users-available-devices](https://developer.spotify.com/documentation/web-api/reference/get-a-users-available-devices)
- [X] Get Currently Playing Track `GET` `/me/player/currently-playing` [get-the-users-currently-playing-track](https://developer.spotify.com/documentation/web-api/reference/get-the-users-currently-playing-track)
- [X] Start/Resume Playback `PUT` `/me/player/play` [start-a-users-playback](https://developer.spotify.com/documentation/web-api/reference/start-a-users-playback)
- [X] Pause Playback `PUT` `/me/player/pause` [pause-a-users-playback](https://developer.spotify.com/documentation/web-api/reference/pause-a-users-playback)
- [X] Skip To Next `POST` `/me/player/next` [skip-users-playback-to-next-track](https://developer.spotify.com/documentation/web-api/reference/skip-users-playback-to-next-track)
- [X] Skip To Previous `POST` `/me/player/previous` [skip-users-playback-to-previous-track](https://developer.spotify.com/documentation/web-api/reference/skip-users-playback-to-previous-track)
- [X] Seek To Position `PUT` `/me/player/seek` [seek-to-position-in-currently-playing-track](https://developer.spotify.com/documentation/web-api/reference/seek-to-position-in-currently-playing-track)
- [X] Set Repeat Mode `PUT` `/me/player/repeat` [set-repeat-mode-on-users-playback](https://developer.spotify.com/documentation/web-api/reference/set-repeat-mode-on-users-playback)
- [X] Set Playback Volume `PUT` `/me/player/volume` [set-volume-for-users-playback](https://developer.spotify.com/documentation/web-api/reference/set-volume-for-users-playback)
- [X] Toggle Playback Shuffle `PUT` `/me/player/shuffle` [toggle-shuffle-for-users-playback](https://developer.spotify.com/documentation/web-api/reference/toggle-shuffle-for-users-playback)
- [X] Get Recently Played Tracks `GET` `/me/player/recently-played` [get-recently-played](https://developer.spotify.com/documentation/web-api/reference/get-recently-played)
- [X] Get the User's Queue `GET` `/me/player/queue` [get-queue](https://developer.spotify.com/documentation/web-api/reference/get-queue)
- [X] Add Item to Playback Queue `POST` `/me/player/queue` [add-to-queue](https://developer.spotify.com/documentation/web-api/reference/add-to-queue)

### Playlists

- [X] Get Playlist `GET` `/playlists/{playlist_id}` [get-playlist](https://developer.spotify.com/documentation/web-api/reference/get-playlist)
- [X] Change Playlist Details `PUT` `/playlists/{playlist_id}` [change-playlist-details](https://developer.spotify.com/documentation/web-api/reference/change-playlist-details)
- [X] Get Playlist Items `GET` `/playlists/{playlist_id}/tracks` [get-playlists-tracks](https://developer.spotify.com/documentation/web-api/reference/get-playlists-tracks)
- [X] Update Playlist Items `PUT` `/playlists/{playlist_id}/tracks` [reorder-or-replace-playlists-tracks](https://developer.spotify.com/documentation/web-api/reference/reorder-or-replace-playlists-tracks)
- [X] Add Items to Playlist `POST` `/playlists/{playlist_id}/tracks` [add-tracks-to-playlist](https://developer.spotify.com/documentation/web-api/reference/add-tracks-to-playlist)
- [X] Remove Playlist Items `DELETE` `/playlists/{playlist_id}/tracks` [remove-tracks-playlist](https://developer.spotify.com/documentation/web-api/reference/remove-tracks-playlist)
- [X] Get Current User's Playlists `GET` `/me/playlists` [get-a-list-of-current-users-playlists](https://developer.spotify.com/documentation/web-api/reference/get-a-list-of-current-users-playlists)
- [X] Get User's Playlists `GET` `/users/{user_id}/playlists` [get-list-users-playlists](https://developer.spotify.com/documentation/web-api/reference/get-list-users-playlists)
- [X] Create Playlist `POST` `/users/{user_id}/playlists` [create-playlist](https://developer.spotify.com/documentation/web-api/reference/create-playlist)
- [X] Get Playlist Cover Image `GET` `/playlists/{playlist_id}/images` [get-playlist-cover](https://developer.spotify.com/documentation/web-api/reference/get-playlist-cover)
- [ ] Add Custom Playlist Cover Image `PUT` `/playlists/{playlist_id}/images` [upload-custom-playlist-cover](https://developer.spotify.com/documentation/web-api/reference/upload-custom-playlist-cover)

### Search

- [X] Search for Item `GET` `/search` [search](https://developer.spotify.com/documentation/web-api/reference/search)

### Shows

- [X] Get Show `GET` `/shows/{id}` [get-a-show](https://developer.spotify.com/documentation/web-api/reference/get-a-show)
- [X] Get Several Shows `GET` `/shows` [get-multiple-shows](https://developer.spotify.com/documentation/web-api/reference/get-multiple-shows)
- [X] Get Show Episodes `GET` `/shows/{id}/episodes` [get-a-shows-episodes](https://developer.spotify.com/documentation/web-api/reference/get-a-shows-episodes)
- [X] Get User's Saved Shows `GET` `me/shows` [get-users-saved-shows](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-shows)
- [X] Save Shows for Current User `PUT` `me/shows` [save-shows-user](https://developer.spotify.com/documentation/web-api/reference/save-shows-user)
- [X] Remove User's Saved Shows `DELETE` `me/shows` [remove-shows-user](https://developer.spotify.com/documentation/web-api/reference/remove-shows-user)
- [X] Check User's Saved Shows `GET` `me/shows/contains` [check-users-saved-shows](https://developer.spotify.com/documentation/web-api/reference/check-users-saved-shows)

### Tracks

- [x] Get Track `GET` `/tracks/{id}` [get-track](https://developer.spotify.com/documentation/web-api/reference/get-track)
- [X] Get Several Tracks `GET` `/tracks` [get-several-tracks](https://developer.spotify.com/documentation/web-api/reference/get-several-tracks)
- [X] Get User's Saved Tracks `GET` `me/tracks` [get-users-saved-tracks](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-tracks)
- [X] Save Tracks for Current User `PUT` `me/tracks` [save-tracks-user](https://developer.spotify.com/documentation/web-api/reference/save-tracks-user)
- [X] Remove User's Saved Tracks `DELETE` `me/tracks` [remove-tracks-user](https://developer.spotify.com/documentation/web-api/reference/remove-tracks-user)
- [X] Check User's Saved Tracks `GET` `me/tracks/contains` [check-users-saved-tracks](https://developer.spotify.com/documentation/web-api/reference/check-users-saved-tracks)

### Users

- [x] Get Current User's Profile `GET` `/me` [get-current-users-profile](https://developer.spotify.com/documentation/web-api/reference/get-current-users-profile)
- [X] Get User's Top Items `GET` `/me/top/{type}` [get-users-top-artists-and-tracks](https://developer.spotify.com/documentation/web-api/reference/get-users-top-artists-and-tracks)
- [X] Get User's Profile `GET` `/users/{user_id}` [get-users-profile](https://developer.spotify.com/documentation/web-api/reference/get-users-profile)
- [X] Follow Playlist `PUT` `/playlists/{playlist_id}/followers` [follow-playlist](https://developer.spotify.com/documentation/web-api/reference/follow-playlist)
- [X] Unfollow Playlist `DELETE` `/playlists/{playlist_id}/followers` [unfollow-playlist](https://developer.spotify.com/documentation/web-api/reference/unfollow-playlist)
- [X] Get Followed Artists `GET` `/me/following` [get-followed](https://developer.spotify.com/documentation/web-api/reference/get-followed)
- [X] Follow Artists or Users `PUT` `/me/following` [follow-artists-users](https://developer.spotify.com/documentation/web-api/reference/follow-artists-users)
- [X] Unfollow Artists or Users `DELETE` `/me/following` [unfollow-artists-users](https://developer.spotify.com/documentation/web-api/reference/unfollow-artists-users)
- [X] Check If User Follows Artists or Users `GET` `/me/following/contains` [check-current-user-follows](https://developer.spotify.com/documentation/web-api/reference/check-current-user-follows)
- [X] Check if Current User Follows Playlist `GET` `/playlists/{playlist_id}/followers/contains` [check-if-user-follows-playlist](https://developer.spotify.com/documentation/web-api/reference/check-if-user-follows-playlist)


#### License

<sup>
Licensed under the <a href="LICENSE">MIT license</a>.
</sup>
