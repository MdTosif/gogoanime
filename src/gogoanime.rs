use reqwest::{Client, Url};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

use scraper::selectable::Selectable;

#[derive(Serialize, Deserialize, Debug)]
struct SearchResultItem {
    // Renamed to avoid conflict with the built-in Result type
    title: String,
    id: String,
    url: String,
    image: String,
    release_date: String,
    is_dub: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    has_next_page: bool,
    current_page: i64,
    result: Vec<SearchResultItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeInfoResultItem {
    id: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeInfoResult {
    title: String,
    ep_start: String,
    ep_end: String,
    movie_id: String,
    alias: String,
    result: Vec<AnimeInfoResultItem>,
}

pub async fn search_anime(search: &str, page: i64) -> Result<SearchResult, Box<dyn Error>> {
    let client = Client::new();
    let mut url = Url::parse("https://anitaku.so/filter.html")?;
    url.query_pairs_mut()
        .append_pair("keyword", search)
        .append_pair("page", &page.to_string());

    let response = client.get(url).send().await?;

    let mut search_result = SearchResult {
        has_next_page: false,
        current_page: 0,
        result: vec![],
    };

    if response.status().is_success() {
        let data = response.text().await?;
        let document = Html::parse_document(&data);
        let search_result_selector = Selector::parse("div.last_episodes > ul > li")?;

        let has_next_page_selector =
            Selector::parse("div.anime_name.new_series > div > div > ul > li.selected")?;

        search_result.has_next_page = document
            .select(&has_next_page_selector)
            .next()
            .is_some_and(|x| x.next_siblings().count() > 0);

        search_result.current_page = page;

        for search_el in document.select(&search_result_selector) {
            let title_selector = Selector::parse("p.name > a")?;
            let title_el = search_el.select(&title_selector).next().unwrap();
            let title = title_el.text().next().unwrap();
            let url = title_el.attr("href").unwrap();
            let id = *title_el
                .attr("href")
                .unwrap()
                .split('/')
                .collect::<Vec<&str>>()
                .get(2)
                .unwrap();

            search_result.result.push(SearchResultItem {
                title: title.to_string(),
                id: id.to_string(),
                url: url.to_string(),
                image: "".to_string(),
                release_date: "".to_string(),
                is_dub: true,
            });
        }

        Ok(search_result)
    } else {
        Err("failed at api call".into())
    }
}

pub async fn get_anime_episodes(anime_id: &str) -> Result<AnimeInfoResult, Box<dyn Error>> {
    let client = Client::new();
    let url = Url::parse(&format!("https://anitaku.so/category/{}", anime_id))?;
    let response = client.get(url).send().await?;

    let mut anime_info_result = AnimeInfoResult {
        title: "".to_string(),
        ep_start: "".to_string(),
        ep_end: "".to_string(),
        movie_id: "".to_string(),
        alias: "".to_string(),
        result: vec![],
    };

    if response.status().is_success() {
        let data = response.text().await?;
        let document = Html::parse_document(&data);
        let ep_selector = Selector::parse("#episode_page > li")?;

        let episode = document.select(&ep_selector);
        let anchor_tag_selector = Selector::parse("a")?;
        anime_info_result.title = document
            .select(&Selector::parse("section.content_left > div.main_body > div:nth-child(2) > div.anime_info_body_bg > h1")?)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string();

        anime_info_result.ep_start = episode
            .clone()
            .next()
            .and_then(|a| a.select(&anchor_tag_selector).next())
            .and_then(|a| a.value().attr("ep_start"))
            .unwrap_or("Unknown")
            .to_string();
        anime_info_result.ep_end = episode
            .clone()
            .last()
            .and_then(|last| last.select(&anchor_tag_selector).next())
            .and_then(|a| a.value().attr("ep_end"))
            .unwrap_or("")
            .to_string();

        let movie_id_selector = Selector::parse("#movie_id")?;

        let movie_id_doc = document.select(&movie_id_selector);

        anime_info_result.movie_id = movie_id_doc
            .clone()
            .next()
            .and_then(|a| a.attr("value"))
            .unwrap_or("")
            .to_string();

        let alias_selector = Selector::parse("#alias_anime")?;

        let alias_doc = document.select(&alias_selector);

        anime_info_result.alias = alias_doc
            .clone()
            .next()
            .and_then(|a| a.attr("value"))
            .unwrap_or("")
            .to_string();

        get_episodes(&mut anime_info_result).await?;
        Ok(anime_info_result)
    } else {
        return Err("wasn't able to find anime".into());
    }
}

async fn get_episodes(
    anime_info_result: &mut AnimeInfoResult,
) -> Result<&AnimeInfoResult, Box<dyn Error>> {
    let anchor_tag_selector = Selector::parse("a")?;
    let client = Client::new();

    let mut params = HashMap::new();
    params.insert("ep_start", anime_info_result.ep_start.to_string());
    params.insert("ep_end", anime_info_result.ep_end.to_string());
    params.insert("id", anime_info_result.movie_id.to_string());
    params.insert("default_ep", "0".to_string());
    params.insert("alias", anime_info_result.alias.to_string());
    let resp = client
        .get("https://ajax.gogocdn.net/ajax/load-list-episode")
        .query(&params)
        .send()
        .await?;
    if resp.status().is_success() {
        let document = Html::parse_document(&resp.text().await?);
        let ep_selector = Selector::parse("#episode_related > li")?;
        let episode = document.select(&ep_selector);
        episode.for_each(|x| {
            let a_tag = x.select(&anchor_tag_selector).next().unwrap();
            let href = a_tag.attr("href").unwrap();
            let binding = href.split("/").collect::<Vec<&str>>();
            let id: &str = *binding.get(1).unwrap();
            anime_info_result.result.push(AnimeInfoResultItem {
                id: id.to_string(),
                url: href.to_string(),
            })
        });
        return Ok(anime_info_result);
    } else {
        return Err("wasn't able to get episodes".into());
    }
}


