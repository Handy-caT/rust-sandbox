use url::Url;
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
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::AddRole(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::DeleteUserId(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::DeleteRoleSlug(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::UpdateUser(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::UpdateRole(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::AssignRole(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::UnassignRole(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::ShowUsers(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::ShowRoles(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::ShowUser(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
            CliCommand::ShowRole(request) => {
                self.send_json_to_backend_root(serde_json::to_string(&request).unwrap()).await
            }
        }
    }
}