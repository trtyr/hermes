use super::*;

impl KernelHandle {
    pub fn agent_queries(&self) -> AgentQueryFacade {
        AgentQueryFacade {
            kernel: self.clone(),
        }
    }

    pub fn agent_commands(&self) -> AgentCommandFacade {
        AgentCommandFacade {
            kernel: self.clone(),
        }
    }

    pub fn auth(&self) -> AuthFacade {
        AuthFacade {
            kernel: self.clone(),
        }
    }

    pub fn tasks(&self) -> TaskFacade {
        TaskFacade {
            kernel: self.clone(),
        }
    }

    pub fn command_sessions(&self) -> CommandSessionFacade {
        CommandSessionFacade {
            kernel: self.clone(),
        }
    }

    pub fn agent_builds(&self) -> AgentBuildFacade {
        AgentBuildFacade {
            kernel: self.clone(),
        }
    }

    pub fn listener_queries(&self) -> ListenerQueryFacade {
        ListenerQueryFacade {
            kernel: self.clone(),
        }
    }

    pub fn listener_commands(&self) -> ListenerCommandFacade {
        ListenerCommandFacade {
            kernel: self.clone(),
        }
    }

    pub fn proxy(&self) -> ProxyFacade {
        ProxyFacade {
            kernel: self.clone(),
        }
    }

    pub fn events(&self) -> EventBus {
        self.events.clone()
    }

    pub fn publish_web_event(&self, event: WebEvent) {
        if let Ok(payload) = serde_json::to_string(&event) {
            let _ = self.events.send(payload);
        }
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn agent_auth_config(
        &self,
    ) -> &Arc<std::sync::RwLock<crate::kernel::config::AgentAuthConfig>> {
        &self.agent_auth_config
    }
}
