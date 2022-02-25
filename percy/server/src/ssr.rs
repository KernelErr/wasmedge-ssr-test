use serde::Serialize;
use isomorphic_app::App;

const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

#[derive(Serialize)]
pub struct PercySSR {
    click_count: u32,
    path: String,
    contributors: Option<Vec<PercyContributor>>,
    has_initiated_contributors_download: bool,
}

impl PercySSR {
    pub fn new(init: Option<u32>, path: String) -> Self {
        Self {
            click_count: init.unwrap_or(1001),
            path: path.clone(),
            contributors: None,
            has_initiated_contributors_download: false,
        }
    }

    pub fn times(&self) -> u32 {
        self.click_count
    }
}

#[derive(Serialize)]
pub struct PercyContributor {
    pub login: String,
    pub html_url: String,
}

pub struct ServerSideRender;

impl ServerSideRender {
    pub fn percy(original: String, path: String, init: Option<u32>) -> String {
        let app = App::new(init.unwrap_or(1001), path);
        let state = app.store.borrow();

        let mut res = original.replace(HTML_PLACEHOLDER, &app.render().to_string());
        res = res.replace(STATE_PLACEHOLDER, &state.to_json());
        res
    }
}
