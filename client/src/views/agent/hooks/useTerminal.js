import { ref, onMounted, onBeforeUnmount } from 'vue';
import { message } from 'ant-design-vue';
import { openTerminalSession, submitTerminalCommand, closeTerminalSession } from '@/api/terminal';
import { useTerminalCore } from './useTerminalCore';
import { useTerminalHistory } from './useTerminalHistory';
import { useTerminalSocket } from './useTerminalSocket';
export function useTerminal(agentId) {
    const sessionId = ref('');
    const cwd = ref('');
    let isExecuting = false;
    // 1. Core DOM & Xterm wrapper
    const { terminalContainer, initXterm, disposeXterm, getTerm } = useTerminalCore();
    // 2. Interactive Input Buffer parser
    const { processKey } = useTerminalHistory();
    // 3. Command execution done callback
    const onCommandDone = () => {
        isExecuting = false;
        printPrompt();
    };
    // 4. WebSocket Event Controller
    const { wsConnected, connect, disconnect } = useTerminalSocket(sessionId, cwd, getTerm, onCommandDone);
    function printPrompt() {
        const term = getTerm();
        if (!term || !sessionId.value)
            return;
        term.write(`\r\n\x1b[1;32mhermes\x1b[0m@\x1b[1;34m${agentId}\x1b[0m:\x1b[1;36m${cwd.value}\x1b[0m$ `);
    }
    async function handleEnterCommand(cmd) {
        const term = getTerm();
        if (!cmd) {
            printPrompt();
            return;
        }
        if (cmd === 'clear') {
            term?.clear();
            printPrompt();
            return;
        }
        isExecuting = true;
        term?.writeln('\x1b[33mCommand queued, awaiting physical execution...\x1b[0m');
        try {
            await submitTerminalCommand(sessionId.value, cmd);
        }
        catch (err) {
            term?.write('\x1b[1A\x1b[2K');
            term?.writeln(`\x1b[1;31m网络提交失败: ${err.message}\x1b[0m`);
            isExecuting = false;
            printPrompt();
        }
    }
    onMounted(async () => {
        const term = initXterm();
        if (!term)
            return;
        term.writeln(`\x1b[1;36m===================================================\x1b[0m`);
        term.writeln(`\x1b[1;36m Hermes C2 Interactive Terminal \x1b[0m`);
        term.writeln(`\x1b[1;36m Model: WebSocket Push (Microkernel API) \x1b[0m`);
        term.writeln(`\x1b[1;36m===================================================\x1b[0m\r\n`);
        term.writeln('正在向后端申请打开命令会话...');
        try {
            const res = await openTerminalSession(agentId);
            sessionId.value = res.data.session_id;
            cwd.value = res.data.cwd;
            connect();
            term.writeln(`\x1b[1;32m会话已成功建立 [Session ID: ${sessionId.value}]\x1b[0m`);
            printPrompt();
        }
        catch (err) {
            term.writeln(`\x1b[1;31m[错误] 初始化会话失败: ${err.message}\x1b[0m`);
            message.error('无法连接终端会话');
            return;
        }
        term.onData((e) => {
            if (isExecuting || !sessionId.value)
                return;
            processKey(term, e, handleEnterCommand, printPrompt);
        });
    });
    onBeforeUnmount(async () => {
        disposeXterm();
        disconnect();
        if (sessionId.value) {
            try {
                await closeTerminalSession(sessionId.value);
            }
            catch (e) {
                console.error('Failed to close terminal session silently', e);
            }
        }
    });
    return {
        terminalContainer,
        sessionId,
        wsConnected
    };
}
