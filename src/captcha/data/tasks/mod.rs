#![allow(non_snake_case, reason = "API")]

//TODO Macros for this:

// private items shadows public glob re-export

mod AmazonTask_mod;
mod ComplexImageTask_mod;
mod CustomTask_mod;
mod FunCaptchaTask_mod;
mod GeeTestTask_mod;
mod HCaptchaTask_mod;
mod ImageToTextTask_mod;
mod RecaptchaTask_mod;
mod TurnstileTask_mod;

pub use self::AmazonTask_mod::*;
pub use self::ComplexImageTask_mod::*;
pub use self::CustomTask_mod::*;
pub use self::FunCaptchaTask_mod::*;
pub use self::GeeTestTask_mod::*;
pub use self::HCaptchaTask_mod::*;
pub use self::ImageToTextTask_mod::*;
pub use self::RecaptchaTask_mod::*;
pub use self::TurnstileTask_mod::*;

use serde::Serialize;
use serde_with_macros::skip_serializing_none;
use std::fmt::Debug;

pub trait TaskReqTrait: Clone + Debug + Send + Sync + Serialize {}
pub trait TaskRespTrait: Clone + Debug {}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct ProxySettings<'a> {
    pub proxyType: ProxyType,
    pub proxyAddress: &'a str,
    pub proxyPort: u16,
    pub proxyLogin: Option<&'a str>,
    pub proxyPassword: Option<&'a str>,
}

#[derive(Serialize, Default, Clone, Debug)]
pub enum ProxyType {
    #[default]
    Http,
    Https,
    Socks4,
    Socks5,
}

#[rustfmt::skip]
impl<'a> From<ProxyType> for &'a str {
    fn from(proxy_type: ProxyType) -> Self {
        match proxy_type {
            ProxyType::Http   => "http",
            ProxyType::Https  => "https",
            ProxyType::Socks4 => "socks4",
            ProxyType::Socks5 => "socks5",
        }
    }
}
