#![cfg(test)]

mod hlp;

use crate::*;
use hlp::image_file_to_base64;
use std::sync::OnceLock;
use image::ImageFormat;

const CLIENT_KEY: &str = env!("CMC_KEY");

static INIT: OnceLock<CapMonsterCloudClient> = OnceLock::new();

fn get_client<'a>() -> &'a CapMonsterCloudClient<'a> {
    INIT.get_or_init(|| {
        tracing_subscriber::fmt::init();
        CapMonsterCloudClient::new(CLIENT_KEY).unwrap()
    })
}

#[tokio::test]
async fn get_balance_async_check() {
    let cmc = get_client();

    let result = cmc.get_balance_async().await.unwrap();

    println!("{result}");
    assert!(result > 0.0);
}

#[tokio::test]
async fn image_to_text_task_check() {
    let cmc = get_client();

    let base64 = image_file_to_base64("src/tests/media/captcha_for_ITT.png", ImageFormat::Png);

    let text = cmc
        .image_to_text(ImageToTextTask {
            body: &base64,
            Case: Some(true),
            ..Default::default()
        })
        .await
        .unwrap();

    println!("{text}");
    assert_eq!(text, "SDHB5");
}

#[tokio::test]
async fn recaptcha_v2_task_proxyless_check() {
    let cmc = get_client();

    let response = cmc.recaptcha_v2(RecaptchaV2TaskProxyless {
        websiteURL: "https://lessons.zennolab.com/captchas/recaptcha/v2_nosubmit.php?level=high",
        websiteKey: "6Lcg7CMUAAAAANphynKgn9YAgA4tQ2KI_iqRyTwd",
        ..Default::default()
    }).await.unwrap();

    println!("{response}");

    assert!(!response.is_empty());
}

#[tokio::test]
async fn hcaptcha_task_proxyless_check() {
    let cmc = get_client();

    #[allow(non_snake_case, reason = "API")]
    let gRecaptchaResponse = cmc
        .hcaptcha(HCaptchaTaskProxyless {
            websiteURL: "https://lessons.zennolab.com/captchas/hcaptcha/?level=alwayson",
            websiteKey: "9730e4be-0997-4abd-aef3-bbdd241d211c",
            ..Default::default()
        })
        .await
        .unwrap();

    println!("{}", gRecaptchaResponse);

    assert!(gRecaptchaResponse.len() > 0);
}

// #[tokio::test]
// async fn funcaptcha_task_proxyless_check() {
//     init_logger();
//
//     let cmc = CapMonsterCloudClient::new(CLIENT_KEY).unwrap();
//
//     let obj = cmc.funcaptcha(FunCaptchaTaskProxylessReq {
//         websiteURL: "",
//         websitePublicKey: "",
//         funcaptchaApiJSSubdomain: Some(""),
//         ..Default::default()
//     }).await.unwrap();
//
//     println!("{}", obj.token);
//
//     assert!(obj.token.len() > 0);
// }

// #[test]
// #[should_panic(expected = "values don't match")]
// fn mytest() {
//     assert_eq!(1, 2, "values don't match");
// }
