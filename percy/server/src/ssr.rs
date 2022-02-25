use isomorphic_app::App;

const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

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
