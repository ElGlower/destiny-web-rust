use std::sync::Arc;
use yup_oauth2::authenticator::Authenticator;
use yup_oauth2::hyper_rustls::HttpsConnector;
use yup_oauth2::hyper::client::HttpConnector;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<Authenticator<HttpsConnector<HttpConnector>>>,
    pub project_id: String,
}
