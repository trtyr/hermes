pub(super) const AGENT_PROTOCOL_VERSION: u32 = 2;
pub const TRANSPORT_PROFILE_TCP_JSON_V1: &str = "tcp_json_v1";
pub const TRANSPORT_PROFILE_HTTPS_JSON_V1: &str = "https_json_v1";
pub(super) const TRANSPORT_CAPABILITIES: &[&str] = &[
    "register",
    "beacon",
    "task_dispatch",
    "beacon_config",
    "command_session_queue",
];
