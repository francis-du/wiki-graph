use log::info;
use tide::http::mime;
use tide::{Response, StatusCode};

use crate::handler::get_handle;

/// API struct
#[derive(Debug, Clone)]
pub struct API {
    host: String,
    port: String,
}

impl Default for API {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: "3690".to_string(),
        }
    }
}

impl API {
    pub fn new() -> Self {
        API::default()
    }

    /// setting host
    pub fn host(&mut self, host: String) -> &mut API {
        self.host = host;
        self
    }

    /// setting port
    pub fn port(&mut self, port: String) -> &mut API {
        self.port = port;
        self
    }

    /// start a api service
    pub async fn start(&mut self) -> tide::Result<()> {
        info!("Start a API service");
        let mut api = tide::new();

        // service router bind
        api.at("/").get(|_| async {
            let response = Response::builder(StatusCode::Ok)
                .body(
                    "<div>Wiki Graph api docs link: \
                    <a href=\"https://wiki-graph.francis.run/\"> \
                    https://wiki-graph.fracnis.run</a></div>",
                )
                .content_type(mime::HTML)
                .build();
            Ok(response)
        });
        api.at("/api/v1/search").get(get_handle::search);

        let listener = format!("{}:{}", self.host, self.port);
        api.listen(listener).await?;
        Ok(())
    }
}
