use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ZConfig{
    pub global: ZGlobalConfig,
    pub proxy: Option<ZProxyConfig>
}

#[derive(Serialize, Deserialize)]
pub struct ZGlobalConfig{
    pub domain: Option<String>,
    pub username: String,
    pub token: Option<String>,
    pub lang: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct ZProxyConfig{
    pub http: Option<String>,
    pub socks5: Option<String>
}