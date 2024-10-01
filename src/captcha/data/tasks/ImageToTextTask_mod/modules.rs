#![allow(non_camel_case_types, reason = "API")]
#![allow(clippy::upper_case_acronyms, reason = "API")]

use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub enum ITT_Modules {
    AMAZON,
    BOT_DETECT,
    FACEBOOK,
    GMX,
    GOOGLE,
    HOTMAIL,
    MAIL_RU,
    OK,
    OK_NEW,
    RAMBLER_RUS,
    SOLVE_MEDIA,
    STEAM,
    VK,
    VK_RUS,
 // YANDEX,
 // YANDEX_NEW,
 // YANDEX_WAVE,
    UNIVERSAL,
}

impl<'a> From<ITT_Modules> for &'a str {
    fn from(value: ITT_Modules) -> Self {
        match value {
            ITT_Modules::AMAZON      => "amazon",
            ITT_Modules::BOT_DETECT  => "botdetect",
            ITT_Modules::FACEBOOK    => "facebook",
            ITT_Modules::GMX         => "gmx",
            ITT_Modules::GOOGLE      => "google",
            ITT_Modules::HOTMAIL     => "hotmail",
            ITT_Modules::MAIL_RU     => "mailru",
            ITT_Modules::OK          => "ok",
            ITT_Modules::OK_NEW      => "oknew",
            ITT_Modules::RAMBLER_RUS => "ramblerrus",
            ITT_Modules::SOLVE_MEDIA => "solvemedia",
            ITT_Modules::STEAM       => "steam",
            ITT_Modules::VK          => "vk",
            ITT_Modules::VK_RUS      => "vk_rus",
         // ITT_Modules::YANDEX      => "yandex",
         // ITT_Modules::YANDEX_NEW  => "yandexnew",
         // ITT_Modules::YANDEX_WAVE => "yandexwave",
            ITT_Modules::UNIVERSAL   => "universal",
        }
    }
}