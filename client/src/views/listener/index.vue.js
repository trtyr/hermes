/// <reference types="../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import { ApiOutlined, ReloadOutlined, PlusOutlined, } from '@ant-design/icons-vue';
import dayjs from 'dayjs';
import { fetchListeners, startListener, stopListener, deleteListener } from '@/api/listener';
import CreateListenerModal from './components/CreateListenerModal.vue';
const listeners = ref([]);
const loading = ref(false);
const createModalVisible = ref(false);
const actionLoading = ref('');
const columns = [
    { title: '标识 ID', dataIndex: 'id', key: 'id', width: 120, ellipsis: true },
    { title: '名称', dataIndex: 'name', key: 'name', width: 200 },
    { title: '协议', dataIndex: 'protocol', key: 'protocol', width: 100 },
    { title: '侦听地址', key: 'address', width: 200 },
    { title: '当前状态', dataIndex: 'status', key: 'status', width: 140 },
    { title: '创建时间', dataIndex: 'created_at', key: 'created_at', width: 180 },
    { title: '操作', key: 'action', width: 220, fixed: 'right' },
];
const loadListeners = async () => {
    loading.value = true;
    try {
        const res = await fetchListeners();
        listeners.value = res.listeners || [];
    }
    catch (err) {
        message.error(err.message || '获取监听器列表失败');
    }
    finally {
        loading.value = false;
    }
};
onMounted(() => {
    loadListeners();
});
// Row Actions
const doStartListener = async (record) => {
    actionLoading.value = record.id + 'start';
    try {
        await startListener(record.id);
        message.success(`监听器 ${record.name} 已启动`);
        await loadListeners();
    }
    catch (err) {
        message.error(err.message || '操作失败');
    }
    finally {
        actionLoading.value = '';
    }
};
const doStopListener = async (record) => {
    actionLoading.value = record.id + 'stop';
    try {
        await stopListener(record.id);
        message.success(`监听器 ${record.name} 已停止`);
        await loadListeners();
    }
    catch (err) {
        message.error(err.message || '操作失败');
    }
    finally {
        actionLoading.value = '';
    }
};
const doDeleteListener = async (record) => {
    actionLoading.value = record.id + 'delete';
    try {
        await deleteListener(record.id);
        message.success(`监听器 ${record.name} 已删除`);
        await loadListeners();
    }
    catch (err) {
        message.error(err.message || '操作失败');
    }
    finally {
        actionLoading.value = '';
    }
};
// Formatting Helpers
const formatTimestamp = (ts) => {
    if (!ts)
        return '-';
    // Attempt to detect if it's seconds vs milliseconds
    const ms = ts < 1e12 ? ts * 1000 : ts;
    return dayjs(ms).format('YYYY-MM-DD HH:mm:ss');
};
const getProtocolColor = (proto) => {
    const p = proto?.toUpperCase() || '';
    if (p === 'TCP')
        return 'blue';
    if (p === 'HTTP' || p === 'HTTPS')
        return 'purple';
    if (p === 'DNS')
        return 'cyan';
    return 'default';
};
const getStatusDotColor = (status) => {
    if (status === 'running')
        return 'bg-green-500';
    if (status === 'stopped')
        return 'bg-slate-400';
    if (status === 'error')
        return 'bg-red-500';
    if (status === 'starting')
        return 'bg-blue-400';
    return 'bg-slate-300';
};
const getStatusTextColor = (status) => {
    if (status === 'running')
        return 'text-green-600 dark:text-green-500 font-medium';
    if (status === 'stopped')
        return 'text-slate-500 dark:text-[var(--text-secondary)]';
    if (status === 'error')
        return 'text-red-600 dark:text-red-500 font-medium';
    return 'text-slate-700 dark:text-[var(--text-secondary)]';
};
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
/** @ts-ignore @type {typeof __VLS_components.ApiOutlined} */
ApiOutlined;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    ...{ class: "text-indigo-500" },
}));
const __VLS_2 = __VLS_1({
    ...{ class: "text-indigo-500" },
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
/** @type {__VLS_StyleScopedClasses['text-indigo-500']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex gap-2" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
let __VLS_5;
/** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
aButton;
// @ts-ignore
const __VLS_6 = __VLS_asFunctionalComponent1(__VLS_5, new __VLS_5({
    ...{ 'onClick': {} },
    loading: (__VLS_ctx.loading),
}));
const __VLS_7 = __VLS_6({
    ...{ 'onClick': {} },
    loading: (__VLS_ctx.loading),
}, ...__VLS_functionalComponentArgsRest(__VLS_6));
let __VLS_10;
const __VLS_11 = ({ click: {} },
    { onClick: (__VLS_ctx.loadListeners) });
const { default: __VLS_12 } = __VLS_8.slots;
{
    const { icon: __VLS_13 } = __VLS_8.slots;
    let __VLS_14;
    /** @ts-ignore @type {typeof __VLS_components.ReloadOutlined} */
    ReloadOutlined;
    // @ts-ignore
    const __VLS_15 = __VLS_asFunctionalComponent1(__VLS_14, new __VLS_14({}));
    const __VLS_16 = __VLS_15({}, ...__VLS_functionalComponentArgsRest(__VLS_15));
    // @ts-ignore
    [loading, loadListeners,];
}
// @ts-ignore
[];
var __VLS_8;
var __VLS_9;
let __VLS_19;
/** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
aButton;
// @ts-ignore
const __VLS_20 = __VLS_asFunctionalComponent1(__VLS_19, new __VLS_19({
    ...{ 'onClick': {} },
    type: "primary",
}));
const __VLS_21 = __VLS_20({
    ...{ 'onClick': {} },
    type: "primary",
}, ...__VLS_functionalComponentArgsRest(__VLS_20));
let __VLS_24;
const __VLS_25 = ({ click: {} },
    { onClick: (...[$event]) => {
            __VLS_ctx.createModalVisible = true;
            // @ts-ignore
            [createModalVisible,];
        } });
const { default: __VLS_26 } = __VLS_22.slots;
{
    const { icon: __VLS_27 } = __VLS_22.slots;
    let __VLS_28;
    /** @ts-ignore @type {typeof __VLS_components.PlusOutlined} */
    PlusOutlined;
    // @ts-ignore
    const __VLS_29 = __VLS_asFunctionalComponent1(__VLS_28, new __VLS_28({}));
    const __VLS_30 = __VLS_29({}, ...__VLS_functionalComponentArgsRest(__VLS_29));
    // @ts-ignore
    [];
}
// @ts-ignore
[];
var __VLS_22;
var __VLS_23;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex-1 bg-white dark:bg-[var(--bg-card)] rounded-lg border border-gray-200 dark:border-[var(--border-default)] shadow-sm flex flex-col overflow-hidden" },
});
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-card)]']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-lg']} */ ;
/** @type {__VLS_StyleScopedClasses['border']} */ ;
/** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:border-[var(--border-default)]']} */ ;
/** @type {__VLS_StyleScopedClasses['shadow-sm']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-hidden']} */ ;
let __VLS_33;
/** @ts-ignore @type {typeof __VLS_components.aTable | typeof __VLS_components.ATable | typeof __VLS_components.aTable | typeof __VLS_components.ATable} */
aTable;
// @ts-ignore
const __VLS_34 = __VLS_asFunctionalComponent1(__VLS_33, new __VLS_33({
    columns: (__VLS_ctx.columns),
    dataSource: (__VLS_ctx.listeners),
    rowKey: "id",
    loading: (__VLS_ctx.loading),
    pagination: ({ pageSize: 20 }),
    ...{ class: "w-full flex-1" },
    scroll: ({ y: 'max-content' }),
}));
const __VLS_35 = __VLS_34({
    columns: (__VLS_ctx.columns),
    dataSource: (__VLS_ctx.listeners),
    rowKey: "id",
    loading: (__VLS_ctx.loading),
    pagination: ({ pageSize: 20 }),
    ...{ class: "w-full flex-1" },
    scroll: ({ y: 'max-content' }),
}, ...__VLS_functionalComponentArgsRest(__VLS_34));
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
const { default: __VLS_38 } = __VLS_36.slots;
{
    const { bodyCell: __VLS_39 } = __VLS_36.slots;
    const [{ column, record }] = __VLS_vSlot(__VLS_39);
    if (column.key === 'protocol') {
        let __VLS_40;
        /** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
        aTag;
        // @ts-ignore
        const __VLS_41 = __VLS_asFunctionalComponent1(__VLS_40, new __VLS_40({
            color: (__VLS_ctx.getProtocolColor(record.protocol)),
            ...{ class: "font-medium mr-0" },
        }));
        const __VLS_42 = __VLS_41({
            color: (__VLS_ctx.getProtocolColor(record.protocol)),
            ...{ class: "font-medium mr-0" },
        }, ...__VLS_functionalComponentArgsRest(__VLS_41));
        /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
        /** @type {__VLS_StyleScopedClasses['mr-0']} */ ;
        const { default: __VLS_45 } = __VLS_43.slots;
        (record.protocol);
        // @ts-ignore
        [loading, columns, listeners, getProtocolColor,];
        var __VLS_43;
    }
    else if (column.key === 'address') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "font-mono text-sm text-slate-600 dark:text-[var(--text-secondary)]" },
        });
        /** @type {__VLS_StyleScopedClasses['font-mono']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-slate-600']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-[var(--text-secondary)]']} */ ;
        (record.bind_host);
        (record.bind_port);
    }
    else if (column.key === 'status') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "flex items-center gap-2" },
        });
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "relative flex h-2.5 w-2.5 shrink-0" },
        });
        /** @type {__VLS_StyleScopedClasses['relative']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['h-2.5']} */ ;
        /** @type {__VLS_StyleScopedClasses['w-2.5']} */ ;
        /** @type {__VLS_StyleScopedClasses['shrink-0']} */ ;
        if (record.status === 'running') {
            __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
                ...{ class: "absolute inline-flex h-full w-full animate-ping rounded-full bg-green-400 opacity-75" },
            });
            /** @type {__VLS_StyleScopedClasses['absolute']} */ ;
            /** @type {__VLS_StyleScopedClasses['inline-flex']} */ ;
            /** @type {__VLS_StyleScopedClasses['h-full']} */ ;
            /** @type {__VLS_StyleScopedClasses['w-full']} */ ;
            /** @type {__VLS_StyleScopedClasses['animate-ping']} */ ;
            /** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
            /** @type {__VLS_StyleScopedClasses['bg-green-400']} */ ;
            /** @type {__VLS_StyleScopedClasses['opacity-75']} */ ;
        }
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "relative inline-flex h-2.5 w-2.5 rounded-full" },
            ...{ class: (__VLS_ctx.getStatusDotColor(record.status)) },
        });
        /** @type {__VLS_StyleScopedClasses['relative']} */ ;
        /** @type {__VLS_StyleScopedClasses['inline-flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['h-2.5']} */ ;
        /** @type {__VLS_StyleScopedClasses['w-2.5']} */ ;
        /** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: (__VLS_ctx.getStatusTextColor(record.status)) },
            ...{ class: "capitalize" },
        });
        /** @type {__VLS_StyleScopedClasses['capitalize']} */ ;
        (record.status);
    }
    else if (column.key === 'created_at') {
        (__VLS_ctx.formatTimestamp(record.created_at));
    }
    else if (column.key === 'action') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "flex gap-2 items-center" },
        });
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        if (record.status !== 'running') {
            let __VLS_46;
            /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
            aButton;
            // @ts-ignore
            const __VLS_47 = __VLS_asFunctionalComponent1(__VLS_46, new __VLS_46({
                ...{ 'onClick': {} },
                type: "text",
                size: "small",
                ...{ class: "text-green-600 dark:text-green-400 hover:text-green-700 hover:bg-green-50 dark:hover:bg-green-900/30" },
                loading: (__VLS_ctx.actionLoading === record.id + 'start'),
            }));
            const __VLS_48 = __VLS_47({
                ...{ 'onClick': {} },
                type: "text",
                size: "small",
                ...{ class: "text-green-600 dark:text-green-400 hover:text-green-700 hover:bg-green-50 dark:hover:bg-green-900/30" },
                loading: (__VLS_ctx.actionLoading === record.id + 'start'),
            }, ...__VLS_functionalComponentArgsRest(__VLS_47));
            let __VLS_51;
            const __VLS_52 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!!(column.key === 'protocol'))
                            return;
                        if (!!(column.key === 'address'))
                            return;
                        if (!!(column.key === 'status'))
                            return;
                        if (!!(column.key === 'created_at'))
                            return;
                        if (!(column.key === 'action'))
                            return;
                        if (!(record.status !== 'running'))
                            return;
                        __VLS_ctx.doStartListener(record);
                        // @ts-ignore
                        [getStatusDotColor, getStatusTextColor, formatTimestamp, actionLoading, doStartListener,];
                    } });
            /** @type {__VLS_StyleScopedClasses['text-green-600']} */ ;
            /** @type {__VLS_StyleScopedClasses['dark:text-green-400']} */ ;
            /** @type {__VLS_StyleScopedClasses['hover:text-green-700']} */ ;
            /** @type {__VLS_StyleScopedClasses['hover:bg-green-50']} */ ;
            /** @type {__VLS_StyleScopedClasses['dark:hover:bg-green-900/30']} */ ;
            const { default: __VLS_53 } = __VLS_49.slots;
            // @ts-ignore
            [];
            var __VLS_49;
            var __VLS_50;
        }
        if (record.status === 'running') {
            let __VLS_54;
            /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
            aButton;
            // @ts-ignore
            const __VLS_55 = __VLS_asFunctionalComponent1(__VLS_54, new __VLS_54({
                ...{ 'onClick': {} },
                type: "text",
                size: "small",
                ...{ class: "text-amber-600 dark:text-amber-500 hover:text-amber-700 hover:bg-amber-50 dark:hover:bg-amber-900/30" },
                loading: (__VLS_ctx.actionLoading === record.id + 'stop'),
            }));
            const __VLS_56 = __VLS_55({
                ...{ 'onClick': {} },
                type: "text",
                size: "small",
                ...{ class: "text-amber-600 dark:text-amber-500 hover:text-amber-700 hover:bg-amber-50 dark:hover:bg-amber-900/30" },
                loading: (__VLS_ctx.actionLoading === record.id + 'stop'),
            }, ...__VLS_functionalComponentArgsRest(__VLS_55));
            let __VLS_59;
            const __VLS_60 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!!(column.key === 'protocol'))
                            return;
                        if (!!(column.key === 'address'))
                            return;
                        if (!!(column.key === 'status'))
                            return;
                        if (!!(column.key === 'created_at'))
                            return;
                        if (!(column.key === 'action'))
                            return;
                        if (!(record.status === 'running'))
                            return;
                        __VLS_ctx.doStopListener(record);
                        // @ts-ignore
                        [actionLoading, doStopListener,];
                    } });
            /** @type {__VLS_StyleScopedClasses['text-amber-600']} */ ;
            /** @type {__VLS_StyleScopedClasses['dark:text-amber-500']} */ ;
            /** @type {__VLS_StyleScopedClasses['hover:text-amber-700']} */ ;
            /** @type {__VLS_StyleScopedClasses['hover:bg-amber-50']} */ ;
            /** @type {__VLS_StyleScopedClasses['dark:hover:bg-amber-900/30']} */ ;
            const { default: __VLS_61 } = __VLS_57.slots;
            // @ts-ignore
            [];
            var __VLS_57;
            var __VLS_58;
        }
        let __VLS_62;
        /** @ts-ignore @type {typeof __VLS_components.aPopconfirm | typeof __VLS_components.APopconfirm | typeof __VLS_components.aPopconfirm | typeof __VLS_components.APopconfirm} */
        aPopconfirm;
        // @ts-ignore
        const __VLS_63 = __VLS_asFunctionalComponent1(__VLS_62, new __VLS_62({
            ...{ 'onConfirm': {} },
            title: "确定要彻底删除该监听器吗？",
            okText: "删除",
            cancelText: "取消",
            okType: "danger",
        }));
        const __VLS_64 = __VLS_63({
            ...{ 'onConfirm': {} },
            title: "确定要彻底删除该监听器吗？",
            okText: "删除",
            cancelText: "取消",
            okType: "danger",
        }, ...__VLS_functionalComponentArgsRest(__VLS_63));
        let __VLS_67;
        const __VLS_68 = ({ confirm: {} },
            { onConfirm: (...[$event]) => {
                    if (!!(column.key === 'protocol'))
                        return;
                    if (!!(column.key === 'address'))
                        return;
                    if (!!(column.key === 'status'))
                        return;
                    if (!!(column.key === 'created_at'))
                        return;
                    if (!(column.key === 'action'))
                        return;
                    __VLS_ctx.doDeleteListener(record);
                    // @ts-ignore
                    [doDeleteListener,];
                } });
        const { default: __VLS_69 } = __VLS_65.slots;
        let __VLS_70;
        /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
        aButton;
        // @ts-ignore
        const __VLS_71 = __VLS_asFunctionalComponent1(__VLS_70, new __VLS_70({
            type: "text",
            size: "small",
            danger: true,
            loading: (__VLS_ctx.actionLoading === record.id + 'delete'),
        }));
        const __VLS_72 = __VLS_71({
            type: "text",
            size: "small",
            danger: true,
            loading: (__VLS_ctx.actionLoading === record.id + 'delete'),
        }, ...__VLS_functionalComponentArgsRest(__VLS_71));
        const { default: __VLS_75 } = __VLS_73.slots;
        // @ts-ignore
        [actionLoading,];
        var __VLS_73;
        // @ts-ignore
        [];
        var __VLS_65;
        var __VLS_66;
    }
    // @ts-ignore
    [];
}
// @ts-ignore
[];
var __VLS_36;
const __VLS_76 = CreateListenerModal;
// @ts-ignore
const __VLS_77 = __VLS_asFunctionalComponent1(__VLS_76, new __VLS_76({
    ...{ 'onSuccess': {} },
    visible: (__VLS_ctx.createModalVisible),
}));
const __VLS_78 = __VLS_77({
    ...{ 'onSuccess': {} },
    visible: (__VLS_ctx.createModalVisible),
}, ...__VLS_functionalComponentArgsRest(__VLS_77));
let __VLS_81;
const __VLS_82 = ({ success: {} },
    { onSuccess: (__VLS_ctx.loadListeners) });
var __VLS_79;
var __VLS_80;
// @ts-ignore
[loadListeners, createModalVisible,];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
