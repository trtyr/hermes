/// <reference types="../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import { FileSearchOutlined, ReloadOutlined, SearchOutlined, } from '@ant-design/icons-vue';
import dayjs from 'dayjs';
import { fetchAudits } from '@/api/audit';
const audits = ref([]);
const loading = ref(false);
const total = ref(0);
const currentPage = ref(1);
const pageSize = 25;
// Filter
const filterForm = ref({
    operator: undefined,
    action: undefined,
    target_kind: undefined,
    target_id: undefined,
});
// All known action types from server
const actionTypes = [
    'dispatch_task',
    'broadcast_task',
    'cancel_task',
    'open_command_session',
    'queue_command_session',
    'execute_command_session',
    'close_command_session',
    'disconnect_agent',
    'disable_agent',
    'enable_agent',
    'delete_agent',
    'upload_file',
    'download_file',
    'create_listener',
    'update_listener',
    'enable_listener',
    'disable_listener',
    'delete_listener',
    'create_listener_agent_build',
    'create_agent_build',
    'update_beacon_config',
    'open_terminal_session',
    'queue_terminal_command',
    'close_terminal_session',
];
const targetKinds = [
    'agent',
    'task',
    'command_session',
    'listener',
    'agent_build',
    'terminal_session',
];
const columns = [
    { title: 'ID', dataIndex: 'audit_id', key: 'audit_id', width: 80 },
    { title: '操作者', dataIndex: 'operator', key: 'operator', width: 120 },
    { title: '操作', dataIndex: 'action', key: 'action', width: 170 },
    { title: '目标', key: 'target', width: 200 },
    { title: '详情', dataIndex: 'detail', key: 'detail', width: 280, ellipsis: true },
    { title: '时间', dataIndex: 'created_at', key: 'created_at', width: 170 },
];
const loadAudits = async () => {
    loading.value = true;
    try {
        const filter = {
            limit: pageSize,
            offset: (currentPage.value - 1) * pageSize,
        };
        if (filterForm.value.operator)
            filter.operator = filterForm.value.operator;
        if (filterForm.value.action)
            filter.action = filterForm.value.action;
        if (filterForm.value.target_kind)
            filter.target_kind = filterForm.value.target_kind;
        if (filterForm.value.target_id)
            filter.target_id = filterForm.value.target_id;
        const res = await fetchAudits(filter);
        audits.value = res.audits || [];
        total.value = res.total || 0;
    }
    catch (err) {
        message.error(err.message || '获取审计日志失败');
    }
    finally {
        loading.value = false;
    }
};
const onPageChange = (page) => {
    currentPage.value = page;
    loadAudits();
};
const applyFilter = () => {
    currentPage.value = 1;
    loadAudits();
};
const resetFilter = () => {
    filterForm.value = {
        operator: undefined,
        action: undefined,
        target_kind: undefined,
        target_id: undefined,
    };
    currentPage.value = 1;
    loadAudits();
};
onMounted(() => {
    loadAudits();
});
// Formatting Helpers
const formatTimestamp = (ts) => {
    if (!ts)
        return '-';
    const ms = ts < 1e12 ? ts * 1000 : ts;
    return dayjs(ms).format('YYYY-MM-DD HH:mm:ss');
};
const formatAction = (action) => {
    return action?.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase()) || '-';
};
const formatTargetKind = (kind) => {
    const map = {
        agent: 'Agent',
        task: 'Task',
        command_session: 'Session',
        listener: 'Listener',
        agent_build: 'Build',
        terminal_session: 'Terminal',
    };
    return map[kind] || kind || '-';
};
const getActionColor = (action) => {
    if (action?.includes('create') || action?.includes('enable') || action?.includes('open'))
        return 'green';
    if (action?.includes('delete') || action?.includes('disable') || action?.includes('disconnect') || action?.includes('close'))
        return 'red';
    if (action?.includes('update') || action?.includes('dispatch') || action?.includes('queue') || action?.includes('execute'))
        return 'blue';
    if (action?.includes('cancel'))
        return 'orange';
    if (action?.includes('upload') || action?.includes('download'))
        return 'purple';
    return 'default';
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
/** @ts-ignore @type {typeof __VLS_components.FileSearchOutlined} */
FileSearchOutlined;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    ...{ class: "text-cyan-500" },
}));
const __VLS_2 = __VLS_1({
    ...{ class: "text-cyan-500" },
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
/** @type {__VLS_StyleScopedClasses['text-cyan-500']} */ ;
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
    { onClick: (__VLS_ctx.loadAudits) });
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
    [loading, loadAudits,];
}
// @ts-ignore
[];
var __VLS_8;
var __VLS_9;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "mb-4 flex flex-wrap gap-3 items-center" },
});
/** @type {__VLS_StyleScopedClasses['mb-4']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-wrap']} */ ;
/** @type {__VLS_StyleScopedClasses['gap-3']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
let __VLS_19;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_20 = __VLS_asFunctionalComponent1(__VLS_19, new __VLS_19({
    ...{ 'onPressEnter': {} },
    value: (__VLS_ctx.filterForm.operator),
    placeholder: "操作者",
    allowClear: true,
    ...{ class: "!w-40" },
}));
const __VLS_21 = __VLS_20({
    ...{ 'onPressEnter': {} },
    value: (__VLS_ctx.filterForm.operator),
    placeholder: "操作者",
    allowClear: true,
    ...{ class: "!w-40" },
}, ...__VLS_functionalComponentArgsRest(__VLS_20));
let __VLS_24;
const __VLS_25 = ({ pressEnter: {} },
    { onPressEnter: (__VLS_ctx.applyFilter) });
/** @type {__VLS_StyleScopedClasses['!w-40']} */ ;
var __VLS_22;
var __VLS_23;
let __VLS_26;
/** @ts-ignore @type {typeof __VLS_components.aSelect | typeof __VLS_components.ASelect | typeof __VLS_components.aSelect | typeof __VLS_components.ASelect} */
aSelect;
// @ts-ignore
const __VLS_27 = __VLS_asFunctionalComponent1(__VLS_26, new __VLS_26({
    ...{ 'onChange': {} },
    value: (__VLS_ctx.filterForm.action),
    allowClear: true,
    placeholder: "操作类型",
    ...{ class: "!w-48" },
}));
const __VLS_28 = __VLS_27({
    ...{ 'onChange': {} },
    value: (__VLS_ctx.filterForm.action),
    allowClear: true,
    placeholder: "操作类型",
    ...{ class: "!w-48" },
}, ...__VLS_functionalComponentArgsRest(__VLS_27));
let __VLS_31;
const __VLS_32 = ({ change: {} },
    { onChange: (__VLS_ctx.applyFilter) });
/** @type {__VLS_StyleScopedClasses['!w-48']} */ ;
const { default: __VLS_33 } = __VLS_29.slots;
for (const [a] of __VLS_vFor((__VLS_ctx.actionTypes))) {
    let __VLS_34;
    /** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
    aSelectOption;
    // @ts-ignore
    const __VLS_35 = __VLS_asFunctionalComponent1(__VLS_34, new __VLS_34({
        key: (a),
        value: (a),
    }));
    const __VLS_36 = __VLS_35({
        key: (a),
        value: (a),
    }, ...__VLS_functionalComponentArgsRest(__VLS_35));
    const { default: __VLS_39 } = __VLS_37.slots;
    (__VLS_ctx.formatAction(a));
    // @ts-ignore
    [filterForm, filterForm, applyFilter, applyFilter, actionTypes, formatAction,];
    var __VLS_37;
    // @ts-ignore
    [];
}
// @ts-ignore
[];
var __VLS_29;
var __VLS_30;
let __VLS_40;
/** @ts-ignore @type {typeof __VLS_components.aSelect | typeof __VLS_components.ASelect | typeof __VLS_components.aSelect | typeof __VLS_components.ASelect} */
aSelect;
// @ts-ignore
const __VLS_41 = __VLS_asFunctionalComponent1(__VLS_40, new __VLS_40({
    ...{ 'onChange': {} },
    value: (__VLS_ctx.filterForm.target_kind),
    allowClear: true,
    placeholder: "目标类型",
    ...{ class: "!w-36" },
}));
const __VLS_42 = __VLS_41({
    ...{ 'onChange': {} },
    value: (__VLS_ctx.filterForm.target_kind),
    allowClear: true,
    placeholder: "目标类型",
    ...{ class: "!w-36" },
}, ...__VLS_functionalComponentArgsRest(__VLS_41));
let __VLS_45;
const __VLS_46 = ({ change: {} },
    { onChange: (__VLS_ctx.applyFilter) });
/** @type {__VLS_StyleScopedClasses['!w-36']} */ ;
const { default: __VLS_47 } = __VLS_43.slots;
for (const [k] of __VLS_vFor((__VLS_ctx.targetKinds))) {
    let __VLS_48;
    /** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
    aSelectOption;
    // @ts-ignore
    const __VLS_49 = __VLS_asFunctionalComponent1(__VLS_48, new __VLS_48({
        key: (k),
        value: (k),
    }));
    const __VLS_50 = __VLS_49({
        key: (k),
        value: (k),
    }, ...__VLS_functionalComponentArgsRest(__VLS_49));
    const { default: __VLS_53 } = __VLS_51.slots;
    (__VLS_ctx.formatTargetKind(k));
    // @ts-ignore
    [filterForm, applyFilter, targetKinds, formatTargetKind,];
    var __VLS_51;
    // @ts-ignore
    [];
}
// @ts-ignore
[];
var __VLS_43;
var __VLS_44;
let __VLS_54;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_55 = __VLS_asFunctionalComponent1(__VLS_54, new __VLS_54({
    ...{ 'onPressEnter': {} },
    value: (__VLS_ctx.filterForm.target_id),
    placeholder: "目标 ID",
    allowClear: true,
    ...{ class: "!w-32" },
}));
const __VLS_56 = __VLS_55({
    ...{ 'onPressEnter': {} },
    value: (__VLS_ctx.filterForm.target_id),
    placeholder: "目标 ID",
    allowClear: true,
    ...{ class: "!w-32" },
}, ...__VLS_functionalComponentArgsRest(__VLS_55));
let __VLS_59;
const __VLS_60 = ({ pressEnter: {} },
    { onPressEnter: (__VLS_ctx.applyFilter) });
/** @type {__VLS_StyleScopedClasses['!w-32']} */ ;
var __VLS_57;
var __VLS_58;
let __VLS_61;
/** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
aButton;
// @ts-ignore
const __VLS_62 = __VLS_asFunctionalComponent1(__VLS_61, new __VLS_61({
    ...{ 'onClick': {} },
    type: "primary",
}));
const __VLS_63 = __VLS_62({
    ...{ 'onClick': {} },
    type: "primary",
}, ...__VLS_functionalComponentArgsRest(__VLS_62));
let __VLS_66;
const __VLS_67 = ({ click: {} },
    { onClick: (__VLS_ctx.applyFilter) });
const { default: __VLS_68 } = __VLS_64.slots;
{
    const { icon: __VLS_69 } = __VLS_64.slots;
    let __VLS_70;
    /** @ts-ignore @type {typeof __VLS_components.SearchOutlined} */
    SearchOutlined;
    // @ts-ignore
    const __VLS_71 = __VLS_asFunctionalComponent1(__VLS_70, new __VLS_70({}));
    const __VLS_72 = __VLS_71({}, ...__VLS_functionalComponentArgsRest(__VLS_71));
    // @ts-ignore
    [filterForm, applyFilter, applyFilter,];
}
// @ts-ignore
[];
var __VLS_64;
var __VLS_65;
let __VLS_75;
/** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
aButton;
// @ts-ignore
const __VLS_76 = __VLS_asFunctionalComponent1(__VLS_75, new __VLS_75({
    ...{ 'onClick': {} },
}));
const __VLS_77 = __VLS_76({
    ...{ 'onClick': {} },
}, ...__VLS_functionalComponentArgsRest(__VLS_76));
let __VLS_80;
const __VLS_81 = ({ click: {} },
    { onClick: (__VLS_ctx.resetFilter) });
const { default: __VLS_82 } = __VLS_78.slots;
// @ts-ignore
[resetFilter,];
var __VLS_78;
var __VLS_79;
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
let __VLS_83;
/** @ts-ignore @type {typeof __VLS_components.aTable | typeof __VLS_components.ATable | typeof __VLS_components.aTable | typeof __VLS_components.ATable} */
aTable;
// @ts-ignore
const __VLS_84 = __VLS_asFunctionalComponent1(__VLS_83, new __VLS_83({
    columns: (__VLS_ctx.columns),
    dataSource: (__VLS_ctx.audits),
    rowKey: "audit_id",
    loading: (__VLS_ctx.loading),
    pagination: ({
        pageSize: __VLS_ctx.pageSize,
        total: __VLS_ctx.total,
        current: __VLS_ctx.currentPage,
        onChange: __VLS_ctx.onPageChange,
        showTotal: (t) => `共 ${t} 条`,
        showSizeChanger: false,
    }),
    ...{ class: "w-full flex-1" },
    scroll: ({ y: 'max-content' }),
}));
const __VLS_85 = __VLS_84({
    columns: (__VLS_ctx.columns),
    dataSource: (__VLS_ctx.audits),
    rowKey: "audit_id",
    loading: (__VLS_ctx.loading),
    pagination: ({
        pageSize: __VLS_ctx.pageSize,
        total: __VLS_ctx.total,
        current: __VLS_ctx.currentPage,
        onChange: __VLS_ctx.onPageChange,
        showTotal: (t) => `共 ${t} 条`,
        showSizeChanger: false,
    }),
    ...{ class: "w-full flex-1" },
    scroll: ({ y: 'max-content' }),
}, ...__VLS_functionalComponentArgsRest(__VLS_84));
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
const { default: __VLS_88 } = __VLS_86.slots;
{
    const { bodyCell: __VLS_89 } = __VLS_86.slots;
    const [{ column, record }] = __VLS_vSlot(__VLS_89);
    if (column.key === 'audit_id') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "font-mono text-sm text-slate-400" },
        });
        /** @type {__VLS_StyleScopedClasses['font-mono']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
        (record.audit_id);
    }
    else if (column.key === 'operator') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "font-medium text-slate-700 dark:text-[var(--text-secondary)]" },
        });
        /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-slate-700']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-[var(--text-secondary)]']} */ ;
        (record.operator);
    }
    else if (column.key === 'action') {
        let __VLS_90;
        /** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
        aTag;
        // @ts-ignore
        const __VLS_91 = __VLS_asFunctionalComponent1(__VLS_90, new __VLS_90({
            color: (__VLS_ctx.getActionColor(record.action)),
            ...{ class: "font-medium mr-0" },
        }));
        const __VLS_92 = __VLS_91({
            color: (__VLS_ctx.getActionColor(record.action)),
            ...{ class: "font-medium mr-0" },
        }, ...__VLS_functionalComponentArgsRest(__VLS_91));
        /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
        /** @type {__VLS_StyleScopedClasses['mr-0']} */ ;
        const { default: __VLS_95 } = __VLS_93.slots;
        (__VLS_ctx.formatAction(record.action));
        // @ts-ignore
        [loading, formatAction, columns, audits, pageSize, total, currentPage, onPageChange, getActionColor,];
        var __VLS_93;
    }
    else if (column.key === 'target') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "flex items-center gap-2" },
        });
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
        let __VLS_96;
        /** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
        aTag;
        // @ts-ignore
        const __VLS_97 = __VLS_asFunctionalComponent1(__VLS_96, new __VLS_96({
            size: "small",
            ...{ class: "mr-0" },
        }));
        const __VLS_98 = __VLS_97({
            size: "small",
            ...{ class: "mr-0" },
        }, ...__VLS_functionalComponentArgsRest(__VLS_97));
        /** @type {__VLS_StyleScopedClasses['mr-0']} */ ;
        const { default: __VLS_101 } = __VLS_99.slots;
        (__VLS_ctx.formatTargetKind(record.target_kind));
        // @ts-ignore
        [formatTargetKind,];
        var __VLS_99;
        if (record.target_id) {
            __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
                ...{ class: "font-mono text-sm text-slate-500 dark:text-[var(--text-secondary)]" },
            });
            /** @type {__VLS_StyleScopedClasses['font-mono']} */ ;
            /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
            /** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
            /** @type {__VLS_StyleScopedClasses['dark:text-[var(--text-secondary)]']} */ ;
            (record.target_id);
        }
    }
    else if (column.key === 'detail') {
        if (record.detail) {
            let __VLS_102;
            /** @ts-ignore @type {typeof __VLS_components.aTooltip | typeof __VLS_components.ATooltip | typeof __VLS_components.aTooltip | typeof __VLS_components.ATooltip} */
            aTooltip;
            // @ts-ignore
            const __VLS_103 = __VLS_asFunctionalComponent1(__VLS_102, new __VLS_102({
                title: (record.detail),
            }));
            const __VLS_104 = __VLS_103({
                title: (record.detail),
            }, ...__VLS_functionalComponentArgsRest(__VLS_103));
            const { default: __VLS_107 } = __VLS_105.slots;
            __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
                ...{ class: "text-sm text-slate-500 dark:text-[var(--text-secondary)] truncate max-w-[260px] inline-block align-bottom" },
            });
            /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
            /** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
            /** @type {__VLS_StyleScopedClasses['dark:text-[var(--text-secondary)]']} */ ;
            /** @type {__VLS_StyleScopedClasses['truncate']} */ ;
            /** @type {__VLS_StyleScopedClasses['max-w-[260px]']} */ ;
            /** @type {__VLS_StyleScopedClasses['inline-block']} */ ;
            /** @type {__VLS_StyleScopedClasses['align-bottom']} */ ;
            (record.detail);
            // @ts-ignore
            [];
            var __VLS_105;
        }
        else {
            __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
                ...{ class: "text-slate-400" },
            });
            /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
        }
    }
    else if (column.key === 'created_at') {
        (__VLS_ctx.formatTimestamp(record.created_at));
    }
    // @ts-ignore
    [formatTimestamp,];
}
// @ts-ignore
[];
var __VLS_86;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
