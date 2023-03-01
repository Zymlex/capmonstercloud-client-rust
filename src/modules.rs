#![allow(non_camel_case_types)]

use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub enum Modules {
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
    YANDEX,
    YANDEX_NEW,
    YANDEX_WAVE,
    UNIVERSAL,
}

// impl<'a> Into<&str> for Modules {
//     fn into(self) -> &'a str {
//         match self {
//             Modules::AMAZON => "amazon",
//             Modules::BOT_DETECT => "botdetect",
//             Modules::FACEBOOK => "facebook",
//             Modules::GMX => "gmx",
//             Modules::GOOGLE => "google",
//             Modules::HOTMAIL => "hotmail",
//             Modules::MAIL_RU => "mailru",
//             Modules::OK => "ok",
//             Modules::OK_NEW => "oknew",
//             Modules::RAMBLER_RUS => "ramblerrus",
//             Modules::SOLVE_MEDIA => "solvemedia",
//             Modules::STEAM => "steam",
//             Modules::VK => "vk",
//             Modules::YANDEX => "yandex",
//             Modules::YANDEX_NEW => "yandexnew",
//             Modules::YANDEX_WAVE => "yandexwave",
//             Modules::UNIVERSAL => "universal",
//         }
//     }
// }