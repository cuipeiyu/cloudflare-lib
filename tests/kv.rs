use cloudflare_lib::kv::*;
use cloudflare_lib::*;
use std::collections::HashMap;

// RUST_LOG=error,cloudflare_lib=trace,kv=trace cargo test --test kv --features with-kv
#[tokio::test]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = ClientBuilder::new(ApiAuth::default()).build().unwrap();

    let account_id = ::std::env::var("CLOUDFLARE_ACCOUNT_ID").unwrap_or_default();

    test_namespace(&client, account_id).await;

    tracing::info!("all done");
}

async fn test_namespace(client: &Client, account_id: String) {
    // new namespace
    let body = NamespaceNewBody {
        title: "test".to_string(),
    };
    let new_one = client.kv_namespaces_new(&account_id, &body).await.unwrap();
    tracing::info!("new namespace resp: {:?}", new_one);

    assert_ne!(new_one, None);

    let new_one = new_one.unwrap();

    // list all namespaces
    let list = client.kv_namespaces_list(&account_id, None).await.unwrap();
    tracing::info!("list namespace resp: {:?}", list);

    if let Some(list) = list.result {
        for item in list.into_iter() {
            // find test namespace
            if item.id == new_one.id {
                tracing::info!("item namespace resp: {:?}", item);

                let body = NamespaceUpdateBody {
                    title: "test_new_name".to_string(),
                };
                let update = client
                    .kv_namespaces_update(&account_id, &item.id, &body)
                    .await
                    .unwrap();
                tracing::info!("update namespace resp: {:?}", update);

                if &update.title != "test_new_name" {
                    panic!("namespace title not updated");
                }

                test_namespace_keys(&client, account_id.clone(), update.id.clone()).await;

                let delete = client
                    .kv_namespaces_delete(&account_id, &update.id)
                    .await
                    .unwrap();
                tracing::info!("delete namespace resp: {:?}", delete);
            }
        }
    }
}

async fn test_namespace_keys(client: &Client, account_id: String, namespace_id: String) {
    // write new key to namespace
    let body = vec![
        NamespaceBulkUpdateParamsBody {
            key: "test_key1".to_string(),
            value: "test_value1".to_string(),
            ..Default::default()
        },
        NamespaceBulkUpdateParamsBody {
            key: "test_key2".to_string(),
            value: "test_value2".to_string(),
            expiration_ttl: Some(200),
            ..Default::default()
        },
    ];
    let bulk_update = client
        .kv_namespaces_bulk_update(&account_id, &namespace_id, &body)
        .await
        .unwrap();
    tracing::info!("bulk update keys resp: {:?}", bulk_update);

    // list namespace keys
    let list = client
        .kv_namespaces_keys_list(&account_id, &namespace_id, None)
        .await
        .unwrap();
    tracing::info!("list keys resp: {:?}", list);

    for key in list.result.into_iter() {
        tracing::info!("key detail: {:?}", key);

        match key.name.as_str() {
            "test_key1" => {
                let body = NamespaceValueUpdateBody {
                    value: "key1 new_value".to_string(),
                    metadata: Some(HashMap::from([("foo".to_string(), "bar".to_string())])),
                };
                let update = client
                    .kv_namespaces_values_update(&account_id, &namespace_id, &key.name, None, &body)
                    .await
                    .unwrap();
                tracing::info!("key update: {} {:?}", &key.name, update);

                // get key metadata
                let metadata = client
                    .kv_namespaces_metadata_get(&account_id, &namespace_id, &key.name)
                    .await
                    .unwrap();
                tracing::info!("key metadata: {} {:?}", &key.name, metadata);
            }
            "test_key2" => {
                // get key value
                let get = client
                    .kv_namespaces_values_get(&account_id, &namespace_id, &key.name)
                    .await
                    .unwrap();
                tracing::info!("key value: {} {:?}", &key.name, get);
            }
            _ => {}
        }

        // delete namespace key value
        let delete = client
            .kv_namespaces_values_delete(&account_id, &namespace_id, &key.name)
            .await
            .unwrap();
        tracing::info!("delete key: {} {:?}", &key.name, delete);
    }
}
