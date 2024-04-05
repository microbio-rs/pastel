#![allow(dead_code)]
// Copyright (c) 2024 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

pub mod client;
pub mod error;
pub mod mapper;
pub mod secrets;

use async_trait::async_trait;
use client::KubernetesClient;
use kube::api::ListParams;
use mapper::KubernetesMapper;
use secrets::KubernetsSecretsAdapter;

use paastel_auth::{OutgoingKubernetesPort, SecretLabel};

// use std::{
//     collections::{BTreeMap, HashMap},
//     result::Result as StdResult,
// };

// use async_trait::async_trait;
// use k8s_openapi::{
//     api::core::v1::Secret,
//     apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
// };
// use kube::{
//     api::{ListParams, Patch, PatchParams, PostParams},
//     core::ObjectList,
//     runtime::{conditions, wait::await_condition},
//     Api, Client, CustomResource, CustomResourceExt, Error as KError, Resource,
// };

// use paastel::{
//     AuthKubeSecretPort, AuthUser, AuthUsers, CreateAppCommand,
//     CreateKubeCRDPort, Error as PaastelError, Result, Username,
// };
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};
// use tracing::info;

#[derive(Clone)]
pub struct KubernetesAdapter {
    mapper: KubernetesMapper,
    secrets: KubernetsSecretsAdapter,
}

impl KubernetesAdapter {
    pub fn new(client: &KubernetesClient) -> Self {
        Self {
            mapper: KubernetesMapper::default(),
            secrets: KubernetsSecretsAdapter::new(client),
        }
    }
}

#[async_trait]
impl OutgoingKubernetesPort for KubernetesAdapter {
    async fn find_secrets_by_label(
        &self,
        label: &SecretLabel,
    ) -> paastel_auth::Result<paastel_auth::UserSecrets> {
        let label_str = label.to_string();
        println!("{label_str}");
        let lp = ListParams::default().match_any().labels(&label_str);
        let secrets_list = self
            .secrets
            .get_all(&lp)
            .await
            .map_err(|_| paastel_auth::Error::SecretNotFound)?;
        let user_secrets = self.mapper.list_secrets_to_domain(&secrets_list);
        Ok(user_secrets)
    }
}

// #[derive(Debug, Clone)]
// pub struct KubeSecrets {
//     api: Api<Secret>,
// }

// impl KubeSecrets {
//     pub fn new(client: Client) -> Self {
//         Self {
//             api: Api::default_namespaced(client),
//         }
//     }

//     async fn get_all(&self) -> StdResult<ObjectList<Secret>, KError> {
//         let lp = ListParams::default()
//             .match_any()
//             .timeout(60)
//             .labels("paastel.io/api-user-credentials=true");
//         self.api.list(&lp).await
//     }
// }

// #[async_trait]
// impl AuthKubeSecretPort for KubernetesAdapter {
//     async fn list(&self) -> Result<AuthUsers> {
//         let secrets = self
//             .secrets
//             .get_all()
//             .await
//             .map_err(|e| PaastelError::KubePort(e.to_string()))?;

//         let content: HashMap<Username, AuthUser> = secrets
//             .into_iter()
//             .map(|c| {
//                 let data = c.data.unwrap();
//                 let metadata = c.metadata;
//                 let username = {
//                     let u = &data.get("username").unwrap().0;
//                     String::from_utf8(u.to_owned()).unwrap()
//                 };
//                 let password = {
//                     let p = &data.get("password").unwrap().0;
//                     String::from_utf8(p.to_owned()).unwrap()
//                 };
//                 let secrt_name = metadata.name.unwrap();
//                 let auth_user = AuthUser::new(
//                     username.clone().into(),
//                     password,
//                     secrt_name,
//                 );
//                 (username.into(), auth_user)
//             })
//             .collect();

//         let auth_users = AuthUsers::new(content);
//         Ok(auth_users)
//     }
// }

// #[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
// #[kube(
//     group = "application.paastel.io",
//     version = "v1",
//     kind = "App",
//     namespaced
// )]
// struct AppSpec {
//     origin: String,
// }

// #[derive(Debug, Clone)]
// pub struct KubeCustomResource {
//     api: Api<App>,
// }

// impl KubeCustomResource {
//     pub fn new(client: Client) -> Self {
//         Self {
//             api: Api::default_namespaced(client),
//         }
//     }

//     #[allow(dead_code)]
//     async fn init(&self) -> StdResult<(), KError> {
//         info!("init crd App");
//         let client = Client::try_default().await?;
//         let crds: Api<CustomResourceDefinition> = Api::all(client);

//         // Apply the CRD so users can create Foo instances in Kubernetes
//         crds.patch(
//             "apps.application.paastel.io",
//             &PatchParams::apply("apps_manager"),
//             &Patch::Apply(App::crd()),
//         )
//         .await?;

//         // Wait for the CRD to be ready
//         let _ = tokio::time::timeout(
//             std::time::Duration::from_secs(10),
//             await_condition(
//                 crds,
//                 "apps.application.paastel.io",
//                 conditions::is_crd_established(),
//             ),
//         )
//         .await
//         .unwrap();
//         info!("complete init crd App");
//         Ok(())
//     }

//     pub async fn create(&self, name: &str) -> StdResult<(), KError> {
//         let spec = AppSpec {
//             origin: "path".to_string(),
//         };
//         let mut app = App::new(name, spec);
//         let mut annnotations = BTreeMap::new();
//         annnotations.insert(
//             "paastal.io/created-by".to_string(),
//             "admin@paastel.io".to_string(),
//         );
//         app.meta_mut().annotations = Some(annnotations);
//         // println!("{}", serde_yaml::to_string(&app).unwrap()); // crd yaml
//         self.api.create(&PostParams::default(), &app).await?;
//         Ok(())
//     }
// }

// #[async_trait]
// impl CreateKubeCRDPort for KubernetesAdapter {
//     async fn crd(&self, command: &CreateAppCommand) -> Result<()> {
//         // NOTE: run just one
//         // self.crd.init().await.unwrap();
//         self.crd.create(command.name.0.as_str()).await.unwrap();
//         Ok(())
//     }
// }
