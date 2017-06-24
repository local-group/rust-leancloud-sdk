
extern crate reqwest;
extern crate serde_json;


use serde_json::value::Value;

// URL Template: "https://{domain}/{version}/"
const DOMAIN: &'static str = "api.leancloud.cn";
const VERSION: &'static str = "1.1";


pub struct Client {
    app_id: String,
    app_key: String,
    // Use master key
    master: bool,
    master_key: Option<String>,
}

pub enum ApiCategory {
    CObject,
    CUser,
    CRole,
}

impl Client {
    pub fn new(app_id: &str, app_key: &str) -> Client {
        Client {
            app_id: app_id.to_owned(),
            app_key: app_key.to_owned(),
            master: false,
            master_key: None,
        }
    }

    pub fn object_create(&self) -> Object { Object{} }
    pub fn object_update(&self) -> Object { Object{} }
    pub fn object_delete(&self) -> bool { true }
    pub fn object_get(&self) -> Object { Object{} }
    pub fn object_query(&self) -> Vec<Object> { vec![Object{}] }
    pub fn object_scan(&self) -> Vec<Object> { vec![Object{}] }
}

pub struct Object {}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
