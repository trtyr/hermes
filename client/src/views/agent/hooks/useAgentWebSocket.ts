import { onMounted, onBeforeUnmount, type Ref } from 'vue';
import { useEventStore } from '@/store/events';
import type { Agent } from '@/api/agent';

export function useAgentWebSocket(
  agents: Ref<Agent[]>, 
  selectedAgent: Ref<Agent | null>, 
  detailVisible: Ref<boolean>,
  loadAgents: () => void
) {
  const eventStore = useEventStore();
  let unsubscribe: () => void;

  onMounted(() => {
    unsubscribe = eventStore.subscribe((event) => {
      if (event.type === 'snapshot') {
        const onlineIds = new Set(event.agents.map(a => a.agent_id));
        agents.value.forEach(agent => {
          if (onlineIds.has(agent.agent_id)) {
            agent.is_online = true;
            const snap = event.agents.find(a => a.agent_id === agent.agent_id);
            if (snap) {
              agent.last_seen = snap.last_seen;
              agent.peer_addr = snap.peer_addr;
            }
          } 
        });
      } else if (event.type === 'agent_connected' || event.type === 'agent_registered') {
        loadAgents();
      } else if (event.type === 'agent_disconnected') {
        const target = agents.value.find(a => a.agent_id === event.agent_id);
        if (target) target.is_online = false;
        if (selectedAgent.value?.agent_id === event.agent_id) {
          selectedAgent.value.is_online = false;
        }
      } else if (event.type === 'agent_heartbeat') {
        const target = agents.value.find(a => a.agent_id === event.agent_id);
        if (target) {
          target.is_online = true;
          target.last_seen = event.last_seen;
        }
        if (selectedAgent.value?.agent_id === event.agent_id) {
          selectedAgent.value.is_online = true;
          selectedAgent.value.last_seen = event.last_seen;
        }
      } else if (event.type === 'agent_updated') {
        const idx = agents.value.findIndex(a => a.agent_id === event.agent.agent_id);
        if (idx !== -1) {
          agents.value[idx] = event.agent;
        }
        if (selectedAgent.value?.agent_id === event.agent.agent_id) {
          selectedAgent.value = event.agent;
        }
      } else if (event.type === 'agent_disabled') {
        const target = agents.value.find(a => a.agent_id === event.agent_id);
        if (target) target.is_disabled = true;
      } else if (event.type === 'agent_enabled') {
        const target = agents.value.find(a => a.agent_id === event.agent_id);
        if (target) target.is_disabled = false;
      } else if (event.type === 'agent_deleted') {
        agents.value = agents.value.filter(a => a.agent_id !== event.agent_id);
        if (selectedAgent.value?.agent_id === event.agent_id) {
          detailVisible.value = false;
        }
      }
    });
  });

  onBeforeUnmount(() => {
    if (unsubscribe) {
      unsubscribe();
    }
  });
}
