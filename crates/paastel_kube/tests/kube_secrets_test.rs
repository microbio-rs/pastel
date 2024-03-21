mod common;

use paastel_kube::{KubeSecrets, Secrets};

#[tokio::test]
async fn list_secrets() {
    let client = common::kube_client().await;
    let kube_secrets = KubeSecrets::new(client);
    let secrets = kube_secrets.list_user_secrets().await.unwrap();
    assert!(secrets.len() > 0);
}
