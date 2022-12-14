// mod mongo-connect;
// use crate::libs::rust::mongo-connect;
// use log::{info, warn};

// use k8s_openapi::serde_json;
// use serde_json::json;
// use cli_test::cli_test;
// use k8s_openapi::api::core::v1::Pod;
// use k8s_openapi::serde_json;
// use kube::core::{CustomResourceExt, Resource};
// use kube::CustomResource;
use mongo_connect::auth_utils::models::Credentials;
use mongo_connect::authenticate;
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

/// Struct corresponding to the Specification (`spec`) part of the `Echo` resource, directly
/// reflects context of the `echoes.example.com.yaml` file to be found in this repository.
/// The `Echo` struct will be generated by the `CustomResource` derive macro.
// #[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
// #[kube(
//     group = "example.com",
//     version = "v1",
//     kind = "Echo",
//     plural = "echoes",
//     derive = "PartialEq",
//     namespaced
// )]
// pub struct EchoSpec {
//     pub replicas: i32,
// }
//
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// struct FooStatus {
//     replicas: i32,
// }

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";
fn main() {
    let yuri = Credentials {
        username: "arisa".to_string(),
        password: "123456".to_string(),
    };
    authenticate(yuri);
    println!("Hello, world!");
    println!("Running at url: {}", ECHO_SERVER_ADDRESS);
    let _grade = Some("A+");
    let grades = vec!["B+", "C+", "D"];

    // let pods: Api<Pod> = Api::default_namespaced(client);
    //
    // let p = pods.get("blog").await?;
    // println!(
    //     "Got blog pod with containers: {:?}",
    //     p.spec.unwrap().containers
    // );
    //
    // let patch = json!({"spec": {
    //     "activeDeadlineSeconds": 5
    // }});
    // let pp = PatchParams::apply("kube");
    // let patched = pods.patch("blog", &pp, &Patch::Apply(patch)).await?;
    // assert_eq!(patched.spec.active_deadline_seconds, Some(5));

    // grades.extend(grade);
    // use k8s_openapi::Resource;
    // println!("kind = {}", Foo::KIND); // impl k8s_openapi::Resource
    // let f = Foo::new(
    //     "foo-1",
    //     FooSpec {
    //
    //         // info: "informative info".into(),
    //     },
    // );
    // println!("foo: {:?}", f); // debug print on generated type
    // println!("crd: {}", serde_yaml::to_string(&Foo::crd()).unwrap()); // crd yaml
    // let gradess: Vec<Option<&str>> = grades.iter().collect();
    // println!("my item {}", gradess[0]);
    println!("{grades:?}");
}
