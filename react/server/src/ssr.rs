use serde::Serialize;

const HTML_PLACEHOLDER: &str = "\"#INITIAL_STATE#\"";

#[derive(Serialize)]
pub struct ReactSSR {
    initialized: bool,
    msg: String,
}

#[derive(Serialize)]
pub struct PercyContributor {
    pub login: String,
    pub html_url: String,
}

pub struct ServerSideRender;

impl ServerSideRender {
    pub fn react(original: String) -> String {
        let app = ReactSSR {
            initialized: true,
            msg: "SSR!!!".to_string(),
        };
        original.replace(HTML_PLACEHOLDER, &format!("'{}'", serde_json::to_string(&app).unwrap()))
    }
}
