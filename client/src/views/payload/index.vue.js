/// <reference types="../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, onMounted } from 'vue';
import { message } from 'ant-design-vue';
import { RocketOutlined, ReloadOutlined, PlusOutlined, } from '@ant-design/icons-vue';
import dayjs from 'dayjs';
import { fetchAgentBuilds, createAgentBuild, } from '@/api/agentBuild';
import { fetchListeners } from '@/api/listener';
const builds = ref([]);
const loading = ref(false);
const total = ref(0);
const currentPage = ref(1);
const pageSize = 20;
// Build form
const buildModalVisible = ref(false);
const building = ref(false);
const buildForm = ref({
    target_triple: undefined,
    listener_id: undefined,
    server_addr: '',
    agent_token: '',
    profile: 'release',
});
// Listeners for the form dropdown
const listeners = ref([]);
const listenersLoading = ref(false);
const columns = [
    { title: '构建 ID', dataIndex: 'build_id', key: 'build_id', width: 100 },
    { title: '目标平台', dataIndex: 'target_triple', key: 'target_triple', width: 220 },
    { title: '配置', dataIndex: 'profile', key: 'profile', width: 90 },
    { title: '监听器', dataIndex: 'listener_id', key: 'listener_id', width: 100 },
    { title: '回连地址', dataIndex: 'server_addr', key: 'server_addr', width: 180 },
    { title: '状态', dataIndex: 'status', key: 'status', width: 120 },
    { title: '详情', dataIndex: 'detail', key: 'detail', width: 200, ellipsis: true },
    { title: '创建时间', dataIndex: 'created_at', key: 'created_at', width: 170 },
];
const loadBuilds = async () => {
    loading.value = true;
    try {
        const res = await fetchAgentBuilds({
            limit: pageSize,
            offset: (currentPage.value - 1) * pageSize,
        });
        builds.value = res.builds || [];
        total.value = res.total || 0;
    }
    catch (err) {
        message.error(err.message || '获取构建列表失败');
    }
    finally {
        loading.value = false;
    }
};
const loadListeners = async () => {
    listenersLoading.value = true;
    try {
        const res = await fetchListeners();
        listeners.value = res.listeners || [];
    }
    catch {
        // Silently fail — listeners are optional for the form
    }
    finally {
        listenersLoading.value = false;
    }
};
const onPageChange = (page) => {
    currentPage.value = page;
    loadBuilds();
};
onMounted(() => {
    loadBuilds();
    loadListeners();
});
// Build action
const handleBuild = async () => {
    building.value = true;
    try {
        const data = {
            profile: buildForm.value.profile,
        };
        if (buildForm.value.target_triple)
            data.target_triple = buildForm.value.target_triple;
        if (buildForm.value.listener_id)
            data.listener_id = buildForm.value.listener_id;
        if (buildForm.value.server_addr)
            data.server_addr = buildForm.value.server_addr;
        if (buildForm.value.agent_token)
            data.agent_token = buildForm.value.agent_token;
        await createAgentBuild(data);
        message.success('构建任务已提交');
        buildModalVisible.value = false;
        resetBuildForm();
        // Reload after a short delay so pending builds show up
        setTimeout(() => loadBuilds(), 500);
    }
    catch (err) {
        message.error(err.message || '创建构建失败');
    }
    finally {
        building.value = false;
    }
};
const resetBuildForm = () => {
    buildForm.value = {
        target_triple: undefined,
        listener_id: undefined,
        server_addr: '',
        agent_token: '',
        profile: 'release',
    };
};
// Formatting Helpers
const formatTimestamp = (ts) => {
    if (!ts)
        return '-';
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
    if (status === 'succeeded')
        return 'bg-green-500';
    if (status === 'pending')
        return 'bg-blue-400';
    if (status === 'failed')
        return 'bg-red-500';
    return 'bg-slate-400';
};
const getStatusTextColor = (status) => {
    if (status === 'succeeded')
        return 'text-green-600 dark:text-green-500 font-medium';
    if (status === 'pending')
        return 'text-blue-600 dark:text-blue-400 font-medium';
    if (status === 'failed')
        return 'text-red-600 dark:text-red-500 font-medium';
    return 'text-slate-500 dark:text-slate-400';
};
const getStatusLabel = (status) => {
    if (status === 'succeeded')
        return '成功';
    if (status === 'pending')
        return '构建中';
    if (status === 'failed')
        return '失败';
    return status;
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
    ...{ class: "text-xl font-semibold text-slate-800 dark:text-slate-100 flex items-center gap-2 m-0" },
});
/** @type {__VLS_StyleScopedClasses['text-xl']} */ ;
/** @type {__VLS_StyleScopedClasses['font-semibold']} */ ;
/** @type {__VLS_StyleScopedClasses['text-slate-800']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-slate-100']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
/** @type {__VLS_StyleScopedClasses['m-0']} */ ;
let __VLS_0;
/** @ts-ignore @type {typeof __VLS_components.RocketOutlined} */
RocketOutlined;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    ...{ class: "text-orange-500" },
}));
const __VLS_2 = __VLS_1({
    ...{ class: "text-orange-500" },
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
/** @type {__VLS_StyleScopedClasses['text-orange-500']} */ ;
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
    { onClick: (__VLS_ctx.loadBuilds) });
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
    [loading, loadBuilds,];
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
            __VLS_ctx.buildModalVisible = true;
            // @ts-ignore
            [buildModalVisible,];
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
    ...{ class: "flex-1 bg-white dark:bg-[#1C1E22] rounded-lg border border-gray-200 dark:border-[#14161A] shadow-sm flex flex-col overflow-hidden" },
});
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[#1C1E22]']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-lg']} */ ;
/** @type {__VLS_StyleScopedClasses['border']} */ ;
/** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:border-[#14161A]']} */ ;
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
    dataSource: (__VLS_ctx.builds),
    rowKey: "build_id",
    loading: (__VLS_ctx.loading),
    pagination: ({ pageSize: 20, total: __VLS_ctx.total, current: __VLS_ctx.currentPage, onChange: __VLS_ctx.onPageChange }),
    ...{ class: "w-full flex-1" },
    scroll: ({ y: 'max-content' }),
}));
const __VLS_35 = __VLS_34({
    columns: (__VLS_ctx.columns),
    dataSource: (__VLS_ctx.builds),
    rowKey: "build_id",
    loading: (__VLS_ctx.loading),
    pagination: ({ pageSize: 20, total: __VLS_ctx.total, current: __VLS_ctx.currentPage, onChange: __VLS_ctx.onPageChange }),
    ...{ class: "w-full flex-1" },
    scroll: ({ y: 'max-content' }),
}, ...__VLS_functionalComponentArgsRest(__VLS_34));
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
const { default: __VLS_38 } = __VLS_36.slots;
{
    const { bodyCell: __VLS_39 } = __VLS_36.slots;
    const [{ column, record }] = __VLS_vSlot(__VLS_39);
    if (column.key === 'build_id') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "font-mono text-sm" },
        });
        /** @type {__VLS_StyleScopedClasses['font-mono']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
        (record.build_id);
    }
    else if (column.key === 'target_triple') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "font-mono text-sm text-slate-600 dark:text-slate-300" },
        });
        /** @type {__VLS_StyleScopedClasses['font-mono']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-slate-600']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-slate-300']} */ ;
        (record.target_triple);
    }
    else if (column.key === 'profile') {
        let __VLS_40;
        /** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
        aTag;
        // @ts-ignore
        const __VLS_41 = __VLS_asFunctionalComponent1(__VLS_40, new __VLS_40({
            color: (record.profile === 'release' ? 'green' : 'blue'),
            ...{ class: "font-medium mr-0" },
        }));
        const __VLS_42 = __VLS_41({
            color: (record.profile === 'release' ? 'green' : 'blue'),
            ...{ class: "font-medium mr-0" },
        }, ...__VLS_functionalComponentArgsRest(__VLS_41));
        /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
        /** @type {__VLS_StyleScopedClasses['mr-0']} */ ;
        const { default: __VLS_45 } = __VLS_43.slots;
        (record.profile);
        // @ts-ignore
        [loading, columns, builds, total, currentPage, onPageChange,];
        var __VLS_43;
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
        if (record.status === 'pending') {
            __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
                ...{ class: "absolute inline-flex h-full w-full animate-ping rounded-full bg-blue-400 opacity-75" },
            });
            /** @type {__VLS_StyleScopedClasses['absolute']} */ ;
            /** @type {__VLS_StyleScopedClasses['inline-flex']} */ ;
            /** @type {__VLS_StyleScopedClasses['h-full']} */ ;
            /** @type {__VLS_StyleScopedClasses['w-full']} */ ;
            /** @type {__VLS_StyleScopedClasses['animate-ping']} */ ;
            /** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
            /** @type {__VLS_StyleScopedClasses['bg-blue-400']} */ ;
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
        });
        (__VLS_ctx.getStatusLabel(record.status));
    }
    else if (column.key === 'listener_id') {
        if (record.listener_id) {
            __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
                ...{ class: "font-mono text-sm" },
            });
            /** @type {__VLS_StyleScopedClasses['font-mono']} */ ;
            /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
            (record.listener_id);
        }
        else {
            __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
                ...{ class: "text-slate-400" },
            });
            /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
        }
    }
    else if (column.key === 'server_addr') {
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "font-mono text-sm text-slate-600 dark:text-slate-300" },
        });
        /** @type {__VLS_StyleScopedClasses['font-mono']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-slate-600']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-slate-300']} */ ;
        (record.server_addr || '-');
    }
    else if (column.key === 'created_at') {
        (__VLS_ctx.formatTimestamp(record.created_at));
    }
    else if (column.key === 'detail') {
        if (record.detail) {
            let __VLS_46;
            /** @ts-ignore @type {typeof __VLS_components.aTooltip | typeof __VLS_components.ATooltip | typeof __VLS_components.aTooltip | typeof __VLS_components.ATooltip} */
            aTooltip;
            // @ts-ignore
            const __VLS_47 = __VLS_asFunctionalComponent1(__VLS_46, new __VLS_46({
                title: (record.detail),
            }));
            const __VLS_48 = __VLS_47({
                title: (record.detail),
            }, ...__VLS_functionalComponentArgsRest(__VLS_47));
            const { default: __VLS_51 } = __VLS_49.slots;
            __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
                ...{ class: "text-sm text-slate-500 dark:text-slate-400 truncate max-w-[200px] inline-block align-bottom" },
            });
            /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
            /** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
            /** @type {__VLS_StyleScopedClasses['dark:text-slate-400']} */ ;
            /** @type {__VLS_StyleScopedClasses['truncate']} */ ;
            /** @type {__VLS_StyleScopedClasses['max-w-[200px]']} */ ;
            /** @type {__VLS_StyleScopedClasses['inline-block']} */ ;
            /** @type {__VLS_StyleScopedClasses['align-bottom']} */ ;
            (record.detail);
            // @ts-ignore
            [getStatusDotColor, getStatusTextColor, getStatusLabel, formatTimestamp,];
            var __VLS_49;
        }
        else {
            __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
                ...{ class: "text-slate-400" },
            });
            /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
        }
    }
    // @ts-ignore
    [];
}
// @ts-ignore
[];
var __VLS_36;
let __VLS_52;
/** @ts-ignore @type {typeof __VLS_components.aModal | typeof __VLS_components.AModal | typeof __VLS_components.aModal | typeof __VLS_components.AModal} */
aModal;
// @ts-ignore
const __VLS_53 = __VLS_asFunctionalComponent1(__VLS_52, new __VLS_52({
    ...{ 'onOk': {} },
    open: (__VLS_ctx.buildModalVisible),
    title: "新建载荷构建",
    width: "600px",
    confirmLoading: (__VLS_ctx.building),
    okText: "开始构建",
    cancelText: "取消",
    destroyOnClose: (true),
}));
const __VLS_54 = __VLS_53({
    ...{ 'onOk': {} },
    open: (__VLS_ctx.buildModalVisible),
    title: "新建载荷构建",
    width: "600px",
    confirmLoading: (__VLS_ctx.building),
    okText: "开始构建",
    cancelText: "取消",
    destroyOnClose: (true),
}, ...__VLS_functionalComponentArgsRest(__VLS_53));
let __VLS_57;
const __VLS_58 = ({ ok: {} },
    { onOk: (__VLS_ctx.handleBuild) });
const { default: __VLS_59 } = __VLS_55.slots;
let __VLS_60;
/** @ts-ignore @type {typeof __VLS_components.aForm | typeof __VLS_components.AForm | typeof __VLS_components.aForm | typeof __VLS_components.AForm} */
aForm;
// @ts-ignore
const __VLS_61 = __VLS_asFunctionalComponent1(__VLS_60, new __VLS_60({
    layout: "vertical",
    ...{ class: "mt-4" },
}));
const __VLS_62 = __VLS_61({
    layout: "vertical",
    ...{ class: "mt-4" },
}, ...__VLS_functionalComponentArgsRest(__VLS_61));
/** @type {__VLS_StyleScopedClasses['mt-4']} */ ;
const { default: __VLS_65 } = __VLS_63.slots;
let __VLS_66;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_67 = __VLS_asFunctionalComponent1(__VLS_66, new __VLS_66({
    label: "目标平台",
    help: "留空则使用当前服务器平台",
}));
const __VLS_68 = __VLS_67({
    label: "目标平台",
    help: "留空则使用当前服务器平台",
}, ...__VLS_functionalComponentArgsRest(__VLS_67));
const { default: __VLS_71 } = __VLS_69.slots;
let __VLS_72;
/** @ts-ignore @type {typeof __VLS_components.aSelect | typeof __VLS_components.ASelect | typeof __VLS_components.aSelect | typeof __VLS_components.ASelect} */
aSelect;
// @ts-ignore
const __VLS_73 = __VLS_asFunctionalComponent1(__VLS_72, new __VLS_72({
    value: (__VLS_ctx.buildForm.target_triple),
    allowClear: true,
    placeholder: "自动检测当前平台",
}));
const __VLS_74 = __VLS_73({
    value: (__VLS_ctx.buildForm.target_triple),
    allowClear: true,
    placeholder: "自动检测当前平台",
}, ...__VLS_functionalComponentArgsRest(__VLS_73));
const { default: __VLS_77 } = __VLS_75.slots;
let __VLS_78;
/** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
aSelectOption;
// @ts-ignore
const __VLS_79 = __VLS_asFunctionalComponent1(__VLS_78, new __VLS_78({
    value: "x86_64-pc-windows-msvc",
}));
const __VLS_80 = __VLS_79({
    value: "x86_64-pc-windows-msvc",
}, ...__VLS_functionalComponentArgsRest(__VLS_79));
const { default: __VLS_83 } = __VLS_81.slots;
// @ts-ignore
[buildModalVisible, building, handleBuild, buildForm,];
var __VLS_81;
let __VLS_84;
/** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
aSelectOption;
// @ts-ignore
const __VLS_85 = __VLS_asFunctionalComponent1(__VLS_84, new __VLS_84({
    value: "i686-pc-windows-msvc",
}));
const __VLS_86 = __VLS_85({
    value: "i686-pc-windows-msvc",
}, ...__VLS_functionalComponentArgsRest(__VLS_85));
const { default: __VLS_89 } = __VLS_87.slots;
// @ts-ignore
[];
var __VLS_87;
let __VLS_90;
/** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
aSelectOption;
// @ts-ignore
const __VLS_91 = __VLS_asFunctionalComponent1(__VLS_90, new __VLS_90({
    value: "x86_64-unknown-linux-gnu",
}));
const __VLS_92 = __VLS_91({
    value: "x86_64-unknown-linux-gnu",
}, ...__VLS_functionalComponentArgsRest(__VLS_91));
const { default: __VLS_95 } = __VLS_93.slots;
// @ts-ignore
[];
var __VLS_93;
let __VLS_96;
/** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
aSelectOption;
// @ts-ignore
const __VLS_97 = __VLS_asFunctionalComponent1(__VLS_96, new __VLS_96({
    value: "aarch64-unknown-linux-gnu",
}));
const __VLS_98 = __VLS_97({
    value: "aarch64-unknown-linux-gnu",
}, ...__VLS_functionalComponentArgsRest(__VLS_97));
const { default: __VLS_101 } = __VLS_99.slots;
// @ts-ignore
[];
var __VLS_99;
let __VLS_102;
/** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
aSelectOption;
// @ts-ignore
const __VLS_103 = __VLS_asFunctionalComponent1(__VLS_102, new __VLS_102({
    value: "x86_64-apple-darwin",
}));
const __VLS_104 = __VLS_103({
    value: "x86_64-apple-darwin",
}, ...__VLS_functionalComponentArgsRest(__VLS_103));
const { default: __VLS_107 } = __VLS_105.slots;
// @ts-ignore
[];
var __VLS_105;
let __VLS_108;
/** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
aSelectOption;
// @ts-ignore
const __VLS_109 = __VLS_asFunctionalComponent1(__VLS_108, new __VLS_108({
    value: "aarch64-apple-darwin",
}));
const __VLS_110 = __VLS_109({
    value: "aarch64-apple-darwin",
}, ...__VLS_functionalComponentArgsRest(__VLS_109));
const { default: __VLS_113 } = __VLS_111.slots;
// @ts-ignore
[];
var __VLS_111;
// @ts-ignore
[];
var __VLS_75;
// @ts-ignore
[];
var __VLS_69;
let __VLS_114;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_115 = __VLS_asFunctionalComponent1(__VLS_114, new __VLS_114({
    label: "绑定监听器",
    help: "推荐选择，用于确定通信协议和回连地址",
}));
const __VLS_116 = __VLS_115({
    label: "绑定监听器",
    help: "推荐选择，用于确定通信协议和回连地址",
}, ...__VLS_functionalComponentArgsRest(__VLS_115));
const { default: __VLS_119 } = __VLS_117.slots;
let __VLS_120;
/** @ts-ignore @type {typeof __VLS_components.aSelect | typeof __VLS_components.ASelect | typeof __VLS_components.aSelect | typeof __VLS_components.ASelect} */
aSelect;
// @ts-ignore
const __VLS_121 = __VLS_asFunctionalComponent1(__VLS_120, new __VLS_120({
    value: (__VLS_ctx.buildForm.listener_id),
    allowClear: true,
    placeholder: "选择监听器",
    loading: (__VLS_ctx.listenersLoading),
}));
const __VLS_122 = __VLS_121({
    value: (__VLS_ctx.buildForm.listener_id),
    allowClear: true,
    placeholder: "选择监听器",
    loading: (__VLS_ctx.listenersLoading),
}, ...__VLS_functionalComponentArgsRest(__VLS_121));
const { default: __VLS_125 } = __VLS_123.slots;
for (const [l] of __VLS_vFor((__VLS_ctx.listeners))) {
    let __VLS_126;
    /** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
    aSelectOption;
    // @ts-ignore
    const __VLS_127 = __VLS_asFunctionalComponent1(__VLS_126, new __VLS_126({
        key: (l.id),
        value: (l.id),
    }));
    const __VLS_128 = __VLS_127({
        key: (l.id),
        value: (l.id),
    }, ...__VLS_functionalComponentArgsRest(__VLS_127));
    const { default: __VLS_131 } = __VLS_129.slots;
    __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
        ...{ class: "font-mono mr-2" },
    });
    /** @type {__VLS_StyleScopedClasses['font-mono']} */ ;
    /** @type {__VLS_StyleScopedClasses['mr-2']} */ ;
    (l.id);
    (l.name);
    let __VLS_132;
    /** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
    aTag;
    // @ts-ignore
    const __VLS_133 = __VLS_asFunctionalComponent1(__VLS_132, new __VLS_132({
        color: (__VLS_ctx.getProtocolColor(l.protocol)),
        size: "small",
        ...{ class: "ml-2 mr-0" },
    }));
    const __VLS_134 = __VLS_133({
        color: (__VLS_ctx.getProtocolColor(l.protocol)),
        size: "small",
        ...{ class: "ml-2 mr-0" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_133));
    /** @type {__VLS_StyleScopedClasses['ml-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['mr-0']} */ ;
    const { default: __VLS_137 } = __VLS_135.slots;
    (l.protocol);
    // @ts-ignore
    [buildForm, listenersLoading, listeners, getProtocolColor,];
    var __VLS_135;
    // @ts-ignore
    [];
    var __VLS_129;
    // @ts-ignore
    [];
}
// @ts-ignore
[];
var __VLS_123;
// @ts-ignore
[];
var __VLS_117;
let __VLS_138;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_139 = __VLS_asFunctionalComponent1(__VLS_138, new __VLS_138({
    label: "回连地址",
    help: "留空则使用监听器绑定地址",
}));
const __VLS_140 = __VLS_139({
    label: "回连地址",
    help: "留空则使用监听器绑定地址",
}, ...__VLS_functionalComponentArgsRest(__VLS_139));
const { default: __VLS_143 } = __VLS_141.slots;
let __VLS_144;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_145 = __VLS_asFunctionalComponent1(__VLS_144, new __VLS_144({
    value: (__VLS_ctx.buildForm.server_addr),
    placeholder: "例: 192.168.1.100:4444",
    ...{ class: "font-mono" },
}));
const __VLS_146 = __VLS_145({
    value: (__VLS_ctx.buildForm.server_addr),
    placeholder: "例: 192.168.1.100:4444",
    ...{ class: "font-mono" },
}, ...__VLS_functionalComponentArgsRest(__VLS_145));
/** @type {__VLS_StyleScopedClasses['font-mono']} */ ;
// @ts-ignore
[buildForm,];
var __VLS_141;
let __VLS_149;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_150 = __VLS_asFunctionalComponent1(__VLS_149, new __VLS_149({
    label: "Agent Token",
    help: "留空则不嵌入默认认证令牌",
}));
const __VLS_151 = __VLS_150({
    label: "Agent Token",
    help: "留空则不嵌入默认认证令牌",
}, ...__VLS_functionalComponentArgsRest(__VLS_150));
const { default: __VLS_154 } = __VLS_152.slots;
let __VLS_155;
/** @ts-ignore @type {typeof __VLS_components.aInputPassword | typeof __VLS_components.AInputPassword} */
aInputPassword;
// @ts-ignore
const __VLS_156 = __VLS_asFunctionalComponent1(__VLS_155, new __VLS_155({
    value: (__VLS_ctx.buildForm.agent_token),
    placeholder: "可选，嵌入编译时的认证令牌",
    ...{ class: "font-mono" },
}));
const __VLS_157 = __VLS_156({
    value: (__VLS_ctx.buildForm.agent_token),
    placeholder: "可选，嵌入编译时的认证令牌",
    ...{ class: "font-mono" },
}, ...__VLS_functionalComponentArgsRest(__VLS_156));
/** @type {__VLS_StyleScopedClasses['font-mono']} */ ;
// @ts-ignore
[buildForm,];
var __VLS_152;
let __VLS_160;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_161 = __VLS_asFunctionalComponent1(__VLS_160, new __VLS_160({
    label: "构建配置",
}));
const __VLS_162 = __VLS_161({
    label: "构建配置",
}, ...__VLS_functionalComponentArgsRest(__VLS_161));
const { default: __VLS_165 } = __VLS_163.slots;
let __VLS_166;
/** @ts-ignore @type {typeof __VLS_components.aRadioGroup | typeof __VLS_components.ARadioGroup | typeof __VLS_components.aRadioGroup | typeof __VLS_components.ARadioGroup} */
aRadioGroup;
// @ts-ignore
const __VLS_167 = __VLS_asFunctionalComponent1(__VLS_166, new __VLS_166({
    value: (__VLS_ctx.buildForm.profile),
}));
const __VLS_168 = __VLS_167({
    value: (__VLS_ctx.buildForm.profile),
}, ...__VLS_functionalComponentArgsRest(__VLS_167));
const { default: __VLS_171 } = __VLS_169.slots;
let __VLS_172;
/** @ts-ignore @type {typeof __VLS_components.aRadioButton | typeof __VLS_components.ARadioButton | typeof __VLS_components.aRadioButton | typeof __VLS_components.ARadioButton} */
aRadioButton;
// @ts-ignore
const __VLS_173 = __VLS_asFunctionalComponent1(__VLS_172, new __VLS_172({
    value: "release",
}));
const __VLS_174 = __VLS_173({
    value: "release",
}, ...__VLS_functionalComponentArgsRest(__VLS_173));
const { default: __VLS_177 } = __VLS_175.slots;
// @ts-ignore
[buildForm,];
var __VLS_175;
let __VLS_178;
/** @ts-ignore @type {typeof __VLS_components.aRadioButton | typeof __VLS_components.ARadioButton | typeof __VLS_components.aRadioButton | typeof __VLS_components.ARadioButton} */
aRadioButton;
// @ts-ignore
const __VLS_179 = __VLS_asFunctionalComponent1(__VLS_178, new __VLS_178({
    value: "debug",
}));
const __VLS_180 = __VLS_179({
    value: "debug",
}, ...__VLS_functionalComponentArgsRest(__VLS_179));
const { default: __VLS_183 } = __VLS_181.slots;
// @ts-ignore
[];
var __VLS_181;
// @ts-ignore
[];
var __VLS_169;
// @ts-ignore
[];
var __VLS_163;
// @ts-ignore
[];
var __VLS_63;
// @ts-ignore
[];
var __VLS_55;
var __VLS_56;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
