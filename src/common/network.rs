use std::io::Read;
use std::time::Duration;

use log::info;
use reqwest::Proxy;
use serde_json;
use serde_json::Value;
use wikipedia::http::HttpClient;

/// ProxyClient
pub struct ProxyClient {
    user_agent: String,
}

impl Default for ProxyClient {
    fn default() -> Self {
        ProxyClient {
            user_agent: "wikipedia (https://github.com/seppo0010/wikipedia-rs)".to_owned(),
        }
    }
}

impl HttpClient for ProxyClient {
    fn user_agent(&mut self, user_agent: String) {
        self.user_agent = user_agent;
    }

    fn get<'a, I>(&self, base_url: &str, args: I) -> Result<String, wikipedia::http::Error>
    where
        I: Iterator<Item = (&'a str, &'a str)>,
    {
        let client = match std::env::var("PROXY") {
            Ok(proxy_url) => {
                info!("proxy with {}", &proxy_url);
                let client;
                if proxy_url.contains("https://") {
                    client = reqwest::Client::builder()
                        .proxy(Proxy::https(proxy_url.as_str()).unwrap())
                        .timeout(Duration::from_secs(10))
                        .build()?;
                } else {
                    client = reqwest::Client::builder()
                        .proxy(Proxy::http(proxy_url.as_str()).unwrap())
                        .timeout(Duration::from_secs(10))
                        .build()?;
                }
                client
            }
            Err(_) => reqwest::Client::new(),
        };

        let url = reqwest::Url::parse_with_params(base_url, args)?;
        let mut response = client
            .get(url)
            .header(reqwest::header::USER_AGENT, self.user_agent.clone())
            .send()?;

        let mut response_str = String::new();
        response.read_to_string(&mut response_str)?;
        Ok(response_str)
    }
}

pub fn set_proxy() -> Result<(), reqwest::Error> {
    let json: Value = reqwest::get("http://118.24.52.95/get")?.json()?;

    match json["proxy"].as_str() {
        None => {}
        Some(proxy) => {
            std::env::set_var("PROXY", format!("http:://{}", &proxy));
            info!("Setup proxy http://{}", proxy);
        }
    };

    Ok(())
}
