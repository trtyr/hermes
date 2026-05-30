use super::*;

impl KernelState {
    pub fn load_tasks(&mut self, tasks: Vec<TaskSnapshot>) {
        self.tasks = tasks
            .into_iter()
            .map(|task| (task.task_id.clone(), TaskRecord::from_snapshot(task)))
            .collect();
    }

    pub fn create_task(&mut self, task: NewTask) -> TaskSnapshot {
        let snapshot = TaskSnapshot {
            task_id: task.task_id.clone(),
            parent_task_id: task.parent_task_id.clone(),
            target_agent_id: task.target_agent_id.clone(),
            command: task.command.clone(),
            payload: task.payload.clone(),
            status: TaskStatus::Pending,
            created_at: task.created_at,
            updated_at: task.created_at,
            success: None,
            output: None,
            children: Vec::new(),
        };

        self.tasks.insert(
            task.task_id.clone(),
            TaskRecord::from_snapshot(snapshot.clone()),
        );

        if let Some(parent_task_id) = task.parent_task_id {
            if let Some(parent) = self.tasks.get_mut(&parent_task_id) {
                parent.children.push(task.task_id);
                parent.updated_at = task.created_at;
            }
            self.refresh_parent_status(&parent_task_id);
        }

        snapshot
    }

    pub fn mark_task_dispatched(&mut self, task_id: &str, updated_at: u64) -> Option<TaskSnapshot> {
        let task = self.tasks.get_mut(task_id)?;
        if !matches!(task.status, TaskStatus::Pending | TaskStatus::Dispatched) {
            return None;
        }
        task.status = TaskStatus::Dispatched;
        task.updated_at = updated_at;
        let snapshot = task.snapshot();
        let parent_task_id = task.parent_task_id.clone();
        let _ = task;
        if let Some(parent_task_id) = parent_task_id {
            self.refresh_parent_status(&parent_task_id);
        }
        Some(snapshot)
    }

    pub fn mark_task_running(&mut self, task_id: &str, updated_at: u64) -> Option<TaskSnapshot> {
        let task = self.tasks.get_mut(task_id)?;
        if !matches!(
            task.status,
            TaskStatus::Pending | TaskStatus::Dispatched | TaskStatus::Running
        ) {
            return None;
        }
        task.status = TaskStatus::Running;
        task.updated_at = updated_at;
        let snapshot = task.snapshot();
        let parent_task_id = task.parent_task_id.clone();
        let _ = task;
        if let Some(parent_task_id) = parent_task_id {
            self.refresh_parent_status(&parent_task_id);
        }
        Some(snapshot)
    }

    pub fn mark_task_cancel_requested(
        &mut self,
        task_id: &str,
        output: Option<String>,
        updated_at: u64,
    ) -> Option<TaskSnapshot> {
        let task = self.tasks.get_mut(task_id)?;
        if !matches!(
            task.status,
            TaskStatus::Dispatched | TaskStatus::Running | TaskStatus::CancelRequested
        ) {
            return None;
        }
        task.status = TaskStatus::CancelRequested;
        task.updated_at = updated_at;
        task.success = None;
        if output.is_some() {
            task.output = output;
        }
        let snapshot = task.snapshot();
        let parent_task_id = task.parent_task_id.clone();
        let _ = task;
        if let Some(parent_task_id) = parent_task_id {
            self.refresh_parent_status(&parent_task_id);
        }
        Some(snapshot)
    }

    pub fn mark_task_failed(
        &mut self,
        task_id: &str,
        output: String,
        updated_at: u64,
    ) -> Option<TaskSnapshot> {
        let task = self.tasks.get_mut(task_id)?;
        if !matches!(
            task.status,
            TaskStatus::Pending
                | TaskStatus::Dispatched
                | TaskStatus::Running
                | TaskStatus::CancelRequested
        ) {
            return None;
        }
        task.status = TaskStatus::Failed;
        task.updated_at = updated_at;
        task.success = Some(false);
        task.output = Some(output);
        let snapshot = task.snapshot();
        let parent_task_id = task.parent_task_id.clone();
        let _ = task;
        if let Some(parent_task_id) = parent_task_id {
            self.refresh_parent_status(&parent_task_id);
        }
        Some(snapshot)
    }

    pub fn complete_task(
        &mut self,
        task_id: &str,
        success: bool,
        output: String,
        updated_at: u64,
    ) -> Option<TaskSnapshot> {
        let task = self.tasks.get_mut(task_id)?;
        if !matches!(
            task.status,
            TaskStatus::Pending
                | TaskStatus::Dispatched
                | TaskStatus::Running
                | TaskStatus::CancelRequested
        ) {
            return None;
        }
        task.status = if success {
            TaskStatus::Succeeded
        } else {
            TaskStatus::Failed
        };
        task.updated_at = updated_at;
        task.success = Some(success);
        task.output = Some(output);
        let snapshot = task.snapshot();
        let parent_task_id = task.parent_task_id.clone();
        let _ = task;
        if let Some(parent_task_id) = parent_task_id {
            self.refresh_parent_status(&parent_task_id);
        }
        Some(snapshot)
    }

    pub fn buffer_task_chunk(
        &mut self,
        task_id: String,
        chunk_index: u32,
        total_chunks: u32,
        data: String,
    ) {
        let entry = self.pending_task_chunks.entry(task_id).or_default();
        entry.push((chunk_index, data));
        entry.sort_by_key(|(idx, _)| *idx);
        entry.dedup_by_key(|(idx, _)| *idx);
        entry.truncate(total_chunks as usize);
    }

    pub fn assemble_task_chunks(&mut self, task_id: &str) -> String {
        let chunks = self.pending_task_chunks.remove(task_id).unwrap_or_default();
        let mut output = String::new();
        for (_, data) in chunks {
            output.push_str(&data);
        }
        output
    }

    pub fn mark_task_cancelled(
        &mut self,
        task_id: &str,
        output: Option<String>,
        updated_at: u64,
    ) -> Option<TaskSnapshot> {
        let task = self.tasks.get_mut(task_id)?;
        if !matches!(
            task.status,
            TaskStatus::Pending
                | TaskStatus::Dispatched
                | TaskStatus::Running
                | TaskStatus::CancelRequested
        ) {
            return None;
        }
        task.status = TaskStatus::Cancelled;
        task.updated_at = updated_at;
        task.success = Some(false);
        if output.is_some() {
            task.output = output;
        }
        let snapshot = task.snapshot();
        let parent_task_id = task.parent_task_id.clone();
        let _ = task;
        if let Some(parent_task_id) = parent_task_id {
            self.refresh_parent_status(&parent_task_id);
        }
        Some(snapshot)
    }

    pub fn child_task_ids(&self, task_id: &str) -> Vec<String> {
        self.tasks
            .get(task_id)
            .map(|task| task.children.clone())
            .unwrap_or_default()
    }

    pub fn recover_interrupted_tasks(
        &mut self,
        updated_at: u64,
        reason: &str,
    ) -> Vec<TaskSnapshot> {
        let recoverable = self
            .tasks
            .values()
            .filter(|task| {
                matches!(
                    task.status,
                    TaskStatus::Dispatched | TaskStatus::Running | TaskStatus::CancelRequested
                ) && (task.target_agent_id.is_some() || task.children.is_empty())
            })
            .map(|task| task.task_id.clone())
            .collect::<Vec<_>>();

        let mut changed = Vec::new();
        for task_id in recoverable {
            if let Some(task) = self.mark_task_failed(&task_id, reason.to_string(), updated_at) {
                changed.push(task);
            }
        }

        let parent_ids = self
            .tasks
            .values()
            .filter(|task| task.target_agent_id.is_none() && !task.children.is_empty())
            .map(|task| task.task_id.clone())
            .collect::<Vec<_>>();

        for task_id in parent_ids {
            if let Some(task) = self.task_snapshot(&task_id) {
                changed.push(task);
            }
        }

        changed
    }

    pub fn active_task_ids_for_agent(&self, agent_id: &str) -> Vec<String> {
        self.tasks
            .values()
            .filter(|task| {
                task.target_agent_id.as_deref() == Some(agent_id)
                    && matches!(
                        task.status,
                        TaskStatus::Dispatched | TaskStatus::Running | TaskStatus::CancelRequested
                    )
            })
            .map(|task| task.task_id.clone())
            .collect()
    }

    pub fn pending_task_ids_for_agent(&self, agent_id: &str) -> Vec<String> {
        let mut tasks = self
            .tasks
            .values()
            .filter(|task| {
                task.target_agent_id.as_deref() == Some(agent_id)
                    && matches!(task.status, TaskStatus::Pending)
            })
            .collect::<Vec<_>>();

        tasks.sort_by(|a, b| {
            a.created_at
                .cmp(&b.created_at)
                .then_with(|| a.task_id.cmp(&b.task_id))
        });

        tasks.into_iter().map(|task| task.task_id.clone()).collect()
    }

    pub fn is_task_cancellable(&self, task_id: &str) -> bool {
        self.tasks
            .get(task_id)
            .map(|task| {
                matches!(
                    task.status,
                    TaskStatus::Pending | TaskStatus::Dispatched | TaskStatus::Running
                )
            })
            .unwrap_or(false)
    }

    pub fn task_snapshot(&self, task_id: &str) -> Option<TaskSnapshot> {
        self.tasks.get(task_id).map(TaskRecord::snapshot)
    }

    pub fn parent_task_snapshot(&self, task_id: &str) -> Option<TaskSnapshot> {
        let parent_task_id = self.tasks.get(task_id)?.parent_task_id.clone()?;
        self.task_snapshot(&parent_task_id)
    }

    pub fn task_snapshots(&self) -> Vec<TaskSnapshot> {
        let mut tasks = self
            .tasks
            .values()
            .map(TaskRecord::snapshot)
            .collect::<Vec<_>>();
        tasks.sort_by(|a, b| {
            b.created_at
                .cmp(&a.created_at)
                .then_with(|| a.task_id.cmp(&b.task_id))
        });
        tasks
    }

    fn refresh_parent_status(&mut self, parent_task_id: &str) {
        let Some(children) = self
            .tasks
            .get(parent_task_id)
            .map(|task| task.children.clone())
        else {
            return;
        };
        if children.is_empty() {
            return;
        }

        let child_snapshots = children
            .iter()
            .filter_map(|child_id| self.tasks.get(child_id).map(TaskRecord::snapshot))
            .collect::<Vec<_>>();
        if child_snapshots.is_empty() {
            return;
        }

        let all_succeeded = child_snapshots
            .iter()
            .all(|task| matches!(task.status, TaskStatus::Succeeded));
        let all_failed = child_snapshots
            .iter()
            .all(|task| matches!(task.status, TaskStatus::Failed));
        let all_cancelled = child_snapshots
            .iter()
            .all(|task| matches!(task.status, TaskStatus::Cancelled));
        let all_cancel_requested = child_snapshots
            .iter()
            .all(|task| matches!(task.status, TaskStatus::CancelRequested));
        let all_active = child_snapshots.iter().all(|task| {
            matches!(
                task.status,
                TaskStatus::Pending
                    | TaskStatus::Dispatched
                    | TaskStatus::Running
                    | TaskStatus::CancelRequested
            )
        });
        let any_cancel_requested = child_snapshots
            .iter()
            .any(|task| matches!(task.status, TaskStatus::CancelRequested));
        let any_running = child_snapshots
            .iter()
            .any(|task| matches!(task.status, TaskStatus::Running));
        let any_dispatched = child_snapshots
            .iter()
            .any(|task| matches!(task.status, TaskStatus::Dispatched));

        if let Some(parent) = self.tasks.get_mut(parent_task_id) {
            parent.updated_at = child_snapshots
                .iter()
                .map(|task| task.updated_at)
                .max()
                .unwrap_or(parent.updated_at);

            parent.status = if all_succeeded {
                parent.success = Some(true);
                TaskStatus::Succeeded
            } else if all_failed {
                parent.success = Some(false);
                TaskStatus::Failed
            } else if all_cancelled {
                parent.success = Some(false);
                TaskStatus::Cancelled
            } else if all_cancel_requested {
                parent.success = None;
                TaskStatus::CancelRequested
            } else if all_active {
                parent.success = None;
                if any_cancel_requested {
                    TaskStatus::CancelRequested
                } else if any_running {
                    TaskStatus::Running
                } else if any_dispatched {
                    TaskStatus::Dispatched
                } else {
                    TaskStatus::Pending
                }
            } else {
                parent.success = None;
                TaskStatus::Partial
            };
        }
    }
}

impl TaskRecord {
    fn from_snapshot(snapshot: TaskSnapshot) -> Self {
        Self {
            task_id: snapshot.task_id,
            parent_task_id: snapshot.parent_task_id,
            target_agent_id: snapshot.target_agent_id,
            command: snapshot.command,
            payload: snapshot.payload,
            status: snapshot.status,
            created_at: snapshot.created_at,
            updated_at: snapshot.updated_at,
            success: snapshot.success,
            output: snapshot.output,
            children: snapshot.children,
        }
    }

    fn snapshot(&self) -> TaskSnapshot {
        TaskSnapshot {
            task_id: self.task_id.clone(),
            parent_task_id: self.parent_task_id.clone(),
            target_agent_id: self.target_agent_id.clone(),
            command: self.command.clone(),
            payload: self.payload.clone(),
            status: self.status.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            success: self.success,
            output: self.output.clone(),
            children: self.children.clone(),
        }
    }
}
