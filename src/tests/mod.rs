#![cfg(test)]
#![allow(non_snake_case, reason = "API")]

mod hlp;

use crate::*;
use hlp::image_file_to_base64;
use image::ImageFormat;
use tokio::sync::OnceCell;
use tracing::info;

const CLIENT_KEY: &str = env!("CMC_KEY");

static CELL: OnceCell<(CapMonsterCloudClient, String)> = OnceCell::const_new();

async fn get_client_and_ua<'a>() -> &'a (CapMonsterCloudClient<'a>, String) {
    CELL.get_or_init(|| async {
        tracing_subscriber::fmt::init();
        let cmc = CapMonsterCloudClient::new(CLIENT_KEY).unwrap();
        let ua = cmc.get_user_agent_async().await.unwrap();

        (cmc, ua)
    }).await
}

#[tokio::test]
async fn get_balance_async_check() {
    let (cmc, _) = get_client_and_ua().await;

    let result = cmc.get_balance_async().await.unwrap();

    info!("{result}");
    assert!(result > 0.0);
}

#[tokio::test]
async fn image_to_text_task_check() {
    let (cmc, _) = get_client_and_ua().await;

    let base64 = image_file_to_base64("src/tests/media/captcha_for_ITT.png", ImageFormat::Png);

    let text = cmc
        .image_to_text(ImageToTextTask {
            body: &base64,
            Case: Some(true),
            ..Default::default()
        })
        .await
        .unwrap();

    info!("{text}");
    assert_eq!(text, "SDHB5");
}

#[tokio::test]
async fn recaptcha_v2_task_proxyless_check() {
    let (cmc, ua) = get_client_and_ua().await;

    let gRecaptchaResponse = cmc.recaptcha_v2(RecaptchaV2TaskProxyless {
        websiteURL: "https://lessons.zennolab.com/captchas/recaptcha/v2_nosubmit.php?level=high",
        websiteKey: "6Lcg7CMUAAAAANphynKgn9YAgA4tQ2KI_iqRyTwd",
        userAgent: Some(ua),
        ..Default::default()
    }).await.unwrap();

    info!("{gRecaptchaResponse}");

    assert!(!gRecaptchaResponse.is_empty());
}

#[tokio::test]
async fn recaptcha_v3_task_proxyless_check() {
    let (cmc, _) = get_client_and_ua().await;

    let gRecaptchaResponse = cmc.recaptcha_v3(RecaptchaV3TaskProxyless {
        websiteURL: "https://lessons.zennolab.com/captchas/recaptcha/v3.php?level=beta",
        websiteKey: "6Le0xVgUAAAAAIt20XEB4rVhYOODgTl00d8juDob",
        minScore: Some(0.3),
        pageAction: Some("myverify"),
        ..Default::default()
    }).await.unwrap();

    info!("{gRecaptchaResponse}");

    assert!(!gRecaptchaResponse.is_empty());
}

#[tokio::test]
async fn recaptcha_v2_enterprise_task_proxyless_check() {
    let (cmc, ua) = get_client_and_ua().await;

    let gRecaptchaResponse = cmc.recaptcha_v2_enterprise(RecaptchaV2EnterpriseTaskProxyless{
        websiteURL: "https://cabura.salon/",
        websiteKey: "6LeelqAUAAAAANC5GR_WWHaMeDH45EPA6gTZ1WAk",
        enterprisePayload: None,
        userAgent: Some(ua),
        ..Default::default()
    }).await.unwrap();

    info!("{gRecaptchaResponse}");

    assert!(!gRecaptchaResponse.is_empty());
}

#[tokio::test]
async fn hcaptcha_task_proxyless_check() {
    let (cmc, ua) = get_client_and_ua().await;

    let gRecaptchaResponse = cmc
        .hcaptcha(HCaptchaTaskProxyless {
            websiteURL: "https://lessons.zennolab.com/captchas/hcaptcha/?level=alwayson",
            websiteKey: "9730e4be-0997-4abd-aef3-bbdd241d211c",
            userAgent: Some(ua),
            ..Default::default()
        })
        .await
        .unwrap();

    info!("{gRecaptchaResponse}");

    assert!(!gRecaptchaResponse.is_empty());
}

#[tokio::test]
async fn geetest_task_proxyless_check() {
    let (cmc, ua) = get_client_and_ua().await;

    let result = cmc
        .geetest(GeeTestTaskProxyless {
            websiteURL: "https://www.geetest.com/en/demo",
            gt: "81388ea1fc187e0c335c0a8907ff2625",
            challenge: "f5892e6323e5be0dab92c37859ac671c",
            userAgent: Some(ua),
            ..Default::default()
        })
        .await
        .unwrap();

    info!("{result:?}");

    assert!(!result.challenge.is_empty());
    assert!(!result.seccode.is_empty());
    assert!(!result.validate.is_empty());
}

#[tokio::test]
async fn turnstile_check() {
    let (cmc, _) = get_client_and_ua().await;

    let result = cmc
        .turnstile(Turnstile {
            websiteURL: "http://tsmanaged.zlsupport.com",
            websiteKey: "0x4AAAAAAABUYP0XeMJF0xoy",
            pageAction: None,
        })
        .await
        .unwrap();

    info!("{result:?}");

    assert!(!result.token.is_empty());
    assert!(!result.cf_clearance.is_empty());
}

// #[tokio::test]
// async fn cloudflare_token_check() {
//     let (cmc, ua) = get_client_and_ua().await;

//     let result = cmc
//         .cloudflare_token(CloudFlareToken {
//             websiteURL: "https://www.cityline.com/Events.html",
//             websiteKey: "",
//             userAgent: ua,
//             pageAction: "",
//             pageData: "",
//             data: "",
//         })
//         .await
//         .unwrap();

//     info!("{result:?}");

//     assert!(!result.token.is_empty());
//     assert!(!result.cf_clearance.is_empty());
// }

// #[tokio::test]
// async fn funcaptcha_task_proxyless_check() {
//     let (cmc, ua) = get_client();
//
//     let obj = cmc.funcaptcha(FunCaptchaTaskProxylessReq {
//         websiteURL: "",
//         websitePublicKey: "",
//         funcaptchaApiJSSubdomain: Some(""),
//         ..Default::default()
//     }).await.unwrap();
//
//     info!("{}", obj.token);
//
//     assert!(obj.token.len() > 0);
// }

// #[test]
// #[should_panic(expected = "values don't match")]
// fn mytest() {
//     assert_eq!(1, 2, "values don't match");
// }
