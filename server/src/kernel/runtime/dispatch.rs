use super::*;

pub(super) async fn kernel_loop(
    mut receiver: mpsc::Receiver<KernelMessage>,
    state: Arc<RwLock<KernelState>>,
    events: EventBus,
    storage: Storage,
) {
    let effects = RuntimePorts::new(events, storage);
    while let Some(message) = receiver.recv().await {
        match message {
            KernelMessage::Agent(agent_message) => {
                route_agent_message(&state, &effects, agent_message).await;
            }
            KernelMessage::Task(task_message) => {
                route_task_message(&state, &effects, task_message).await;
            }
            KernelMessage::CommandSession(session_message) => {
                route_command_session_message(&state, &effects, session_message).await;
            }
        }
    }
}

async fn route_agent_message(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    message: AgentKernelMessage,
) {
    match message {
        AgentKernelMessage::Connected {
            session_id,
            listener_id,
            listener_name,
            peer_addr,
            sender,
        } => {
            agent_lifecycle::handle_agent_connected(
                state,
                effects,
                session_id,
                listener_id,
                listener_name,
                peer_addr,
                sender,
            )
            .await
        }
        AgentKernelMessage::Disconnected { session_id } => {
            agent_lifecycle::handle_agent_disconnected(state, effects, session_id).await;
        }
        AgentKernelMessage::Frame { session_id, frame } => {
            agent_lifecycle::handle_agent_frame(state, effects, session_id, frame).await;
        }
        AgentKernelMessage::UpdateBeaconConfig {
            agent_id,
            request_id,
            sleep_interval,
            jitter,
            respond_to,
        } => {
            agent_lifecycle::update_beacon_config(
                state,
                effects,
                agent_id,
                request_id,
                sleep_interval,
                jitter,
                respond_to,
            )
            .await;
        }
        AgentKernelMessage::SweepHeartbeats => {
            agent_lifecycle::sweep_heartbeats(state, effects).await;
        }
    }
}

async fn route_task_message(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    message: TaskKernelMessage,
) {
    match message {
        TaskKernelMessage::Dispatch {
            target_agent_id,
            task_id,
            command,
            payload,
        } => {
            task_flow::dispatch_task(state, effects, target_agent_id, task_id, command, payload)
                .await;
        }
        TaskKernelMessage::Broadcast {
            task_id,
            command,
            payload,
        } => {
            task_flow::broadcast_task(state, effects, task_id, command, payload).await;
        }
        TaskKernelMessage::Cancel { task_id } => {
            task_flow::cancel_task(state, effects, task_id).await;
        }
        TaskKernelMessage::DisconnectAgent { agent_id } => {
            agent_lifecycle::disconnect_agent(state, effects, agent_id).await;
        }
    }
}

async fn route_command_session_message(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    message: CommandSessionKernelMessage,
) {
    match message {
        CommandSessionKernelMessage::Open {
            agent_id,
            command_session_id,
            created_by,
            respond_to,
        } => {
            command_sessions::open_command_session(
                state,
                effects,
                agent_id,
                command_session_id,
                created_by,
                respond_to,
            )
            .await;
        }
        CommandSessionKernelMessage::Execute {
            command_session_id,
            command_id,
            line,
            respond_to,
        } => {
            command_sessions::execute_command_session(
                state,
                effects,
                command_session_id,
                command_id,
                line,
                respond_to,
            )
            .await;
        }
        CommandSessionKernelMessage::Queue {
            command_session_id,
            command_id,
            line,
            respond_to,
        } => {
            command_sessions::queue_command_session(
                state,
                effects,
                command_session_id,
                command_id,
                line,
                respond_to,
            )
            .await;
        }
        CommandSessionKernelMessage::Close {
            command_session_id,
            respond_to,
        } => {
            command_sessions::close_command_session(state, effects, command_session_id, respond_to)
                .await;
        }
    }
}
