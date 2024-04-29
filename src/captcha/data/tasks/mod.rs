#![allow(non_snake_case)]

//TODO Macros for this:

// private items shadows public glob re-export

mod ImageToTextTask_mod;
mod Recaptcha;
mod FunCaptchaTask_mod;
mod HCaptchaTask_mod;
mod GeeTestTask_mod;
mod TurnstileTask_mod;
mod ComplexImageTask_mod;
mod CustomTask;

pub use self::ImageToTextTask_mod::*;
pub use self::Recaptcha::*;
pub use self::FunCaptchaTask_mod::*;
pub use self::HCaptchaTask_mod::*;
pub use self::GeeTestTask_mod::*;
pub use self::TurnstileTask_mod::*;
pub use self::ComplexImageTask_mod::*;
pub use self::CustomTask::*;

use serde::Serialize;
use serde_with_macros::skip_serializing_none;
use std::fmt::Debug;

pub trait TaskReqTrait: Clone + Debug {}
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
    Socks5
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