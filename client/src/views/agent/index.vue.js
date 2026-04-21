/// <reference types="../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, reactive, onMounted, onBeforeUnmount } from 'vue';
import { useRouter } from 'vue-router';
import { message, Modal } from 'ant-design-vue';
import { ReloadOutlined, WindowsOutlined, AppleOutlined, DesktopOutlined, MoreOutlined, CodeOutlined, DisconnectOutlined, StopOutlined, CheckCircleOutlined, DeleteOutlined, FolderOpenOutlined } from '@ant-design/icons-vue';
import { fetchAgents, disconnectAgent, disableAgent, enableAgent, deleteAgent, fetchAgentDetail, takeScreenshot, dispatchTask } from '@/api/agent';
import { formatTimestamp } from '@/utils/format';
import { useAgentWebSocket } from './hooks/useAgentWebSocket';
import AgentDetailDrawer from './components/AgentDetailDrawer.vue';
import AgentTaskModal from './components/AgentTaskModal.vue';
import FileOpsModal from './components/FileOpsModal.vue';
import AgentContextMenu from './components/AgentContextMenu.vue';
const router = useRouter();
// Core State
const agents = ref([]);
const loading = ref(false);
const searchKeyword = ref('');
const pagination = reactive({ current: 1, pageSize: 20, total: 0, showSizeChanger: true });
// UI Component State
const detailVisible = ref(false);
const taskModalVisible = ref(false);
const fileOpsVisible = ref(false);
const selectedAgent = ref(null);
const actionAgent = ref(null);
const contextMenuState = reactive({ visible: false, x: 0, y: 0, record: null });
const columns = [
    { title: '节点 ID', dataIndex: 'agent_id', key: 'agent_id', width: 140 },
    { title: '状态', key: 'status', width: 100 },
    { title: '用户名', dataIndex: 'username', key: 'username', width: 120 },
    { title: '平台/架构', key: 'platform', width: 180 },
    { title: '网络地址', key: 'network', width: 160 },
    { title: 'Beacon', key: 'beacon', width: 120 },
    { title: '最后活跃', key: 'last_seen', width: 160 },
    { title: '操作', key: 'action', width: 100, fixed: 'right' }
];
// Initialize WebSocket Hook (Microkernel approach)
useAgentWebSocket(agents, selectedAgent, detailVisible, loadAgents);
async function loadAgents() {
    loading.value = true;
    try {
        const res = await fetchAgents({
            limit: pagination.pageSize,
            offset: (pagination.current - 1) * pagination.pageSize,
            keyword: searchKeyword.value || undefined
        });
        agents.value = res.agents || [];
        pagination.total = res.total || 0;
    }
    catch (error) {
        message.error(error.message || '获取节点列表失败');
    }
    finally {
        loading.value = false;
    }
}
function handleTableChange(pag) {
    pagination.current = pag.current;
    pagination.pageSize = pag.pageSize;
    loadAgents();
}
function onSearch() {
    pagination.current = 1;
    loadAgents();
}
function openDetail(agent) {
    selectedAgent.value = agent;
    detailVisible.value = true;
}
function openTaskModal(agent) {
    actionAgent.value = agent;
    taskModalVisible.value = true;
}
function openFileOps(agent) {
    actionAgent.value = agent;
    fileOpsVisible.value = true;
}
async function handleScreenshot(agent) {
    try {
        const res = await takeScreenshot(agent.agent_id);
        if (res.success) {
            message.success(`截图任务已下发 (task: ${res.task_id})`);
        }
        else {
            message.error(res.detail || '截图失败');
        }
    }
    catch (e) {
        message.error(e.message);
    }
}
async function handlePs(agent) {
    try {
        const res = await dispatchTask(agent.agent_id, { command: 'ps' });
        if (res.success) {
            message.success(`进程列表任务已下发 (task: ${res.task_id})`);
        }
        else {
            message.error(res.detail || '获取进程列表失败');
        }
    }
    catch (e) {
        message.error(e.message);
    }
}
function openTerminal(agent) {
    router.push(`/agent/terminal/${agent.agent_id}`);
}
function handleAgentStoreUpdate(agent) {
    // Sync changes back to the main agents array if needed
    const idx = agents.value.findIndex(a => a.agent_id === agent.agent_id);
    if (idx > -1)
        agents.value[idx] = agent;
    if (selectedAgent.value?.agent_id === agent.agent_id) {
        selectedAgent.value = agent;
    }
}
function handleAction({ action, agent }) {
    const actionMap = {
        'disconnect': { title: '断开连接', func: disconnectAgent },
        'disable': { title: '禁用节点', func: disableAgent },
        'enable': { title: '启用节点', func: enableAgent },
        'delete': { title: '删除记录', func: deleteAgent },
    };
    const target = actionMap[action];
    if (!target)
        return;
    Modal.confirm({
        title: `确认要对节点 ${agent.agent_id} 执行 [${target.title}] 操作吗？`,
        content: action === 'delete' ? '删除后将不可恢复。' : '',
        okType: action === 'delete' || action === 'disable' || action === 'disconnect' ? 'danger' : 'primary',
        async onOk() {
            try {
                await target.func(agent.agent_id);
                message.success(`操作 [${target.title}] 执行成功`);
                if (action === 'delete') {
                    detailVisible.value = false;
                }
                else if (detailVisible.value && selectedAgent.value?.agent_id === agent.agent_id) {
                    selectedAgent.value = await fetchAgentDetail(agent.agent_id);
                }
                loadAgents();
            }
            catch (e) {
                message.error(e.message);
            }
        }
    });
}
const customRow = (record) => {
    return {
        onContextmenu: (e) => {
            e.preventDefault();
            contextMenuState.x = e.clientX;
            contextMenuState.y = e.clientY;
            contextMenuState.record = record;
            contextMenuState.visible = true;
        }
    };
};
function closeContextMenu() {
    contextMenuState.visible = false;
}
onMounted(() => {
    loadAgents();
    document.addEventListener('click', closeContextMenu);
});
onBeforeUnmount(() => {
    document.removeEventListener('click', closeContextMenu);
});
const __VLS_ctx = {
    ...{},
    ...{},
};
let __VLS_components;
let __VLS_intrinsics;
let __VLS_directives;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "h-full w-full flex flex-col p-4 relative" },
});
/** @type {__VLS_StyleScopedClasses['h-full']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
/** @type {__VLS_StyleScopedClasses['p-4']} */ ;
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex justify-between items-center mb-4" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-between']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['mb-4']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.h2, __VLS_intrinsics.h2)({
    ...{ class: "text-xl font-semibold text-slate-800 dark:text-[var(--text-primary)]" },
});
/** @type {__VLS_StyleScopedClasses['text-xl']} */ ;
/** @type {__VLS_StyleScopedClasses['font-semibold']} */ ;
/** @type {__VLS_StyleScopedClasses['text-slate-800']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-[var(--text-primary)]']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex items-center gap-2" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
let __VLS_0;
/** @ts-ignore @type {typeof __VLS_components.aInputSearch | typeof __VLS_components.AInputSearch} */
aInputSearch;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    ...{ 'onSearch': {} },
    value: (__VLS_ctx.searchKeyword),
    placeholder: "搜索节点...",
    ...{ style: {} },
    allowClear: true,
}));
const __VLS_2 = __VLS_1({
    ...{ 'onSearch': {} },
    value: (__VLS_ctx.searchKeyword),
    placeholder: "搜索节点...",
    ...{ style: {} },
    allowClear: true,
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
let __VLS_5;
const __VLS_6 = ({ search: {} },
    { onSearch: (__VLS_ctx.onSearch) });
var __VLS_3;
var __VLS_4;
let __VLS_7;
/** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
aButton;
// @ts-ignore
const __VLS_8 = __VLS_asFunctionalComponent1(__VLS_7, new __VLS_7({
    ...{ 'onClick': {} },
}));
const __VLS_9 = __VLS_8({
    ...{ 'onClick': {} },
}, ...__VLS_functionalComponentArgsRest(__VLS_8));
let __VLS_12;
const __VLS_13 = ({ click: {} },
    { onClick: (__VLS_ctx.loadAgents) });
const { default: __VLS_14 } = __VLS_10.slots;
{
    const { icon: __VLS_15 } = __VLS_10.slots;
    let __VLS_16;
    /** @ts-ignore @type {typeof __VLS_components.ReloadOutlined} */
    ReloadOutlined;
    // @ts-ignore
    const __VLS_17 = __VLS_asFunctionalComponent1(__VLS_16, new __VLS_16({}));
    const __VLS_18 = __VLS_17({}, ...__VLS_functionalComponentArgsRest(__VLS_17));
    // @ts-ignore
    [searchKeyword, onSearch, loadAgents,];
}
// @ts-ignore
[];
var __VLS_10;
var __VLS_11;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex-1 bg-white dark:bg-[var(--bg-card)] rounded-lg border border-gray-200 dark:border-[var(--border-default)] shadow-sm overflow-hidden flex flex-col" },
});
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-card)]']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-lg']} */ ;
/** @type {__VLS_StyleScopedClasses['border']} */ ;
/** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:border-[var(--border-default)]']} */ ;
/** @type {__VLS_StyleScopedClasses['shadow-sm']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
let __VLS_21;
/** @ts-ignore @type {typeof __VLS_components.aTable | typeof __VLS_components.ATable | typeof __VLS_components.aTable | typeof __VLS_components.ATable} */
aTable;
// @ts-ignore
const __VLS_22 = __VLS_asFunctionalComponent1(__VLS_21, new __VLS_21({
    ...{ 'onChange': {} },
    dataSource: (__VLS_ctx.agents),
    columns: (__VLS_ctx.columns),
    loading: (__VLS_ctx.loading),
    pagination: (__VLS_ctx.pagination),
    customRow: (__VLS_ctx.customRow),
    size: "middle",
    scroll: ({ x: 'max-content', y: 'calc(100vh - 280px)' }),
    rowKey: "agent_id",
    ...{ class: "w-full h-full" },
}));
const __VLS_23 = __VLS_22({
    ...{ 'onChange': {} },
    dataSource: (__VLS_ctx.agents),
    columns: (__VLS_ctx.columns),
    loading: (__VLS_ctx.loading),
    pagination: (__VLS_ctx.pagination),
    customRow: (__VLS_ctx.customRow),
    size: "middle",
    scroll: ({ x: 'max-content', y: 'calc(100vh - 280px)' }),
    rowKey: "agent_id",
    ...{ class: "w-full h-full" },
}, ...__VLS_functionalComponentArgsRest(__VLS_22));
let __VLS_26;
const __VLS_27 = ({ change: {} },
    { onChange: (__VLS_ctx.handleTableChange) });
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['h-full']} */ ;
const { default: __VLS_28 } = __VLS_24.slots;
{
    const { bodyCell: __VLS_29 } = __VLS_24.slots;
    const [{ column, record }] = __VLS_vSlot(__VLS_29);
    if (column.key === 'agent_id') {
        let __VLS_30;
        /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
        aButton;
        // @ts-ignore
        const __VLS_31 = __VLS_asFunctionalComponent1(__VLS_30, new __VLS_30({
            ...{ 'onClick': {} },
            type: "link",
            ...{ class: "p-0 font-medium" },
        }));
        const __VLS_32 = __VLS_31({
            ...{ 'onClick': {} },
            type: "link",
            ...{ class: "p-0 font-medium" },
        }, ...__VLS_functionalComponentArgsRest(__VLS_31));
        let __VLS_35;
        const __VLS_36 = ({ click: {} },
            { onClick: (...[$event]) => {
                    if (!(column.key === 'agent_id'))
                        return;
                    __VLS_ctx.openDetail(record);
                    // @ts-ignore
                    [agents, columns, loading, pagination, customRow, handleTableChange, openDetail,];
                } });
        /** @type {__VLS_StyleScopedClasses['p-0']} */ ;
        /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
        const { default: __VLS_37 } = __VLS_33.slots;
        (record.agent_id);
        // @ts-ignore
        [];
        var __VLS_33;
        var __VLS_34;
    }
    else if (column.key === 'status') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "flex flex-col gap-1" },
        });
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-1']} */ ;
        let __VLS_38;
        /** @ts-ignore @type {typeof __VLS_components.aBadge | typeof __VLS_components.ABadge} */
        aBadge;
        // @ts-ignore
        const __VLS_39 = __VLS_asFunctionalComponent1(__VLS_38, new __VLS_38({
            status: (record.is_online ? 'success' : 'default'),
            text: (record.is_online ? '在线' : '离线'),
        }));
        const __VLS_40 = __VLS_39({
            status: (record.is_online ? 'success' : 'default'),
            text: (record.is_online ? '在线' : '离线'),
        }, ...__VLS_functionalComponentArgsRest(__VLS_39));
        if (record.is_disabled) {
            let __VLS_43;
            /** @ts-ignore @type {typeof __VLS_components.aBadge | typeof __VLS_components.ABadge} */
            aBadge;
            // @ts-ignore
            const __VLS_44 = __VLS_asFunctionalComponent1(__VLS_43, new __VLS_43({
                status: "error",
                text: "已禁用",
            }));
            const __VLS_45 = __VLS_44({
                status: "error",
                text: "已禁用",
            }, ...__VLS_functionalComponentArgsRest(__VLS_44));
        }
    }
    else if (column.key === 'platform') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "flex items-center gap-1.5" },
        });
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-1.5']} */ ;
        if (record.os && record.os.toLowerCase().includes('windows')) {
            let __VLS_48;
            /** @ts-ignore @type {typeof __VLS_components.WindowsOutlined} */
            WindowsOutlined;
            // @ts-ignore
            const __VLS_49 = __VLS_asFunctionalComponent1(__VLS_48, new __VLS_48({
                ...{ class: "text-blue-500" },
            }));
            const __VLS_50 = __VLS_49({
                ...{ class: "text-blue-500" },
            }, ...__VLS_functionalComponentArgsRest(__VLS_49));
            /** @type {__VLS_StyleScopedClasses['text-blue-500']} */ ;
        }
        else if (record.os && (record.os.toLowerCase().includes('mac') || record.os.toLowerCase().includes('darwin'))) {
            let __VLS_53;
            /** @ts-ignore @type {typeof __VLS_components.AppleOutlined} */
            AppleOutlined;
            // @ts-ignore
            const __VLS_54 = __VLS_asFunctionalComponent1(__VLS_53, new __VLS_53({
                ...{ class: "text-gray-500 dark:text-gray-300" },
            }));
            const __VLS_55 = __VLS_54({
                ...{ class: "text-gray-500 dark:text-gray-300" },
            }, ...__VLS_functionalComponentArgsRest(__VLS_54));
            /** @type {__VLS_StyleScopedClasses['text-gray-500']} */ ;
            /** @type {__VLS_StyleScopedClasses['dark:text-gray-300']} */ ;
        }
        else if (record.os && record.os.toLowerCase().includes('linux')) {
            __VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
                ...{ class: "w-3.5 h-3.5 text-yellow-600" },
                viewBox: "0 0 24 24",
                fill: "currentColor",
            });
            /** @type {__VLS_StyleScopedClasses['w-3.5']} */ ;
            /** @type {__VLS_StyleScopedClasses['h-3.5']} */ ;
            /** @type {__VLS_StyleScopedClasses['text-yellow-600']} */ ;
            __VLS_asFunctionalElement1(__VLS_intrinsics.path)({
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-13h2v6h-2zm0 8h2v2h-2z",
            });
        }
        else {
            let __VLS_58;
            /** @ts-ignore @type {typeof __VLS_components.DesktopOutlined} */
            DesktopOutlined;
            // @ts-ignore
            const __VLS_59 = __VLS_asFunctionalComponent1(__VLS_58, new __VLS_58({
                ...{ class: "text-slate-500" },
            }));
            const __VLS_60 = __VLS_59({
                ...{ class: "text-slate-500" },
            }, ...__VLS_functionalComponentArgsRest(__VLS_59));
            /** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
        }
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({});
        (record.os);
        (record.arch);
    }
    else if (column.key === 'network') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "text-xs space-y-0.5" },
        });
        /** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
        /** @type {__VLS_StyleScopedClasses['space-y-0.5']} */ ;
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({});
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "text-slate-400" },
        });
        /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
        (record.internal_ip);
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({});
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "text-slate-400" },
        });
        /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
        (record.external_ip);
    }
    else if (column.key === 'beacon') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "text-xs" },
        });
        /** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
        (record.sleep_interval);
        (record.jitter);
    }
    else if (column.key === 'last_seen') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "text-xs" },
        });
        /** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
        (__VLS_ctx.formatTimestamp(record.last_seen));
    }
    else if (column.key === 'action') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "flex items-center gap-2" },
        });
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
        let __VLS_63;
        /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
        aButton;
        // @ts-ignore
        const __VLS_64 = __VLS_asFunctionalComponent1(__VLS_63, new __VLS_63({
            ...{ 'onClick': {} },
            type: "link",
            size: "small",
            ...{ class: "p-0" },
        }));
        const __VLS_65 = __VLS_64({
            ...{ 'onClick': {} },
            type: "link",
            size: "small",
            ...{ class: "p-0" },
        }, ...__VLS_functionalComponentArgsRest(__VLS_64));
        let __VLS_68;
        const __VLS_69 = ({ click: {} },
            { onClick: (...[$event]) => {
                    if (!!(column.key === 'agent_id'))
                        return;
                    if (!!(column.key === 'status'))
                        return;
                    if (!!(column.key === 'platform'))
                        return;
                    if (!!(column.key === 'network'))
                        return;
                    if (!!(column.key === 'beacon'))
                        return;
                    if (!!(column.key === 'last_seen'))
                        return;
                    if (!(column.key === 'action'))
                        return;
                    __VLS_ctx.openDetail(record);
                    // @ts-ignore
                    [openDetail, formatTimestamp,];
                } });
        /** @type {__VLS_StyleScopedClasses['p-0']} */ ;
        const { default: __VLS_70 } = __VLS_66.slots;
        // @ts-ignore
        [];
        var __VLS_66;
        var __VLS_67;
        let __VLS_71;
        /** @ts-ignore @type {typeof __VLS_components.aDropdown | typeof __VLS_components.ADropdown | typeof __VLS_components.aDropdown | typeof __VLS_components.ADropdown} */
        aDropdown;
        // @ts-ignore
        const __VLS_72 = __VLS_asFunctionalComponent1(__VLS_71, new __VLS_71({
            trigger: (['click']),
        }));
        const __VLS_73 = __VLS_72({
            trigger: (['click']),
        }, ...__VLS_functionalComponentArgsRest(__VLS_72));
        const { default: __VLS_76 } = __VLS_74.slots;
        let __VLS_77;
        /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
        aButton;
        // @ts-ignore
        const __VLS_78 = __VLS_asFunctionalComponent1(__VLS_77, new __VLS_77({
            type: "text",
            size: "small",
            ...{ class: "px-1" },
        }));
        const __VLS_79 = __VLS_78({
            type: "text",
            size: "small",
            ...{ class: "px-1" },
        }, ...__VLS_functionalComponentArgsRest(__VLS_78));
        /** @type {__VLS_StyleScopedClasses['px-1']} */ ;
        const { default: __VLS_82 } = __VLS_80.slots;
        let __VLS_83;
        /** @ts-ignore @type {typeof __VLS_components.MoreOutlined} */
        MoreOutlined;
        // @ts-ignore
        const __VLS_84 = __VLS_asFunctionalComponent1(__VLS_83, new __VLS_83({}));
        const __VLS_85 = __VLS_84({}, ...__VLS_functionalComponentArgsRest(__VLS_84));
        // @ts-ignore
        [];
        var __VLS_80;
        {
            const { overlay: __VLS_88 } = __VLS_74.slots;
            let __VLS_89;
            /** @ts-ignore @type {typeof __VLS_components.aMenu | typeof __VLS_components.AMenu | typeof __VLS_components.aMenu | typeof __VLS_components.AMenu} */
            aMenu;
            // @ts-ignore
            const __VLS_90 = __VLS_asFunctionalComponent1(__VLS_89, new __VLS_89({}));
            const __VLS_91 = __VLS_90({}, ...__VLS_functionalComponentArgsRest(__VLS_90));
            const { default: __VLS_94 } = __VLS_92.slots;
            let __VLS_95;
            /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
            aMenuItem;
            // @ts-ignore
            const __VLS_96 = __VLS_asFunctionalComponent1(__VLS_95, new __VLS_95({
                ...{ 'onClick': {} },
                key: "task",
                disabled: (record.is_disabled),
            }));
            const __VLS_97 = __VLS_96({
                ...{ 'onClick': {} },
                key: "task",
                disabled: (record.is_disabled),
            }, ...__VLS_functionalComponentArgsRest(__VLS_96));
            let __VLS_100;
            const __VLS_101 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!!(column.key === 'agent_id'))
                            return;
                        if (!!(column.key === 'status'))
                            return;
                        if (!!(column.key === 'platform'))
                            return;
                        if (!!(column.key === 'network'))
                            return;
                        if (!!(column.key === 'beacon'))
                            return;
                        if (!!(column.key === 'last_seen'))
                            return;
                        if (!(column.key === 'action'))
                            return;
                        __VLS_ctx.openTaskModal(record);
                        // @ts-ignore
                        [openTaskModal,];
                    } });
            const { default: __VLS_102 } = __VLS_98.slots;
            {
                const { icon: __VLS_103 } = __VLS_98.slots;
                let __VLS_104;
                /** @ts-ignore @type {typeof __VLS_components.CodeOutlined} */
                CodeOutlined;
                // @ts-ignore
                const __VLS_105 = __VLS_asFunctionalComponent1(__VLS_104, new __VLS_104({}));
                const __VLS_106 = __VLS_105({}, ...__VLS_functionalComponentArgsRest(__VLS_105));
                // @ts-ignore
                [];
            }
            // @ts-ignore
            [];
            var __VLS_98;
            var __VLS_99;
            let __VLS_109;
            /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
            aMenuItem;
            // @ts-ignore
            const __VLS_110 = __VLS_asFunctionalComponent1(__VLS_109, new __VLS_109({
                ...{ 'onClick': {} },
                key: "terminal",
                disabled: (record.is_disabled),
            }));
            const __VLS_111 = __VLS_110({
                ...{ 'onClick': {} },
                key: "terminal",
                disabled: (record.is_disabled),
            }, ...__VLS_functionalComponentArgsRest(__VLS_110));
            let __VLS_114;
            const __VLS_115 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!!(column.key === 'agent_id'))
                            return;
                        if (!!(column.key === 'status'))
                            return;
                        if (!!(column.key === 'platform'))
                            return;
                        if (!!(column.key === 'network'))
                            return;
                        if (!!(column.key === 'beacon'))
                            return;
                        if (!!(column.key === 'last_seen'))
                            return;
                        if (!(column.key === 'action'))
                            return;
                        __VLS_ctx.openTerminal(record);
                        // @ts-ignore
                        [openTerminal,];
                    } });
            const { default: __VLS_116 } = __VLS_112.slots;
            {
                const { icon: __VLS_117 } = __VLS_112.slots;
                let __VLS_118;
                /** @ts-ignore @type {typeof __VLS_components.DesktopOutlined} */
                DesktopOutlined;
                // @ts-ignore
                const __VLS_119 = __VLS_asFunctionalComponent1(__VLS_118, new __VLS_118({}));
                const __VLS_120 = __VLS_119({}, ...__VLS_functionalComponentArgsRest(__VLS_119));
                // @ts-ignore
                [];
            }
            // @ts-ignore
            [];
            var __VLS_112;
            var __VLS_113;
            let __VLS_123;
            /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
            aMenuItem;
            // @ts-ignore
            const __VLS_124 = __VLS_asFunctionalComponent1(__VLS_123, new __VLS_123({
                ...{ 'onClick': {} },
                key: "fileops",
                disabled: (record.is_disabled),
            }));
            const __VLS_125 = __VLS_124({
                ...{ 'onClick': {} },
                key: "fileops",
                disabled: (record.is_disabled),
            }, ...__VLS_functionalComponentArgsRest(__VLS_124));
            let __VLS_128;
            const __VLS_129 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!!(column.key === 'agent_id'))
                            return;
                        if (!!(column.key === 'status'))
                            return;
                        if (!!(column.key === 'platform'))
                            return;
                        if (!!(column.key === 'network'))
                            return;
                        if (!!(column.key === 'beacon'))
                            return;
                        if (!!(column.key === 'last_seen'))
                            return;
                        if (!(column.key === 'action'))
                            return;
                        __VLS_ctx.openFileOps(record);
                        // @ts-ignore
                        [openFileOps,];
                    } });
            const { default: __VLS_130 } = __VLS_126.slots;
            {
                const { icon: __VLS_131 } = __VLS_126.slots;
                let __VLS_132;
                /** @ts-ignore @type {typeof __VLS_components.FolderOpenOutlined} */
                FolderOpenOutlined;
                // @ts-ignore
                const __VLS_133 = __VLS_asFunctionalComponent1(__VLS_132, new __VLS_132({}));
                const __VLS_134 = __VLS_133({}, ...__VLS_functionalComponentArgsRest(__VLS_133));
                // @ts-ignore
                [];
            }
            // @ts-ignore
            [];
            var __VLS_126;
            var __VLS_127;
            let __VLS_137;
            /** @ts-ignore @type {typeof __VLS_components.aMenuDivider | typeof __VLS_components.AMenuDivider} */
            aMenuDivider;
            // @ts-ignore
            const __VLS_138 = __VLS_asFunctionalComponent1(__VLS_137, new __VLS_137({}));
            const __VLS_139 = __VLS_138({}, ...__VLS_functionalComponentArgsRest(__VLS_138));
            let __VLS_142;
            /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
            aMenuItem;
            // @ts-ignore
            const __VLS_143 = __VLS_asFunctionalComponent1(__VLS_142, new __VLS_142({
                ...{ 'onClick': {} },
                key: "disconnect",
                disabled: (!record.is_online),
            }));
            const __VLS_144 = __VLS_143({
                ...{ 'onClick': {} },
                key: "disconnect",
                disabled: (!record.is_online),
            }, ...__VLS_functionalComponentArgsRest(__VLS_143));
            let __VLS_147;
            const __VLS_148 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!!(column.key === 'agent_id'))
                            return;
                        if (!!(column.key === 'status'))
                            return;
                        if (!!(column.key === 'platform'))
                            return;
                        if (!!(column.key === 'network'))
                            return;
                        if (!!(column.key === 'beacon'))
                            return;
                        if (!!(column.key === 'last_seen'))
                            return;
                        if (!(column.key === 'action'))
                            return;
                        __VLS_ctx.handleAction({ action: 'disconnect', agent: record });
                        // @ts-ignore
                        [handleAction,];
                    } });
            const { default: __VLS_149 } = __VLS_145.slots;
            {
                const { icon: __VLS_150 } = __VLS_145.slots;
                let __VLS_151;
                /** @ts-ignore @type {typeof __VLS_components.DisconnectOutlined} */
                DisconnectOutlined;
                // @ts-ignore
                const __VLS_152 = __VLS_asFunctionalComponent1(__VLS_151, new __VLS_151({}));
                const __VLS_153 = __VLS_152({}, ...__VLS_functionalComponentArgsRest(__VLS_152));
                // @ts-ignore
                [];
            }
            // @ts-ignore
            [];
            var __VLS_145;
            var __VLS_146;
            if (!record.is_disabled) {
                let __VLS_156;
                /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
                aMenuItem;
                // @ts-ignore
                const __VLS_157 = __VLS_asFunctionalComponent1(__VLS_156, new __VLS_156({
                    ...{ 'onClick': {} },
                    key: "disable",
                }));
                const __VLS_158 = __VLS_157({
                    ...{ 'onClick': {} },
                    key: "disable",
                }, ...__VLS_functionalComponentArgsRest(__VLS_157));
                let __VLS_161;
                const __VLS_162 = ({ click: {} },
                    { onClick: (...[$event]) => {
                            if (!!(column.key === 'agent_id'))
                                return;
                            if (!!(column.key === 'status'))
                                return;
                            if (!!(column.key === 'platform'))
                                return;
                            if (!!(column.key === 'network'))
                                return;
                            if (!!(column.key === 'beacon'))
                                return;
                            if (!!(column.key === 'last_seen'))
                                return;
                            if (!(column.key === 'action'))
                                return;
                            if (!(!record.is_disabled))
                                return;
                            __VLS_ctx.handleAction({ action: 'disable', agent: record });
                            // @ts-ignore
                            [handleAction,];
                        } });
                const { default: __VLS_163 } = __VLS_159.slots;
                {
                    const { icon: __VLS_164 } = __VLS_159.slots;
                    let __VLS_165;
                    /** @ts-ignore @type {typeof __VLS_components.StopOutlined} */
                    StopOutlined;
                    // @ts-ignore
                    const __VLS_166 = __VLS_asFunctionalComponent1(__VLS_165, new __VLS_165({}));
                    const __VLS_167 = __VLS_166({}, ...__VLS_functionalComponentArgsRest(__VLS_166));
                    // @ts-ignore
                    [];
                }
                // @ts-ignore
                [];
                var __VLS_159;
                var __VLS_160;
            }
            if (record.is_disabled) {
                let __VLS_170;
                /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
                aMenuItem;
                // @ts-ignore
                const __VLS_171 = __VLS_asFunctionalComponent1(__VLS_170, new __VLS_170({
                    ...{ 'onClick': {} },
                    key: "enable",
                }));
                const __VLS_172 = __VLS_171({
                    ...{ 'onClick': {} },
                    key: "enable",
                }, ...__VLS_functionalComponentArgsRest(__VLS_171));
                let __VLS_175;
                const __VLS_176 = ({ click: {} },
                    { onClick: (...[$event]) => {
                            if (!!(column.key === 'agent_id'))
                                return;
                            if (!!(column.key === 'status'))
                                return;
                            if (!!(column.key === 'platform'))
                                return;
                            if (!!(column.key === 'network'))
                                return;
                            if (!!(column.key === 'beacon'))
                                return;
                            if (!!(column.key === 'last_seen'))
                                return;
                            if (!(column.key === 'action'))
                                return;
                            if (!(record.is_disabled))
                                return;
                            __VLS_ctx.handleAction({ action: 'enable', agent: record });
                            // @ts-ignore
                            [handleAction,];
                        } });
                const { default: __VLS_177 } = __VLS_173.slots;
                {
                    const { icon: __VLS_178 } = __VLS_173.slots;
                    let __VLS_179;
                    /** @ts-ignore @type {typeof __VLS_components.CheckCircleOutlined} */
                    CheckCircleOutlined;
                    // @ts-ignore
                    const __VLS_180 = __VLS_asFunctionalComponent1(__VLS_179, new __VLS_179({}));
                    const __VLS_181 = __VLS_180({}, ...__VLS_functionalComponentArgsRest(__VLS_180));
                    // @ts-ignore
                    [];
                }
                // @ts-ignore
                [];
                var __VLS_173;
                var __VLS_174;
            }
            let __VLS_184;
            /** @ts-ignore @type {typeof __VLS_components.aMenuDivider | typeof __VLS_components.AMenuDivider} */
            aMenuDivider;
            // @ts-ignore
            const __VLS_185 = __VLS_asFunctionalComponent1(__VLS_184, new __VLS_184({}));
            const __VLS_186 = __VLS_185({}, ...__VLS_functionalComponentArgsRest(__VLS_185));
            let __VLS_189;
            /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
            aMenuItem;
            // @ts-ignore
            const __VLS_190 = __VLS_asFunctionalComponent1(__VLS_189, new __VLS_189({
                ...{ 'onClick': {} },
                key: "delete",
                disabled: (record.is_online),
                danger: true,
            }));
            const __VLS_191 = __VLS_190({
                ...{ 'onClick': {} },
                key: "delete",
                disabled: (record.is_online),
                danger: true,
            }, ...__VLS_functionalComponentArgsRest(__VLS_190));
            let __VLS_194;
            const __VLS_195 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!!(column.key === 'agent_id'))
                            return;
                        if (!!(column.key === 'status'))
                            return;
                        if (!!(column.key === 'platform'))
                            return;
                        if (!!(column.key === 'network'))
                            return;
                        if (!!(column.key === 'beacon'))
                            return;
                        if (!!(column.key === 'last_seen'))
                            return;
                        if (!(column.key === 'action'))
                            return;
                        __VLS_ctx.handleAction({ action: 'delete', agent: record });
                        // @ts-ignore
                        [handleAction,];
                    } });
            const { default: __VLS_196 } = __VLS_192.slots;
            {
                const { icon: __VLS_197 } = __VLS_192.slots;
                let __VLS_198;
                /** @ts-ignore @type {typeof __VLS_components.DeleteOutlined} */
                DeleteOutlined;
                // @ts-ignore
                const __VLS_199 = __VLS_asFunctionalComponent1(__VLS_198, new __VLS_198({}));
                const __VLS_200 = __VLS_199({}, ...__VLS_functionalComponentArgsRest(__VLS_199));
                // @ts-ignore
                [];
            }
            // @ts-ignore
            [];
            var __VLS_192;
            var __VLS_193;
            // @ts-ignore
            [];
            var __VLS_92;
            // @ts-ignore
            [];
        }
        // @ts-ignore
        [];
        var __VLS_74;
    }
    // @ts-ignore
    [];
}
// @ts-ignore
[];
var __VLS_24;
var __VLS_25;
const __VLS_203 = AgentDetailDrawer;
// @ts-ignore
const __VLS_204 = __VLS_asFunctionalComponent1(__VLS_203, new __VLS_203({
    ...{ 'onUpdate:agent': {} },
    ...{ 'onOpenTask': {} },
    ...{ 'onAction': {} },
    visible: (__VLS_ctx.detailVisible),
    agent: (__VLS_ctx.selectedAgent),
}));
const __VLS_205 = __VLS_204({
    ...{ 'onUpdate:agent': {} },
    ...{ 'onOpenTask': {} },
    ...{ 'onAction': {} },
    visible: (__VLS_ctx.detailVisible),
    agent: (__VLS_ctx.selectedAgent),
}, ...__VLS_functionalComponentArgsRest(__VLS_204));
let __VLS_208;
const __VLS_209 = ({ 'update:agent': {} },
    { 'onUpdate:agent': (__VLS_ctx.handleAgentStoreUpdate) });
const __VLS_210 = ({ openTask: {} },
    { onOpenTask: (__VLS_ctx.openTaskModal) });
const __VLS_211 = ({ action: {} },
    { onAction: (__VLS_ctx.handleAction) });
var __VLS_206;
var __VLS_207;
const __VLS_212 = AgentTaskModal;
// @ts-ignore
const __VLS_213 = __VLS_asFunctionalComponent1(__VLS_212, new __VLS_212({
    visible: (__VLS_ctx.taskModalVisible),
    agent: (__VLS_ctx.actionAgent),
}));
const __VLS_214 = __VLS_213({
    visible: (__VLS_ctx.taskModalVisible),
    agent: (__VLS_ctx.actionAgent),
}, ...__VLS_functionalComponentArgsRest(__VLS_213));
const __VLS_217 = FileOpsModal;
// @ts-ignore
const __VLS_218 = __VLS_asFunctionalComponent1(__VLS_217, new __VLS_217({
    visible: (__VLS_ctx.fileOpsVisible),
    agent: (__VLS_ctx.actionAgent),
}));
const __VLS_219 = __VLS_218({
    visible: (__VLS_ctx.fileOpsVisible),
    agent: (__VLS_ctx.actionAgent),
}, ...__VLS_functionalComponentArgsRest(__VLS_218));
const __VLS_222 = AgentContextMenu;
// @ts-ignore
const __VLS_223 = __VLS_asFunctionalComponent1(__VLS_222, new __VLS_222({
    ...{ 'onClose': {} },
    ...{ 'onOpenTask': {} },
    ...{ 'onOpenTerminal': {} },
    ...{ 'onOpenFileOps': {} },
    ...{ 'onScreenshot': {} },
    ...{ 'onPs': {} },
    ...{ 'onAction': {} },
    visible: (__VLS_ctx.contextMenuState.visible),
    x: (__VLS_ctx.contextMenuState.x),
    y: (__VLS_ctx.contextMenuState.y),
    agent: (__VLS_ctx.contextMenuState.record),
}));
const __VLS_224 = __VLS_223({
    ...{ 'onClose': {} },
    ...{ 'onOpenTask': {} },
    ...{ 'onOpenTerminal': {} },
    ...{ 'onOpenFileOps': {} },
    ...{ 'onScreenshot': {} },
    ...{ 'onPs': {} },
    ...{ 'onAction': {} },
    visible: (__VLS_ctx.contextMenuState.visible),
    x: (__VLS_ctx.contextMenuState.x),
    y: (__VLS_ctx.contextMenuState.y),
    agent: (__VLS_ctx.contextMenuState.record),
}, ...__VLS_functionalComponentArgsRest(__VLS_223));
let __VLS_227;
const __VLS_228 = ({ close: {} },
    { onClose: (...[$event]) => {
            __VLS_ctx.contextMenuState.visible = false;
            // @ts-ignore
            [openTaskModal, handleAction, detailVisible, selectedAgent, handleAgentStoreUpdate, taskModalVisible, actionAgent, actionAgent, fileOpsVisible, contextMenuState, contextMenuState, contextMenuState, contextMenuState, contextMenuState,];
        } });
const __VLS_229 = ({ openTask: {} },
    { onOpenTask: (__VLS_ctx.openTaskModal) });
const __VLS_230 = ({ openTerminal: {} },
    { onOpenTerminal: (__VLS_ctx.openTerminal) });
const __VLS_231 = ({ openFileOps: {} },
    { onOpenFileOps: (__VLS_ctx.openFileOps) });
const __VLS_232 = ({ screenshot: {} },
    { onScreenshot: (__VLS_ctx.handleScreenshot) });
const __VLS_233 = ({ ps: {} },
    { onPs: (__VLS_ctx.handlePs) });
const __VLS_234 = ({ action: {} },
    { onAction: (__VLS_ctx.handleAction) });
var __VLS_225;
var __VLS_226;
// @ts-ignore
[openTaskModal, openTerminal, openFileOps, handleAction, handleScreenshot, handlePs,];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
