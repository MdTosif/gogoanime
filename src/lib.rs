
use wasm_bindgen::prelude::*;
mod  gogoanime;
use gogoanime::gogoanime::search_anime;


// Initialization function called from JavaScript
#[wasm_bindgen]
pub async fn search(search: String) -> String {
    search_anime(&search, 1).await.unwrap();
    return "ok".to_string();
}
