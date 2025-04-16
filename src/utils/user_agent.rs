// use serde::Serialize;


// #[derive(Serialize, Default, Clone, Debug)]
// pub struct UserAgent {
//     inner: String,
// }

// // impl UserAgentCache {
// //     pub fn update(&self) {}
// // }

// impl Into<UserAgent> for String {
//     fn into(self) -> UserAgent {
//         UserAgent {
//             inner: self
//         }
//     }
// }

// impl<'a> Into<UserAgent> for &'a str {
//     fn into(self) -> UserAgent {
//         UserAgent {
//             inner: self.to_owned()
//         }
//     }
// }