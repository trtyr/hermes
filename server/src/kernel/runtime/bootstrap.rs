use super::*;

pub async fn new_kernel(
    buffer_size: usize,
    event_buffer_size: usize,
    sqlite_path: impl Into<std::path::PathBuf>,
    api_token: Option<String>,
    web_username: Option<String>,
    web_password: Option<String>,
    session_ttl_secs: u64,
) -> anyhow::Result<KernelHandle> {
    let storage = Storage::new(sqlite_path).await?;
    let bootstrap = storage.bootstrap().await?;
    let (bus, receiver) = KernelBus::new(buffer_size);
    let (events, _) = broadcast::channel(event_buffer_size);
    let mut kernel_state = KernelState::new();
    kernel_state.load_tasks(bootstrap.tasks);
    let state = Arc::new(RwLock::new(kernel_state));

    let handle = KernelHandle::new(
        bus,
        state.clone(),
        events.clone(),
        storage,
        AuthService::new(api_token, web_username, web_password, session_ttl_secs),
        Arc::new(AtomicU64::new(1)),
        Arc::new(AtomicU64::new(bootstrap.next_task_seq)),
        Arc::new(AtomicU64::new(1)),
        Arc::new(AtomicU64::new(1)),
        Arc::new(AtomicU64::new(1)),
    );

    tokio::spawn(dispatch::kernel_loop(
        receiver,
        state,
        events,
        handle.storage().clone(),
    ));
    tokio::spawn(watchdog::heartbeat_watchdog(handle.clone()));

    Ok(handle)
}
