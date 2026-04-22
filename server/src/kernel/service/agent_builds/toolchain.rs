use super::*;

pub(super) async fn detect_host_target_triple() -> anyhow::Result<String> {
    let output = Command::new("rustc").arg("-vV").output().await?;
    if !output.status.success() {
        return Err(anyhow::anyhow!("failed to detect host rust target"));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("host: ").map(str::to_string))
        .ok_or_else(|| anyhow::anyhow!("rustc host triple missing"))
}

pub(super) fn sanitize_target(target_triple: &str) -> String {
    target_triple.replace('/', "-")
}

pub(super) fn build_command_for_target(target_triple: &str, profile: &str) -> Command {
    let mut command = if target_triple.contains("windows-msvc") {
        let mut command = Command::new("cargo");
        command.arg("xwin");
        command.arg("build");
        // Statically link CRT so the binary doesn't depend on vcruntime140.dll
        command.env("RUSTFLAGS", "-C target-feature=+crt-static");
        command
    } else if target_triple.contains("linux-musl") {
        let mut command = Command::new("cargo");
        command.arg("zigbuild");
        command
    } else {
        let mut command = Command::new("cargo");
        command.arg("build");
        command
    };

    if profile == "release" {
        command.arg("--release");
    }
    command.arg("--target").arg(target_triple);
    command
}

pub(super) async fn ensure_target_support(target_triple: &str) -> anyhow::Result<()> {
    if target_triple.contains("windows-msvc") || target_triple.contains("linux-musl") {
        let status = Command::new("rustup")
            .arg("target")
            .arg("add")
            .arg(target_triple)
            .status()
            .await?;
        if !status.success() {
            return Err(anyhow::anyhow!(
                "failed to install rust target {}",
                target_triple
            ));
        }
    }
    Ok(())
}
