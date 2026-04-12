export function useTerminalHistory() {
    let lineBuffer = '';
    const commandHistory = [];
    let historyIndex = -1;
    function handleBackspace(term) {
        if (lineBuffer.length > 0) {
            term.write('\b \b');
            lineBuffer = lineBuffer.substring(0, lineBuffer.length - 1);
        }
    }
    function handleArrowUp(term) {
        if (commandHistory.length > 0 && historyIndex > 0) {
            historyIndex--;
            clearLine(term);
            lineBuffer = commandHistory[historyIndex];
            term.write(lineBuffer);
        }
    }
    function handleArrowDown(term) {
        if (historyIndex < commandHistory.length) {
            historyIndex++;
            clearLine(term);
            if (historyIndex === commandHistory.length) {
                lineBuffer = '';
            }
            else {
                lineBuffer = commandHistory[historyIndex];
                term.write(lineBuffer);
            }
        }
    }
    function clearLine(term) {
        while (lineBuffer.length > 0) {
            term.write('\b \b');
            lineBuffer = lineBuffer.substring(0, lineBuffer.length - 1);
        }
    }
    function commitCommand(cmd) {
        if (cmd && commandHistory[commandHistory.length - 1] !== cmd) {
            commandHistory.push(cmd);
        }
        historyIndex = commandHistory.length;
        lineBuffer = '';
    }
    function appendChar(term, char) {
        term.write(char);
        lineBuffer += char;
    }
    function processKey(term, key, onEnter, onContextBreak) {
        switch (key) {
            case '\r': // Enter
                term.writeln('');
                const cmd = lineBuffer.trim();
                commitCommand(cmd);
                onEnter(cmd);
                break;
            case '\x03': // Ctrl+C
                term.writeln('^C');
                commitCommand('');
                onContextBreak();
                break;
            case '\x1b[A': // Up
                handleArrowUp(term);
                break;
            case '\x1b[B': // Down
                handleArrowDown(term);
                break;
            case '\x1b[C': // Right
            case '\x1b[D': // Left
                break;
            case '\u007F': // Backspace
                handleBackspace(term);
                break;
            default:
                // Basic printable characters filter
                if (key >= String.fromCharCode(0x20) && key <= String.fromCharCode(0x7E)) {
                    appendChar(term, key);
                }
        }
    }
    return { processKey };
}
