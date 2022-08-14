// use k8s_openapi::api::core::v1::Pod;
// use kube::{Api, Client, Error};
// use serde_json::json;
// use kube::Error::Api;

mod crd;

// use crd::Yuri;
// use crd::YuriSpec;

#[tokio::main]
async fn main() {
    // let kubernetes_client = Client::try_default()
    //     .await
    //     .expect("Expected a valid Kubeconfig env var");

    let namespace = std::env::var("NAMESPACE").unwrap_or_else(|_| "default".into());
    // let pods: Api<Pod> = Api::namespaced(kubernetes_client, &namespace);
    // let pods: Api<Yuri> = Api::namespaced(kubernetes_client, &namespace);
    // let crd_api: Api<crd::Yuri> = Api::all(kubernetes_client.clone());
    // let d: Pod = serde_json::json!({
    //     "apiVersion": "v1",
    //   "kind": "Pod",
    //   "metadata": { "name": "blog"},
    //   "spec": {
    //     "containers": [
    //       {
    //         "name": "blod",
    //         "image": "clux/blod:0.1.0"
    //       }
    //     ]
    //   }
    // })
    // let p: Pod = serde_json::from_value(json!({
    //   "apiVersion": "v1",
    //   "kind": "Pod",
    //   "metadata": { "name": "blog"},
    //   "spec": {
    //     "containers": [
    //       {
    //         "name": "blod",
    //         "image": "clux/blod:0.1.0"
    //       }
    //     ]
    //   }
    // }))
    // pods.get()
    println!("hello {}", namespace);
    println!("Hello, world!");
}
