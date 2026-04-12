import { ref } from 'vue';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
export function useTerminalCore() {
    const terminalContainer = ref(null);
    let term = null;
    let fitAddon = null;
    let resizeObserver = null;
    function initXterm() {
        if (!terminalContainer.value)
            return null;
        term = new Terminal({
            cursorBlink: true,
            theme: {
                background: '#1e1e1e',
                foreground: '#d4d4d4',
                cursor: '#ffffff',
                selectionBackground: '#5c5c5c'
            },
            fontFamily: '"Fira Code", monospace, "Consolas"',
            fontSize: 14,
        });
        fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        term.open(terminalContainer.value);
        requestAnimationFrame(() => fitAddon?.fit());
        resizeObserver = new ResizeObserver(() => {
            if (fitAddon)
                requestAnimationFrame(() => fitAddon.fit());
        });
        resizeObserver.observe(terminalContainer.value);
        return term;
    }
    function disposeXterm() {
        if (resizeObserver)
            resizeObserver.disconnect();
        if (term)
            term.dispose();
    }
    return {
        terminalContainer,
        initXterm,
        disposeXterm,
        getTerm: () => term
    };
}
