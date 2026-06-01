import { ref, type Ref } from 'vue';
import type { Terminal } from '@xterm/xterm';
import { message } from '@/utils/message';
import { buildTerminalWsUrl } from '@/api/terminal';

export function useTerminalSocket(
  sessionId: Ref<string>, 
  cwd: Ref<string>, 
  getTerm: () => Terminal | null, 
  onCommandDone: () => void
) {
  const wsConnected = ref(false);
  let ws: WebSocket | null = null;

  function connect() {
    try {
      const url = buildTerminalWsUrl();
      ws = new WebSocket(url);
      
      ws.onopen = () => { wsConnected.value = true; };
      
      ws.onmessage = (event) => {
        const payload = JSON.parse(event.data);
        if (payload.type !== 'terminal' || payload.session_id !== sessionId.value) return;

        const term = getTerm();
        
        if (payload.event === 'command' && (payload.state === 'done' || payload.state === 'error')) {
          term?.write('\x1b[1A\x1b[2K'); // Clear "awaiting physical execution" line
          
          if (payload.stdout) {
            term?.write(payload.stdout.replace(/\n/g, '\r\n'));
            if (!payload.stdout.endsWith('\n')) term?.writeln('');
          }
          if (payload.stderr) {
            term?.write('\x1b[31m' + payload.stderr.replace(/\n/g, '\r\n') + '\x1b[0m');
            if (!payload.stderr.endsWith('\n')) term?.writeln('');
          }
          if (payload.exit_code != null && payload.exit_code !== 0) {
            term?.writeln(`\x1b[31m[Command failed with exit code ${payload.exit_code}]\x1b[0m`);
          }
          
          if (payload.cwd) cwd.value = payload.cwd;

          onCommandDone();
        }
      };
      
      ws.onclose = () => { wsConnected.value = false; };
      ws.onerror = () => {
        const term = getTerm();
        term?.writeln(`\x1b[1;31mWebSocket 服务异常断开。\x1b[0m`);
      };
    } catch (e: any) {
      message.error('终端消息通道建立失败');
    }
  }

  function disconnect() {
    if (ws) ws.close();
  }

  return { wsConnected, connect, disconnect };
}
