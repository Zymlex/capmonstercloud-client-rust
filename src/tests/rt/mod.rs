mod server;

use tokio::{
    sync::OnceCell,
    task::{self, JoinHandle},
};

use super::{CapMonsterCloudClient, cfg::APIRootUrls};

static TRACING: OnceCell<()> = OnceCell::const_new();

static SERVER: OnceCell<JoinHandle<()>> = OnceCell::const_new();

static CLIENT: OnceCell<(CapMonsterCloudClient, String)> = OnceCell::const_new();
static CLIENT_WRONG: OnceCell<(CapMonsterCloudClient, String)> = OnceCell::const_new();

pub(super) async fn init_all_and_get_client_with_ua<'a>() -> &'a (CapMonsterCloudClient<'a>, String)
{
    CLIENT
        .get_or_init(async || {
            init_server();

            let cmc =
                CapMonsterCloudClient::new_for_tests(&super::API_ROOTS, super::CLIENT_KEY).unwrap();
            let ua = cmc.get_user_agent_async().await.unwrap();

            (cmc, ua)
        })
        .await
}

pub(super) async fn init_all_and_get_wrong_client_with_ua<'a>()
-> &'a (CapMonsterCloudClient<'a>, String) {
    CLIENT_WRONG
        .get_or_init(async || {
            init_server();

            let cmc =
                CapMonsterCloudClient::new_for_tests(&super::API_ROOTS, super::CLIENT_KEY_WRONG)
                    .unwrap();
            let ua = "wrong_ua".to_owned();

            (cmc, ua)
        })
        .await
}

fn init_server<'a>() {
    if !SERVER.initialized() {
        init_tracing();

        let server_rt = task::spawn(async move {
            server::start_server(super::IP_PORT, &super::API_ROOTS)
                .await
                .unwrap();
        });

        SERVER.set(server_rt).unwrap();
    }
}

fn init_tracing() {
    if !TRACING.initialized() {
        tracing_subscriber::fmt::init();

        TRACING.set(()).unwrap();
    }
}
