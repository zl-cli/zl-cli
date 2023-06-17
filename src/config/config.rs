use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ZConfig{
    global: ZGlobalConfig,
    proxy: Option<ZProxyConfig>
}

#[derive(Serialize, Deserialize)]
pub struct ZGlobalConfig{
    domain: Option<String>,
    username: String,
    token: Option<String>,
    lang: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct ZProxyConfig{
    http: Option<String>,
    socks5: Option<String>
}