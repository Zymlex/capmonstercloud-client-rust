#![cfg(test)]

mod hlp;

use crate::*;
use hlp::image_file_to_base64;
use std::sync::OnceLock;
use image::ImageFormat;

const CLIENT_KEY: &str = env!("CMC_KEY");

static INIT: OnceLock<CapMonsterCloudClient> = OnceLock::new();

fn get_or_init<'a>() -> &'a CapMonsterCloudClient<'a> {
    INIT.get_or_init(|| {
        tracing_subscriber::fmt::init();
        CapMonsterCloudClient::new(CLIENT_KEY).unwrap()
    })
}

#[tokio::test]
async fn get_balance_async_check() {
    let cmc = get_or_init();

    let result = cmc.get_balance_async().await.unwrap();
    let result = result.get_result().unwrap();

    println!("{result}");
    assert!(result > 0.0);
}

#[tokio::test]
async fn image_to_text_task_check() {
    let cmc = get_or_init();

    let base64 = image_file_to_base64("src/tests/media/captcha_for_ITT.png", ImageFormat::Png);

    let obj = cmc
        .image_to_text_task(ImageToTextTask {
            body: &base64,
            Case: Some(true),
            ..Default::default()
        })
        .await
        .unwrap();

    let text = obj.get_result().unwrap().text;

    assert_eq!(text, "SDHB5");
}

#[tokio::test]
async fn recaptcha_v2_task_proxyless_check() {
    let cmc = get_or_init();

    let obj = cmc.recaptcha_v2_task_proxyless(RecaptchaV2TaskProxyless {
        websiteURL: "https://lessons.zennolab.com/captchas/recaptcha/v2_nosubmit.php?level=high",
        websiteKey: "03AEkXODCB_ZLTUMRkYuXHT-w55MSXplERbnI3tLbrHnbhm_L5fAZpUxHcme7qnXYZhwgHvb_iq4WwGDMDvVkBfscRq7BgjsUGPqH5MXP38U6RcaOxZ8eVEEERFQs82HLzly-nfI8HI_TvVLQuLfq33yN5de95CBOI3MFf5d4-4dyCalXMNte0mZFrK1CGj54AuDWOsj9X5ohttP-yWJ5hBXU2acPFsF7SnIDQEKxX_N92I_m7pGpKjc5NP9tdNl-nDcRutbBSCxjafCj2efAXCBInjthqtzjEEP76ZUCtl4bMpzy6yJ4DhVS-W6GEMSEH500uSp4rbQJlDS1PaoHiepCoKJBJZIOdNbB7HI3ZUplWXSLMBw1yxE-R4AfHsVfV-WpaKOG22I27H4IRP1RfU82nxn7x0b5ebQj-dz7pof0bFp-7gFkF8NwJAw6_vTZVAUej1u72DMweXMCulYo-g9qKbaHBsogDHX5I30j1BD9U1Ep9CzVStXy-yOO2mKJFcjxN_2BmQe2xZdsUNwHrscUEzPqmOwb9g2OiIljgfq5ZU_1vHqIdU2FdAWYDsmbGawrZ1TQYhr97aXo2zCZ7ofjye0U_-uR3Eva6H02-yroKpBoYOXAlSpFEbh8EH8TlheAqKGyL8Mzjj6-k3XK1Qqn8gzqgmAuliCtj_qafn9ycjGoFy-iY-X4MT1mO83GKenMgvEAjA9NbeRi4UMnCWEp8EQkAVgyLCPVSdHyE6F-gr7G5vKTDE5P8QEQh5lFWsrfjfhKIyjXtgIK0RMXFccTLaHFK4hip-knne4CUl4wtWbfHT-aXN8JB4B8NnLCjmEfK-fKPxEKFemu8qJiM9bfuXUslVBx0VraSRAILXquCgVaS2lt83amzNFILMsOjB2nofYVVUqQqyB-mzgAAFrWmKZfC56eiPFSFMcUqij3IuPn_fvp0itp3m_MCNT6M3IlH2X0e7qs7skCiHTXjavpq30grd4Az-JIQBQwZY_B8zHF-4s35PpptTqqsH_St9VAs3cMqHZC6CXUE9uib5JMFqJEO4nYtBSOFrLHP1qoIEaNXAfmQRFZh77sItggMCJ4aTeVbqh7OJBArBEUkZjbP2igZgrVwiC_AKZSb7sv71aFSdUcT9qteU6derGFHQy3HRsY7JU3cdgDv748Zx5d-WijpPmp5CfQfQ0HCqCfa39I1msOMtNzApl4e_NsMh1cQnph3XbQ3S1Yuiot4JgtLwxfwYIhzQyi4uYYYh8tF2-9p1RsZk7vl8ISGU262W8hGzMdAq_2bNpTrSL8bwcENCoVHyvm49L2W3VT_JKkM9-5G7-mele7XnTr4P1pEiTUY3Imn4my-dc50DoPt2GyIckZd2BTeo6ulENRKBceFMxsGnAm9oYA7IKmhiGZHzZZGP_q9SxTwaEQR1k_INajdSJYmRW_5okliTZoz-xmt7C2PuqgZYovVZbP4wES3uqHvBBKQeYbf",
        ..Default::default()
    }).await.unwrap();

    let response = obj.get_result().unwrap().gRecaptchaResponse;

    println!("{response}");

    assert!(!response.is_empty());
}

// #[tokio::test]
// async fn funcaptcha_task_proxyless_check() {
//     init_logger();
//
//     let cmc = CapMonsterCloudClient::new(CLIENT_KEY).unwrap();
//
//     let obj = cmc.funcaptcha_task_proxyless(FunCaptchaTaskProxylessReq {
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

#[tokio::test]
async fn hcaptcha_task_proxyless_check() {
    let cmc = get_or_init();

    let obj = cmc
        .hcaptcha_task_proxyless(HCaptchaTaskProxyless {
            websiteURL: "https://lessons.zennolab.com/captchas/hcaptcha/?level=alwayson",
            websiteKey: "9730e4be-0997-4abd-aef3-bbdd241d211c",
            ..Default::default()
        })
        .await
        .unwrap();

    println!("{}", obj.get_result().unwrap().gRecaptchaResponse);

    assert!(obj.get_result().unwrap().gRecaptchaResponse.len() > 0);
}

// #[test]
// #[should_panic(expected = "values don't match")]
// fn mytest() {
//     assert_eq!(1, 2, "values don't match");
// }
