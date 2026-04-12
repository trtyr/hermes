export function formatBytes(bytes, decimals = 2) {
    if (!+bytes)
        return '0 Bytes';
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
}
export function formatUptime(seconds) {
    if (seconds < 60)
        return `${seconds} 秒`;
    const d = Math.floor(seconds / (3600 * 24));
    const h = Math.floor((seconds % (3600 * 24)) / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const parts = [];
    if (d > 0)
        parts.push(`${d}天`);
    if (h > 0)
        parts.push(`${h}小时`);
    if (m > 0)
        parts.push(`${m}分钟`);
    return parts.join(' ');
}
export function calculateMemoryPercent(used, total) {
    if (total === 0)
        return 0;
    return Math.round((used / total) * 100);
}
export function formatTimestamp(ts) {
    if (!ts)
        return '-';
    // Handle seconds or milliseconds depending on digits length
    const date = new Date(ts.toString().length === 10 ? ts * 1000 : ts);
    return date.toLocaleString('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
        hour12: false
    });
}
