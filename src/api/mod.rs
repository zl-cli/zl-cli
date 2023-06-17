use serde_json::json;
use z_api::ZApi;
use crate::config::ZConfig;

struct ZApiImpl{
    config: ZConfig
}

impl ZApi for ZApiImpl{
    fn get(&self, path: String) -> serde_json::Value{
        let domain = &self.config.global.domain;
        let default_domain = "singlelogin.re".to_string();
        let host = domain.as_ref().unwrap_or(&default_domain);

        let url = format!("https://{host}{path}");

        todo!("switch to async");
        let result = reqwest::get(url).await;
        json!({})
    }

    fn post(&self, path: String, data: serde_json::Value)  -> serde_json::Value{
        todo!("switch to async");
        json!({})
    }
}