use log::info;
use tide::http::mime;
use tide::{Response, StatusCode};

use crate::handler::get_handle;

/// APP struct
#[derive(Debug, Clone)]
pub struct APP {
    host: String,
    port: String,
}

impl Default for APP {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: "3690".to_string(),
        }
    }
}

impl APP {
    pub fn new() -> Self {
        APP::default()
    }

    /// setting host
    pub fn host(&mut self, host: String) -> &mut APP {
        self.host = host;
        self
    }

    /// setting port
    pub fn port(&mut self, port: String) -> &mut APP {
        self.port = port;
        self
    }

    /// start a app service
    pub async fn start(&mut self) -> tide::Result<()> {
        info!("Start a APP service");
        let mut app = tide::new();

        // service router bind
        app.at("/").get(|_| async {
            let html_contents = include_str!("static/index.html");
            let response = Response::builder(StatusCode::Ok)
                .body(html_contents)
                .content_type(mime::HTML)
                .build();
            Ok(response)
        });
        app.at("/search").get(get_handle::search);

        let listener = format!("{}:{}", self.host, self.port);
        app.listen(listener).await?;
        Ok(())
    }
}
