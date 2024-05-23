// extern crate gogoanime;

pub mod gogoanime {
    use std::string;

    use reqwest::Client;
    use reqwest::Url;
    use scraper::element_ref;
    use scraper::selectable::Selectable;
    use scraper::ElementRef;
    use scraper::{Html, Selector};

    pub async fn search_anime(search: &str, page: i64) -> Result<(), ()> {
        let client = Client::new();
        // let url = "?keyword=frieren";

        let mut url = Url::parse("https://anitaku.so/filter.html").unwrap();

        url.query_pairs_mut()
            .append_pair("keyword", search)
            .append_pair("page", &page.to_string());

        let response = client.get(url).send().await.unwrap();

        // Handle the response
        if response.status().is_success() {
            let data = response.text().await.unwrap();

            // println!("Request was successful! {:?}", data);
            let document = Html::parse_document(&data);

            let search_result_selector = Selector::parse("div.last_episodes > ul > li").unwrap();

            for search_result in document.select(&search_result_selector) {
                let title_selector = Selector::parse("p.name > a").unwrap();
                let mut title = search_result.select(&title_selector);
                match title.next() {
                    Some(first) => {
                        println!("The first element is: {:?}", first.text().next().unwrap());
                        println!(
                            "The id of the anime {:?}",
                            first
                                .attr("href")
                                .unwrap()
                                .split("/")
                                .collect::<Vec<&str>>()
                                .get(2)
                                .unwrap()
                        )
                    }
                    None => println!("The iterator is empty"),
                }
            }
        } else {
            println!("Request failed with status: {}", response.status());
        }

        Ok(())
    }

    pub async fn get_anime_episodes(anime_id: &str) -> Result<(), ()> {
        let client = Client::new();
        // let url = "?keyword=frieren";

        let mut url = Url::parse(&format!("https://anitaku.so/category/{}", anime_id)).unwrap();

        let response = client.get(url).send().await.unwrap();

        // Handle the response
        if response.status().is_success() {
            let data = response.text().await.unwrap();

            // println!("Request was successful! {:?}", data);
            let document = Html::parse_document(&data);

            let ep_selector = Selector::parse("#episode_page > li").unwrap();

            let mut episode = document.select(&ep_selector);
            let episode2 = document.select(&ep_selector);
            let anchor_tag_selector = Selector::parse("a").unwrap();
            // println!("{:?}", episode.next().unwrap().text());
            let ep_start = episode.next().unwrap().select(&anchor_tag_selector).next().unwrap().attr("ep_start").unwrap().to_string();
            let ep_end = episode2.last().unwrap().select(&anchor_tag_selector).next().unwrap().attr("ep_end").unwrap();
            println!("{} - {}", ep_start, ep_end);

        } else {
            println!("Request failed with status: {}", response.status());
        }


        Ok(())
    }

fn get_el<'a>(mut title: scraper::element_ref::Select<'a, 'a>) -> Result<ElementRef<'a>,()>{
        match title.next() {
            Some(first) => {
                return Ok(first); 
            }
            None => Err(()),
        }
    }
}
