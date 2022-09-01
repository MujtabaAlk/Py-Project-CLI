use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub info: Info,
}

#[derive(Deserialize)]
pub struct Info {
    pub name: String,
    pub version: String,
}
