mod agent;
mod api;
mod console;
mod kernel;
mod protocol;

use agent::gateway::run_agent_gateway;
use api::run_http_api;
use kernel::{AgentAuthMode, Config, new_kernel};

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let config = Config::get_config()?;
    let api_token = config.api_token().map(ToString::to_string);
    let agent_token = config.agent_token().map(ToString::to_string);
    let web_username = config.web_username().map(ToString::to_string);
    let web_password = config.web_password().map(ToString::to_string);
    let session_ttl_secs = config.session_ttl_secs();
    let kernel = new_kernel(
        1024,
        512,
        config.sqlite_path(),
        api_token.clone(),
        web_username.clone(),
        web_password.clone(),
        session_ttl_secs,
    )
    .await?;
    let agent_auth_mode = match config.agent_token() {
        Some(_) => config.agent_auth_mode(),
        None => AgentAuthMode::PlainToken,
    };

    tokio::try_join!(
        run_agent_gateway(
            kernel.clone(),
            config.tcp_addr(),
            agent_token,
            agent_auth_mode
        ),
        run_http_api(kernel, config.api_addr(),)
    )?;

    Ok(())
}
