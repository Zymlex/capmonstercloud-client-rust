#![cfg(test)]

use crate::CapMonsterCloudClient;
use crate::requests::tasks_data::*;

#[tokio::test]
async fn get_balance_async_check() {
    let cmc = CapMonsterCloudClient::new(env!("CMC_KEY")).unwrap();
    
    let result = cmc.get_balance_async().await.unwrap();
    println!("{}", result);
    assert!(result > 0.0);
}

#[tokio::test]
async fn image_to_text_task_check() {
    let cmc = CapMonsterCloudClient::new(env!("CMC_KEY")).unwrap();
    
    let obj = cmc.image_to_text_task(ImageToTextTaskReq {
        body: "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",
        Case: Some(true),
        ..Default::default()
    }).await.unwrap();
    
    assert_eq!(obj.text, "SDHB5");
}

#[tokio::test]
async fn no_captcha_task_proxyless_check() {
    let cmc = CapMonsterCloudClient::new(env!("CMC_KEY")).unwrap();
    
    let obj = cmc.no_captcha_task_proxyless(NoCaptchaTaskProxylessReq {
        websiteURL: "https://lessons.zennolab.com/captchas/recaptcha/v2_nosubmit.php?level=high",
        websiteKey: "03AEkXODCB_ZLTUMRkYuXHT-w55MSXplERbnI3tLbrHnbhm_L5fAZpUxHcme7qnXYZhwgHvb_iq4WwGDMDvVkBfscRq7BgjsUGPqH5MXP38U6RcaOxZ8eVEEERFQs82HLzly-nfI8HI_TvVLQuLfq33yN5de95CBOI3MFf5d4-4dyCalXMNte0mZFrK1CGj54AuDWOsj9X5ohttP-yWJ5hBXU2acPFsF7SnIDQEKxX_N92I_m7pGpKjc5NP9tdNl-nDcRutbBSCxjafCj2efAXCBInjthqtzjEEP76ZUCtl4bMpzy6yJ4DhVS-W6GEMSEH500uSp4rbQJlDS1PaoHiepCoKJBJZIOdNbB7HI3ZUplWXSLMBw1yxE-R4AfHsVfV-WpaKOG22I27H4IRP1RfU82nxn7x0b5ebQj-dz7pof0bFp-7gFkF8NwJAw6_vTZVAUej1u72DMweXMCulYo-g9qKbaHBsogDHX5I30j1BD9U1Ep9CzVStXy-yOO2mKJFcjxN_2BmQe2xZdsUNwHrscUEzPqmOwb9g2OiIljgfq5ZU_1vHqIdU2FdAWYDsmbGawrZ1TQYhr97aXo2zCZ7ofjye0U_-uR3Eva6H02-yroKpBoYOXAlSpFEbh8EH8TlheAqKGyL8Mzjj6-k3XK1Qqn8gzqgmAuliCtj_qafn9ycjGoFy-iY-X4MT1mO83GKenMgvEAjA9NbeRi4UMnCWEp8EQkAVgyLCPVSdHyE6F-gr7G5vKTDE5P8QEQh5lFWsrfjfhKIyjXtgIK0RMXFccTLaHFK4hip-knne4CUl4wtWbfHT-aXN8JB4B8NnLCjmEfK-fKPxEKFemu8qJiM9bfuXUslVBx0VraSRAILXquCgVaS2lt83amzNFILMsOjB2nofYVVUqQqyB-mzgAAFrWmKZfC56eiPFSFMcUqij3IuPn_fvp0itp3m_MCNT6M3IlH2X0e7qs7skCiHTXjavpq30grd4Az-JIQBQwZY_B8zHF-4s35PpptTqqsH_St9VAs3cMqHZC6CXUE9uib5JMFqJEO4nYtBSOFrLHP1qoIEaNXAfmQRFZh77sItggMCJ4aTeVbqh7OJBArBEUkZjbP2igZgrVwiC_AKZSb7sv71aFSdUcT9qteU6derGFHQy3HRsY7JU3cdgDv748Zx5d-WijpPmp5CfQfQ0HCqCfa39I1msOMtNzApl4e_NsMh1cQnph3XbQ3S1Yuiot4JgtLwxfwYIhzQyi4uYYYh8tF2-9p1RsZk7vl8ISGU262W8hGzMdAq_2bNpTrSL8bwcENCoVHyvm49L2W3VT_JKkM9-5G7-mele7XnTr4P1pEiTUY3Imn4my-dc50DoPt2GyIckZd2BTeo6ulENRKBceFMxsGnAm9oYA7IKmhiGZHzZZGP_q9SxTwaEQR1k_INajdSJYmRW_5okliTZoz-xmt7C2PuqgZYovVZbP4wES3uqHvBBKQeYbf",
        ..Default::default()
    }).await.unwrap();
    
    assert!(obj.gRecaptchaResponse.len() > 0);
}

// #[tokio::test]
// async fn funcaptcha_task_proxyless_check() {
//     let cmc = CapMonsterCloudClient::new(env!("CMC_KEY")).unwrap();
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
    let cmc = CapMonsterCloudClient::new(env!("CMC_KEY")).unwrap();
    
    let obj = cmc.hcaptcha_task_proxyless(HCaptchaTaskProxylessReq {
        websiteURL: "https://lessons.zennolab.com/captchas/hcaptcha/?level=alwayson",
        websiteKey: "9730e4be-0997-4abd-aef3-bbdd241d211c",
        ..Default::default()
    }).await.unwrap();
    
    println!("{}", obj.gRecaptchaResponse);
    
    assert!(obj.gRecaptchaResponse.len() > 0);
}