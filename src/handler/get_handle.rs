use log::{error, info};
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use tide::Request;
use whatlang::{detect, Lang};
use wikipedia::Wikipedia;

use crate::common::network::ProxyClient;

#[derive(Debug, Deserialize)]
struct Search {
    words: String,
    limit: Option<u32>,
}

#[derive(Debug, Serialize, Clone)]
struct WikiGraphInfo {
    id: u32,
    title: String,
    value: u32,
    summary: String,
    children: Vec<WikiGraphInfo>,
}

impl WikiGraphInfo {
    fn new(
        id: u32,
        title: String,
        value: u32,
        summary: String,
        children: Vec<WikiGraphInfo>,
    ) -> Self {
        Self {
            id,
            title,
            value,
            summary,
            children,
        }
    }
}

// search wiki
pub async fn search(req: Request<()>) -> tide::Result {
    let mut response: Vec<WikiGraphInfo> = vec![];

    if req.query::<Search>().is_ok() {
        let query: Search = req.query()?;
        let mut wiki_info =
            WikiGraphInfo::new(0, query.words.clone(), 3, query.words.clone(), vec![]);

        let mut wiki = Wikipedia::<ProxyClient>::default();
        if query.limit.is_some() {
            wiki.search_results = query.limit.unwrap();
        } else {
            wiki.search_results = 10
        }

        let language = match detect(query.words.as_str()) {
            None => "en",
            Some(lang) => match lang.lang() {
                Lang::Eng => "en",
                Lang::Cmn => "zh",
                Lang::Spa => "es",
                Lang::Ita => "it",
                Lang::Fra => "fr",
                _ => "en",
            },
        };

        let words = query.words.as_str();
        wiki.set_base_url(format!("https://{}.wikipedia.org/w/api.php", language).as_str());
        info!("Base URL => {}", wiki.base_url());
        match wiki.search(words) {
            Ok(results) => {
                for res in results {
                    let page = wiki.page_from_title(res.to_string());
                    let title = page.get_title().unwrap();
                    let links_iter = page.get_links().unwrap();

                    let id = match page.get_pageid().unwrap().parse::<u32>() {
                        Ok(i) => i,
                        Err(_) => thread_rng().gen()
                    };

                    let mut children =
                        WikiGraphInfo::new(id, title, 2, page.get_summary().unwrap(), vec![]);

                    let mut index: u32 = 0;
                    for x in links_iter {
                        index += 1;
                        if index > 20 {
                            break;
                        }
                        let link_title = x.title;
                        let wiki_link = WikiGraphInfo::new(
                            index,
                            link_title.clone(),
                            1,
                            link_title.to_string(),
                            vec![],
                        );
                        children.children.push(wiki_link);
                    }
                    wiki_info.children.push(children.clone());
                }
            }
            Err(err) => {
                error!("search keyword {} wiki error - {:?}", words.clone(), err);
            }
        };
        response.push(wiki_info);
    }
    Ok(serde_json::json!(response).into())
}
