use super::*;

use crate::protocol::{ListenerRecord, WebEvent};
use toolchain::{
    build_command_for_target, detect_host_target_triple, ensure_target_support, sanitize_target,
};

const AGENT_SERVER_MODULE_PATH: &str = "src/server.rs";

#[derive(serde::Serialize)]
struct AgentBuildManifest {
    build_id: i64,
    target_triple: String,
    profile: String,
    listener_id: Option<i64>,
    listener_name: Option<String>,
    listener_kind: Option<String>,
    listener_bind: Option<String>,
    embedded_server_addr: String,
    server_addr_binding: &'static str,
    embedded_agent_token: bool,
    artifact_name: String,
    artifact_path: String,
    ignored_runtime_overrides: Vec<&'static str>,
    runtime_overrides: Vec<&'static str>,
}

impl AgentBuildFacade {
    pub async fn build_agent_binary(
        &self,
        target_triple: Option<String>,
        listener_id: Option<i64>,
        server_addr: Option<String>,
        agent_token: Option<String>,
        profile: String,
        heartbeat_secs: Option<u64>,
        jitter: Option<u32>,
    ) -> anyhow::Result<AgentBuildRecord> {
        let target_triple = match target_triple {
            Some(target_triple) => target_triple,
            None => detect_host_target_triple().await?,
        };
        let listener = self.resolve_build_listener(listener_id).await?;
        let listener_bind = listener
            .as_ref()
            .map(|listener| format!("{}:{}", listener.bind_host, listener.bind_port));

        let server_addr = match server_addr {
            Some(server_addr) => server_addr,
            None => {
                let listener_bind = listener_bind.as_ref().ok_or_else(|| {
                    anyhow::anyhow!("no enabled tcp_json listener found for build defaults")
                })?;
                listener_bind.clone()
            }
        };
        self.validate_listener_binding(listener.as_ref(), &server_addr)?;

        let build = self
            .kernel
            .storage
            .create_agent_build_record(
                target_triple.clone(),
                profile.clone(),
                listener.as_ref().map(|item| item.listener_id),
                server_addr.clone(),
                agent_token.is_some(),
            )
            .await?;

        self.kernel
            .publish_web_event(WebEvent::AgentBuildCreated { build: build.clone() });

        let heartbeat_secs = heartbeat_secs.unwrap_or(15);
        let jitter = jitter.unwrap_or(0);

        let kernel = self.kernel.clone();
        let build_id = build.build_id;
        tokio::spawn(async move {
            let result = run_build(
                build_id,
                target_triple,
                server_addr,
                agent_token,
                profile,
                listener,
                heartbeat_secs,
                jitter,
            )
            .await;

            match result {
                Ok((artifact_path, artifact_name, detail)) => {
                    match kernel
                        .storage
                        .update_agent_build_record(
                            build_id,
                            AgentBuildStatus::Succeeded,
                            Some(artifact_path),
                            Some(artifact_name),
                            Some(detail),
                        )
                        .await
                    {
                        Ok(Some(updated)) => {
                            kernel
                                .publish_web_event(WebEvent::AgentBuildCompleted { build: updated });
                        }
                        Ok(None) => {
                            eprintln!(
                                "[agent-build] build {build_id} record missing after success update"
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                "[agent-build] build {build_id} failed to update record: {e}"
                            );
                        }
                    }
                }
                Err(error) => {
                    if let Ok(Some(updated)) = kernel
                        .storage
                        .update_agent_build_record(
                            build_id,
                            AgentBuildStatus::Failed,
                            None,
                            None,
                            Some(error.to_string()),
                        )
                        .await
                    {
                        kernel
                            .publish_web_event(WebEvent::AgentBuildCompleted { build: updated });
                    }
                }
            }
        });

        Ok(build)
    }

    async fn resolve_build_listener(
        &self,
        listener_id: Option<i64>,
    ) -> anyhow::Result<Option<ListenerRecord>> {
        match listener_id {
            Some(listener_id) => Ok(Some(
                self.kernel
                    .listener_queries()
                    .record(listener_id)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("listener {} not found", listener_id))?,
            )),
            None => {
                let tcp = self
                    .kernel
                    .listener_queries()
                    .filtered_records(Some(true), Some(ListenerKind::TcpJson), None)
                    .await?
                    .into_iter()
                    .next();
                if tcp.is_some() {
                    return Ok(tcp);
                }
                Ok(self
                    .kernel
                    .listener_queries()
                    .filtered_records(Some(true), Some(ListenerKind::HttpsJson), None)
                    .await?
                    .into_iter()
                    .next())
            }
        }
    }

    fn validate_listener_binding(
        &self,
        listener: Option<&ListenerRecord>,
        server_addr: &str,
    ) -> anyhow::Result<()> {
        let Some(listener) = listener else {
            return Ok(());
        };
        // Only validate port matches; host can differ (e.g. listener binds 0.0.0.0
        // but agent connects via a specific interface IP like 192.168.x.x)
        let request_port = server_addr.rsplit(':').next();
        if request_port != Some(listener.bind_port.to_string().as_str()) {
            return Err(anyhow::anyhow!(
                "listener {} is bound to port {}, but build requested server_addr={}",
                listener.listener_id,
                listener.bind_port,
                server_addr
            ));
        }
        Ok(())
    }
}

async fn run_build(
    build_id: i64,
    target_triple: String,
    server_addr: String,
    agent_token: Option<String>,
    profile: String,
    listener: Option<ListenerRecord>,
    heartbeat_secs: u64,
    jitter: u32,
) -> anyhow::Result<(String, String, String)> {
    let agent_project_path = PathBuf::from(DEFAULT_AGENT_PROJECT_PATH);
    let artifact_root =
        PathBuf::from(DEFAULT_AGENT_ARTIFACT_DIR).join(format!("build-{build_id}"));
    fs::create_dir_all(&artifact_root)?;

    ensure_target_support(&target_triple).await?;

    let listener_kind = listener.as_ref().map(|l| l.kind.clone());
    let protocol = match listener_kind {
        Some(ListenerKind::HttpsJson) => "tls",
        _ => "tcp",
    };

    let mut command = build_command_for_target(&target_triple, &profile);
    if protocol == "tls" {
        command.args(["--features", "tls"]);
    }
    command.current_dir(&agent_project_path);
    let server_module_path = agent_project_path.join(AGENT_SERVER_MODULE_PATH);
    let previous_server_module = fs::read_to_string(&server_module_path)?;
    fs::write(
        &server_module_path,
        render_server_module(&server_addr, agent_token.as_deref(), protocol, heartbeat_secs, jitter),
    )?;

    let output = command.output().await;
    let restore_result = fs::write(&server_module_path, previous_server_module);
    let output = match (output, restore_result) {
        (Ok(output), Ok(())) => output,
        (Err(build_error), Ok(())) => return Err(build_error.into()),
        (Ok(_), Err(restore_error)) => {
            return Err(anyhow::anyhow!(
                "agent build server module restore failed: {}",
                restore_error
            ));
        }
        (Err(build_error), Err(restore_error)) => {
            return Err(anyhow::anyhow!(
                "agent build failed: {}; server module restore also failed: {}",
                build_error,
                restore_error
            ));
        }
    };
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "agent build failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let binary_name = if target_triple.contains("windows") {
        "agent.exe"
    } else {
        "agent"
    };
    let profile_dir = if profile == "release" {
        "release"
    } else {
        "debug"
    };
    let source_artifact = agent_project_path
        .join("target")
        .join(&target_triple)
        .join(profile_dir)
        .join(binary_name);
    if !source_artifact.exists() {
        return Err(anyhow::anyhow!(
            "expected artifact missing at {}",
            source_artifact.display()
        ));
    }

    let artifact_name = format!("agent-{}-{}", sanitize_target(&target_triple), binary_name);
    let artifact_path = artifact_root.join(&artifact_name);
    fs::copy(&source_artifact, &artifact_path)?;
    let manifest_path = artifact_root.join(format!("{artifact_name}.manifest.json"));
    let manifest = AgentBuildManifest {
        build_id,
        target_triple: target_triple.clone(),
        profile: profile.clone(),
        listener_id: listener.as_ref().map(|item| item.listener_id),
        listener_name: listener.as_ref().map(|item| item.name.clone()),
        listener_kind: listener
            .as_ref()
            .map(|item| format!("{:?}", item.kind).to_lowercase()),
        listener_bind: listener
            .as_ref()
            .map(|item| format!("{}:{}", item.bind_host, item.bind_port)),
        embedded_server_addr: server_addr.clone(),
        server_addr_binding: "compile_time_only",
        embedded_agent_token: agent_token.is_some(),
        artifact_name: artifact_name.clone(),
        artifact_path: artifact_path.display().to_string(),
        ignored_runtime_overrides: vec![
            "HERMES_SERVER_ADDR",
            "HERMES_AGENT_ID",
            "HERMES_AGENT_NAME",
            "HERMES_AGENT_TOKEN",
            "HERMES_HEARTBEAT_SECS",
            "HERMES_JITTER",
            "HERMES_RECONNECT_SECS",
            "HERMES_COMMAND_TIMEOUT_SECS",
        ],
        runtime_overrides: Vec::new(),
    };
    fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest)?)?;

    Ok((
        artifact_path.display().to_string(),
        artifact_name,
        format!(
            "built {} with embedded server_addr={} binding=compile_time_only manifest={}",
            target_triple,
            server_addr,
            manifest_path.display()
        ),
    ))
}

fn render_server_module(
    server_addr: &str,
    agent_token: Option<&str>,
    protocol: &str,
    heartbeat_secs: u64,
    jitter: u32,
) -> String {
    let agent_token = match agent_token {
        Some(agent_token) => format!("Some({:?})", agent_token),
        None => "None".to_string(),
    };
    format!(
        "//! Embedded server connection profile.\n\
         // This file is managed by the server-side build flow.\n\
         // Generated at build time. Do not edit manually during automated builds.\n\n\
         const EMBEDDED_SERVER_ADDR: &str = {:?};\n\
         const EMBEDDED_AGENT_TOKEN: Option<&str> = {};\n\
         const EMBEDDED_PROTOCOL: &str = {:?};\n\
         const EMBEDDED_HEARTBEAT_SECS: u64 = {};\n\
         const EMBEDDED_JITTER: u32 = {};\n\n\
         pub fn get_server_addr() -> String {{\n\
             EMBEDDED_SERVER_ADDR.to_string()\n\
         }}\n\n\
         pub fn get_agent_token() -> Option<String> {{\n\
             EMBEDDED_AGENT_TOKEN.map(str::to_string)\n\
         }}\n\n\
         pub fn get_protocol() -> &'static str {{\n\
             EMBEDDED_PROTOCOL\n\
         }}\n\n\
         pub fn get_heartbeat_secs() -> u64 {{\n\
             EMBEDDED_HEARTBEAT_SECS\n\
         }}\n\n\
         pub fn get_jitter() -> u32 {{\n\
             EMBEDDED_JITTER\n\
         }}\n",
        server_addr, agent_token, protocol, heartbeat_secs, jitter
    )
}
