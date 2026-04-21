/// <reference types="../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { CodeOutlined } from '@ant-design/icons-vue';
import { useAppStore } from '@/store/app';
import { useTerminal } from './hooks/useTerminal';
import 'xterm/css/xterm.css';
const route = useRoute();
const appStore = useAppStore();
const agentId = ref(route.params.id);
// Sync tab title
onMounted(() => {
    const currentView = appStore.visitedViews.find(v => v.path === route.path);
    if (currentView) {
        currentView.title = `终端: ${agentId.value}`;
    }
    else {
        appStore.addView({
            path: route.path,
            name: route.name || 'AgentTerminal',
            title: `终端: ${agentId.value}`
        });
    }
});
// Delegate complex behavior to the Microkernel Composable Hook
const { terminalContainer, sessionId, wsConnected } = useTerminal(agentId.value);
const __VLS_ctx = {
    ...{},
    ...{},
};
let __VLS_components;
let __VLS_intrinsics;
let __VLS_directives;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "h-full w-full flex flex-col p-4 relative bg-[#f0f2f5] dark:bg-[var(--bg-page)] transition-colors duration-300" },
});
/** @type {__VLS_StyleScopedClasses['h-full']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
/** @type {__VLS_StyleScopedClasses['p-4']} */ ;
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-[#f0f2f5]']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-page)]']} */ ;
/** @type {__VLS_StyleScopedClasses['transition-colors']} */ ;
/** @type {__VLS_StyleScopedClasses['duration-300']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex justify-between items-center mb-4" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-between']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['mb-4']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.h2, __VLS_intrinsics.h2)({
    ...{ class: "text-xl font-semibold text-slate-800 dark:text-[var(--text-primary)] flex items-center gap-2 m-0" },
});
/** @type {__VLS_StyleScopedClasses['text-xl']} */ ;
/** @type {__VLS_StyleScopedClasses['font-semibold']} */ ;
/** @type {__VLS_StyleScopedClasses['text-slate-800']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-[var(--text-primary)]']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
/** @type {__VLS_StyleScopedClasses['m-0']} */ ;
let __VLS_0;
/** @ts-ignore @type {typeof __VLS_components.CodeOutlined} */
CodeOutlined;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    ...{ class: "text-blue-500" },
}));
const __VLS_2 = __VLS_1({
    ...{ class: "text-blue-500" },
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
/** @type {__VLS_StyleScopedClasses['text-blue-500']} */ ;
(__VLS_ctx.agentId);
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({});
let __VLS_5;
/** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
aTag;
// @ts-ignore
const __VLS_6 = __VLS_asFunctionalComponent1(__VLS_5, new __VLS_5({
    color: (__VLS_ctx.wsConnected ? (__VLS_ctx.sessionId ? 'success' : 'processing') : 'error'),
    ...{ class: "border-0 font-medium" },
}));
const __VLS_7 = __VLS_6({
    color: (__VLS_ctx.wsConnected ? (__VLS_ctx.sessionId ? 'success' : 'processing') : 'error'),
    ...{ class: "border-0 font-medium" },
}, ...__VLS_functionalComponentArgsRest(__VLS_6));
/** @type {__VLS_StyleScopedClasses['border-0']} */ ;
/** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
const { default: __VLS_10 } = __VLS_8.slots;
(__VLS_ctx.wsConnected ? (__VLS_ctx.sessionId ? '已连接 (Session Active)' : '初始化会话中...') : 'WebSocket 失联');
// @ts-ignore
[agentId, wsConnected, wsConnected, sessionId, sessionId,];
var __VLS_8;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex-1 bg-[#1e1e1e] dark:bg-[#0a0a0a] rounded-lg border border-gray-200 dark:border-[var(--border-default)] shadow-sm overflow-hidden relative" },
});
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-[#1e1e1e]']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[#0a0a0a]']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-lg']} */ ;
/** @type {__VLS_StyleScopedClasses['border']} */ ;
/** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:border-[var(--border-default)]']} */ ;
/** @type {__VLS_StyleScopedClasses['shadow-sm']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ref: "terminalContainer",
    ...{ class: "absolute inset-0 p-3 pt-2" },
});
/** @type {__VLS_StyleScopedClasses['absolute']} */ ;
/** @type {__VLS_StyleScopedClasses['inset-0']} */ ;
/** @type {__VLS_StyleScopedClasses['p-3']} */ ;
/** @type {__VLS_StyleScopedClasses['pt-2']} */ ;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
