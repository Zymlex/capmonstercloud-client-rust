use serde::Serialize;

#[derive(Serialize, Default, Clone, Debug)]
pub enum ProxyType {
    #[default]
    Http,
    Https,
    Socks4,
    Socks5
}

impl<'a> From<ProxyType> for &'a str {
    fn from(proxy_type: ProxyType) -> &'a str {
        match proxy_type {
            ProxyType::Http => "http",
            ProxyType::Https => "https",
            ProxyType::Socks4 => "socks4",
            ProxyType::Socks5 => "socks5",
        }
    }
}