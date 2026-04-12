use super::*;

impl KernelState {
    pub fn command_session_snapshot(
        &self,
        command_session_id: &str,
    ) -> Option<CommandSessionSnapshot> {
        self.command_sessions
            .get(command_session_id)
            .map(CommandSessionRecord::snapshot)
    }

    pub fn command_session_snapshots(&self) -> Vec<CommandSessionSnapshot> {
        let mut sessions = self
            .command_sessions
            .values()
            .map(CommandSessionRecord::snapshot)
            .collect::<Vec<_>>();
        sessions.sort_by(|a, b| {
            b.created_at
                .cmp(&a.created_at)
                .then_with(|| a.command_session_id.cmp(&b.command_session_id))
        });
        sessions
    }

    pub fn command_execution_snapshot(&self, command_id: &str) -> Option<CommandExecutionSnapshot> {
        self.command_executions
            .get(command_id)
            .map(CommandExecutionRecord::snapshot)
    }

    pub fn command_execution_snapshots_for_session(
        &self,
        command_session_id: &str,
    ) -> Vec<CommandExecutionSnapshot> {
        let mut commands = self
            .command_executions
            .values()
            .filter(|record| record.command_session_id == command_session_id)
            .map(CommandExecutionRecord::snapshot)
            .collect::<Vec<_>>();
        commands.sort_by(|a, b| {
            a.queued_at
                .cmp(&b.queued_at)
                .then_with(|| a.command_id.cmp(&b.command_id))
        });
        commands
    }

    pub fn insert_command_session(
        &mut self,
        command_session_id: String,
        agent_id: String,
        created_by: String,
        created_at: u64,
        sender: oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>,
    ) {
        self.pending_open_command_sessions
            .insert(command_session_id.clone(), sender);
        self.command_sessions.insert(
            command_session_id.clone(),
            CommandSessionRecord {
                command_session_id,
                agent_id,
                cwd: String::new(),
                status: CommandSessionStatus::Closed,
                created_by,
                created_at,
                last_active_at: created_at,
                active_command_id: None,
                queued_command_ids: VecDeque::new(),
            },
        );
    }

    pub fn activate_command_session(
        &mut self,
        command_session_id: &str,
        cwd: String,
        activated_at: u64,
    ) -> Option<CommandSessionSnapshot> {
        let record = self.command_sessions.get_mut(command_session_id)?;
        record.cwd = cwd;
        record.status = CommandSessionStatus::Open;
        record.last_active_at = activated_at;
        let snapshot = record.snapshot();
        if let Some(sender) = self
            .pending_open_command_sessions
            .remove(command_session_id)
        {
            let _ = sender.send(Ok(snapshot.clone()));
        }
        Some(snapshot)
    }

    pub fn register_pending_command_execute(
        &mut self,
        command_id: String,
        sender: oneshot::Sender<anyhow::Result<CommandSessionResult>>,
    ) {
        self.pending_command_executes.insert(
            command_id.clone(),
            PendingCommandExecute { command_id, sender },
        );
    }

    pub fn queue_command_execution(
        &mut self,
        command_id: String,
        command_session_id: &str,
        line: String,
        queued_at: u64,
    ) -> Option<CommandExecutionSnapshot> {
        let record = self.command_sessions.get_mut(command_session_id)?;
        let command = CommandExecutionRecord {
            command_id: command_id.clone(),
            command_session_id: command_session_id.to_string(),
            agent_id: record.agent_id.clone(),
            line,
            status: CommandExecutionStatus::Queued,
            queued_at,
            updated_at: queued_at,
            dispatched_at: None,
            started_at: None,
            finished_at: None,
            cwd_before: None,
            cwd_after: None,
            exit_code: None,
            stdout: None,
            stderr: None,
            success: None,
        };
        record.queued_command_ids.push_back(command_id.clone());
        self.command_executions.insert(command_id.clone(), command);
        self.command_execution_snapshot(&command_id)
    }

    pub fn next_queued_command_for_session(
        &self,
        command_session_id: &str,
    ) -> Option<CommandExecutionSnapshot> {
        let record = self.command_sessions.get(command_session_id)?;
        let command_id = record.queued_command_ids.front()?;
        self.command_execution_snapshot(command_id)
    }

    pub fn mark_command_dispatched(
        &mut self,
        command_session_id: &str,
        command_id: &str,
        dispatched_at: u64,
    ) -> Option<CommandExecutionSnapshot> {
        let record = self.command_sessions.get_mut(command_session_id)?;
        if record.active_command_id.is_some() {
            return None;
        }
        let front = record.queued_command_ids.front()?;
        if front != command_id {
            return None;
        }
        record.queued_command_ids.pop_front();
        record.active_command_id = Some(command_id.to_string());

        let command = self.command_executions.get_mut(command_id)?;
        command.status = CommandExecutionStatus::Dispatched;
        command.dispatched_at = Some(dispatched_at);
        command.updated_at = dispatched_at;
        Some(command.snapshot())
    }

    pub fn mark_command_running(
        &mut self,
        command_session_id: &str,
        command_id: &str,
        started_at: u64,
    ) -> Option<CommandExecutionSnapshot> {
        let record = self.command_sessions.get(command_session_id)?;
        if record.active_command_id.as_deref() != Some(command_id) {
            return None;
        }
        let command = self.command_executions.get_mut(command_id)?;
        command.status = CommandExecutionStatus::Running;
        command.started_at = Some(started_at);
        command.updated_at = started_at;
        Some(command.snapshot())
    }

    pub fn append_command_output_chunk(
        &mut self,
        command_session_id: &str,
        command_id: &str,
        stream: &CommandOutputStream,
        chunk: &str,
        updated_at: u64,
    ) -> Option<CommandExecutionSnapshot> {
        let record = self.command_sessions.get(command_session_id)?;
        if record.active_command_id.as_deref() != Some(command_id) {
            return None;
        }
        let command = self.command_executions.get_mut(command_id)?;
        if !matches!(
            command.status,
            CommandExecutionStatus::Dispatched | CommandExecutionStatus::Running
        ) {
            return None;
        }
        match stream {
            CommandOutputStream::Stdout => {
                command
                    .stdout
                    .get_or_insert_with(String::new)
                    .push_str(chunk);
            }
            CommandOutputStream::Stderr => {
                command
                    .stderr
                    .get_or_insert_with(String::new)
                    .push_str(chunk);
            }
        }
        command.updated_at = updated_at;
        Some(command.snapshot())
    }

    pub fn finish_command_execute(
        &mut self,
        command_session_id: &str,
        command_id: &str,
        line: String,
        cwd_before: String,
        cwd_after: String,
        exit_code: i32,
        stdout: String,
        stderr: String,
        success: bool,
        finished_at: u64,
    ) -> Option<CommandSessionResult> {
        let record = self.command_sessions.get_mut(command_session_id)?;
        let command = self.command_executions.get_mut(command_id)?;
        if !matches!(
            command.status,
            CommandExecutionStatus::Dispatched | CommandExecutionStatus::Running
        ) {
            return None;
        }
        if record.active_command_id.as_deref() == Some(command_id) {
            record.active_command_id = None;
        }
        record.cwd = cwd_after.clone();
        record.last_active_at = finished_at;
        let status = if success {
            CommandExecutionStatus::Succeeded
        } else {
            CommandExecutionStatus::Failed
        };
        command.status = status;
        command.updated_at = finished_at;
        command.finished_at = Some(finished_at);
        command.cwd_before = Some(cwd_before.clone());
        command.cwd_after = Some(cwd_after.clone());
        command.exit_code = Some(exit_code);
        command.stdout = Some(stdout.clone());
        command.stderr = Some(stderr.clone());
        command.success = Some(success);
        let result = CommandSessionResult {
            command_session_id: command_session_id.to_string(),
            agent_id: record.agent_id.clone(),
            request_id: command_id.to_string(),
            line,
            cwd_before,
            cwd_after,
            exit_code,
            stdout,
            stderr,
            success,
            finished_at,
        };
        if let Some(pending) = self.pending_command_executes.remove(command_id) {
            let _ = pending.sender.send(Ok(result.clone()));
        }
        Some(result)
    }

    pub fn cancel_command_execution(
        &mut self,
        command_id: &str,
        reason: String,
        finished_at: u64,
    ) -> Option<CommandExecutionSnapshot> {
        let command = self.command_executions.get_mut(command_id)?;
        if !matches!(
            command.status,
            CommandExecutionStatus::Queued
                | CommandExecutionStatus::Dispatched
                | CommandExecutionStatus::Running
        ) {
            return None;
        }
        command.status = CommandExecutionStatus::Cancelled;
        command.updated_at = finished_at;
        command.finished_at = Some(finished_at);
        command.stderr = Some(reason);
        command.success = Some(false);
        Some(command.snapshot())
    }

    pub fn drop_command_execution(
        &mut self,
        command_id: &str,
        reason: String,
        finished_at: u64,
    ) -> Option<CommandExecutionSnapshot> {
        let command = self.command_executions.get_mut(command_id)?;
        if !matches!(
            command.status,
            CommandExecutionStatus::Queued
                | CommandExecutionStatus::Dispatched
                | CommandExecutionStatus::Running
        ) {
            return None;
        }
        command.status = CommandExecutionStatus::Dropped;
        command.updated_at = finished_at;
        command.finished_at = Some(finished_at);
        command.stderr = Some(reason);
        command.success = Some(false);
        Some(command.snapshot())
    }

    pub fn abort_pending_open_command_session(&mut self, command_session_id: &str) -> bool {
        let removed = self
            .pending_open_command_sessions
            .remove(command_session_id)
            .is_some();
        if removed {
            self.command_sessions.remove(command_session_id);
        }
        removed
    }

    pub fn abort_pending_command_execute(&mut self, command_id: &str) -> bool {
        self.pending_command_executes.remove(command_id).is_some()
    }

    pub fn abort_pending_close_command_session(&mut self, command_session_id: &str) -> bool {
        self.pending_close_command_sessions
            .remove(command_session_id)
            .is_some()
    }

    pub fn register_pending_close_command_session(
        &mut self,
        command_session_id: String,
        sender: oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>,
    ) {
        self.pending_close_command_sessions
            .insert(command_session_id, sender);
    }

    pub fn close_command_session(
        &mut self,
        command_session_id: &str,
        closed_at: u64,
    ) -> Option<CommandSessionSnapshot> {
        let record = self.command_sessions.get_mut(command_session_id)?;
        record.status = CommandSessionStatus::Closed;
        record.last_active_at = closed_at;
        record.active_command_id = None;
        record.queued_command_ids.clear();
        let snapshot = record.snapshot();
        if let Some(sender) = self
            .pending_open_command_sessions
            .remove(command_session_id)
        {
            let _ = sender.send(Err(anyhow::anyhow!(format!(
                "command session {} closed before open confirmation",
                command_session_id
            ))));
        }
        if let Some(sender) = self
            .pending_close_command_sessions
            .remove(command_session_id)
        {
            let _ = sender.send(Ok(snapshot.clone()));
        }
        Some(snapshot)
    }

    pub fn close_command_sessions_for_agent(
        &mut self,
        agent_id: &str,
        closed_at: u64,
    ) -> Vec<CommandSessionSnapshot> {
        let ids = self
            .command_sessions
            .values()
            .filter(|record| {
                record.agent_id == agent_id && matches!(record.status, CommandSessionStatus::Open)
            })
            .map(|record| record.command_session_id.clone())
            .collect::<Vec<_>>();
        ids.into_iter()
            .filter_map(|id| self.close_command_session(&id, closed_at))
            .collect()
    }

    pub fn fail_pending_command_sessions_for_agent(&mut self, agent_id: &str, reason: &str) {
        let command_session_ids = self
            .command_sessions
            .values()
            .filter(|record| record.agent_id == agent_id)
            .map(|record| record.command_session_id.clone())
            .collect::<HashSet<_>>();

        for command_session_id in &command_session_ids {
            if let Some(sender) = self
                .pending_open_command_sessions
                .remove(command_session_id)
            {
                let _ = sender.send(Err(anyhow::anyhow!(reason.to_string())));
            }
            if let Some(sender) = self
                .pending_close_command_sessions
                .remove(command_session_id)
            {
                let _ = sender.send(Err(anyhow::anyhow!(reason.to_string())));
            }
        }

        let pending_execute_ids = self
            .pending_command_executes
            .iter()
            .filter(|(_, pending)| {
                self.command_executions
                    .get(&pending.command_id)
                    .map(|command| command_session_ids.contains(&command.command_session_id))
                    .unwrap_or(false)
            })
            .map(|(command_id, _)| command_id.clone())
            .collect::<Vec<_>>();

        for command_id in pending_execute_ids {
            if let Some(pending) = self.pending_command_executes.remove(&command_id) {
                let _ = pending
                    .sender
                    .send(Err(anyhow::anyhow!(reason.to_string())));
            }
        }
    }

    pub fn fail_pending_command_execute(&mut self, command_id: &str, reason: &str) {
        if let Some(pending) = self.pending_command_executes.remove(command_id) {
            let _ = pending
                .sender
                .send(Err(anyhow::anyhow!(reason.to_string())));
        }
    }

    pub fn command_session_active_command_id(&self, command_session_id: &str) -> Option<String> {
        self.command_sessions
            .get(command_session_id)
            .and_then(|record| record.active_command_id.clone())
    }

    pub fn drain_command_session_queue(&mut self, command_session_id: &str) -> Vec<String> {
        let Some(record) = self.command_sessions.get_mut(command_session_id) else {
            return Vec::new();
        };
        record.queued_command_ids.drain(..).collect()
    }

    pub fn clear_active_command_for_session(&mut self, command_session_id: &str) -> Option<String> {
        let record = self.command_sessions.get_mut(command_session_id)?;
        record.active_command_id.take()
    }

    pub fn command_session_ids_for_agent(&self, agent_id: &str) -> Vec<String> {
        self.command_sessions
            .values()
            .filter(|record| record.agent_id == agent_id)
            .map(|record| record.command_session_id.clone())
            .collect()
    }
}

impl CommandSessionRecord {
    fn snapshot(&self) -> CommandSessionSnapshot {
        CommandSessionSnapshot {
            command_session_id: self.command_session_id.clone(),
            agent_id: self.agent_id.clone(),
            cwd: self.cwd.clone(),
            status: self.status.clone(),
            created_by: self.created_by.clone(),
            created_at: self.created_at,
            last_active_at: self.last_active_at,
        }
    }
}

impl CommandExecutionRecord {
    fn snapshot(&self) -> CommandExecutionSnapshot {
        CommandExecutionSnapshot {
            command_id: self.command_id.clone(),
            command_session_id: self.command_session_id.clone(),
            agent_id: self.agent_id.clone(),
            line: self.line.clone(),
            status: self.status.clone(),
            queued_at: self.queued_at,
            updated_at: self.updated_at,
            dispatched_at: self.dispatched_at,
            started_at: self.started_at,
            finished_at: self.finished_at,
            cwd_before: self.cwd_before.clone(),
            cwd_after: self.cwd_after.clone(),
            exit_code: self.exit_code,
            stdout: self.stdout.clone(),
            stderr: self.stderr.clone(),
            success: self.success,
        }
    }
}
