


mod  gogoanime;

use gogoanime::gogoanime::{get_anime_episodes, search_anime};

#[tokio::main]
async fn main() {
    search_anime("girl", 3).await.unwrap();

    get_anime_episodes("high-score-girl").await.unwrap();
}
