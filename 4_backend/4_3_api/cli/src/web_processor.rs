use url::Url;
use entities::user;
use crate::cli_processor::CliCommand;




pub(crate) struct WebProcessor {
    backend: Url
}

impl WebProcessor {
    pub fn new(url: Url) -> Self {
        Self {
            backend: url
        }
    }

    async fn send_json_to_backend_root(&self, json: String) -> String {
        let url = format!("{}", self.backend);
        reqwest::Client::new()
            .post(url)
            .body(json)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    pub async fn process_command(&self, command: CliCommand) -> String {
        match command {
            CliCommand::AddUser(request) => {
                let user = user::Model {
                    id: 0,
                    name: request.name.0
                };
                let json = serde_json::to_string(&user).unwrap();
                let url = format!("{}users/", self.backend);
                reqwest::Client::new()
                    .post(url)
                    .body(json)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::AddRole(request) => {
                let role = entities::role::Model {
                    slug: request.slug.0,
                    name: request.name.map(|name| name.0),
                    permissions: request.permissions.map(|permissions| permissions.0)
                };
                let url = format!("{}roles/", self.backend);
                reqwest::Client::new()
                    .post(url)
                    .body(serde_json::to_string(&role).unwrap())
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::DeleteUserId(request) => {
                let id = request.id;
                let url = format!("{}users/{}", self.backend, id);
                reqwest::Client::new()
                    .delete(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::DeleteRoleSlug(request) => {
                let slug = request.slug;
                let url = format!("{}roles/{}", self.backend, slug);
                reqwest::Client::new()
                    .delete(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::UpdateUser(request) => {
                let user = user::Model {
                    id: request.id,
                    name: request.name.0
                };
                let url = format!("{}users/{}", self.backend, request.id);
                reqwest::Client::new()
                    .put(url)
                    .body(serde_json::to_string(&user).unwrap())
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::UpdateRole(request) => {
                let role = entities::role::Model {
                    slug: request.slug.0.clone(),
                    name: request.name.map(|name| name.0),
                    permissions: request.permissions.map(|permissions| permissions.0)
                };
                let url = format!("{}roles/{}", self.backend, request.slug.0);
                reqwest::Client::new()
                    .put(url)
                    .body(serde_json::to_string(&role).unwrap())
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::AssignRole(request) => {
                let id = request.id;
                let slug = request.slug.0;
                let url = format!("{}users/{}/assign/{}", self.backend, id, slug);
                reqwest::Client::new()
                    .post(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::UnassignRole(request) => {
                let id = request.id;
                let slug = request.slug.0;
                let url = format!("{}users/{}/unassign/{}", self.backend, id, slug);
                reqwest::Client::new()
                    .post(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::ShowUsers(request) => {
                let url = format!("{}users/", self.backend);
                reqwest::Client::new()
                    .get(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::ShowRoles(request) => {
                let url = format!("{}roles/", self.backend);
                reqwest::Client::new()
                    .get(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::ShowUser(request) => {
                let id = request.id.0;
                let url = format!("{}users/{}", self.backend, id);
                reqwest::Client::new()
                    .get(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
            CliCommand::ShowRole(request) => {
                let slug = request.slug.0;
                let url = format!("{}roles/{}", self.backend, slug);
                reqwest::Client::new()
                    .get(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            }
        }
    }
}