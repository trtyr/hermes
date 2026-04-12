/// <reference types="../../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, reactive, watch } from 'vue';
import { message } from 'ant-design-vue';
import { DownOutlined } from '@ant-design/icons-vue';
import { fetchAgentDetail, updateBeaconConfig } from '@/api/agent';
import { formatTimestamp } from '@/utils/format';
const props = defineProps();
const emit = defineEmits(['update:visible', 'open-task', 'action', 'update:agent']);
const localAgent = ref(null);
const beaconUpdating = ref(false);
const beaconForm = reactive({
    sleep_interval: 10,
    jitter: 20
});
watch(() => props.visible, async (newVal) => {
    if (newVal && props.agent) {
        localAgent.value = { ...props.agent };
        beaconForm.sleep_interval = props.agent.sleep_interval;
        beaconForm.jitter = props.agent.jitter;
        try {
            const freshData = await fetchAgentDetail(props.agent.agent_id);
            localAgent.value = freshData;
            emit('update:agent', freshData);
            beaconForm.sleep_interval = freshData.sleep_interval;
            beaconForm.jitter = freshData.jitter;
        }
        catch (e) {
            if (e.message && !e.message.includes('canceled')) {
                message.warning('同步最新状态失败: ' + e.message);
            }
        }
    }
});
async function handleUpdateBeacon() {
    if (!localAgent.value)
        return;
    beaconUpdating.value = true;
    try {
        const res = await updateBeaconConfig(localAgent.value.agent_id, beaconForm.sleep_interval, beaconForm.jitter);
        if (res.success) {
            message.success('Beacon 配置更新成功');
            localAgent.value = res.agent;
            emit('update:agent', res.agent);
        }
    }
    catch (e) {
        message.error(e.message);
    }
    finally {
        beaconUpdating.value = false;
    }
}
function handleAction(action) {
    if (localAgent.value) {
        emit('action', { action, agent: localAgent.value });
    }
}
function openTaskModal() {
    if (localAgent.value) {
        emit('open-task', localAgent.value);
    }
}
function onClose() {
    emit('update:visible', false);
}
const __VLS_ctx = {
    ...{},
    ...{},
    ...{},
    ...{},
    ...{},
};
let __VLS_components;
let __VLS_intrinsics;
let __VLS_directives;
let __VLS_0;
/** @ts-ignore @type {typeof __VLS_components.aDrawer | typeof __VLS_components.ADrawer | typeof __VLS_components.aDrawer | typeof __VLS_components.ADrawer} */
aDrawer;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    ...{ 'onUpdate:open': {} },
    ...{ 'onClose': {} },
    open: (__VLS_ctx.visible),
    title: (`节点详情: ${__VLS_ctx.localAgent?.agent_id || ''}`),
    placement: "right",
    width: "600",
}));
const __VLS_2 = __VLS_1({
    ...{ 'onUpdate:open': {} },
    ...{ 'onClose': {} },
    open: (__VLS_ctx.visible),
    title: (`节点详情: ${__VLS_ctx.localAgent?.agent_id || ''}`),
    placement: "right",
    width: "600",
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
let __VLS_5;
const __VLS_6 = ({ 'update:open': {} },
    { 'onUpdate:open': (...[$event]) => {
            __VLS_ctx.$emit('update:visible', $event);
            // @ts-ignore
            [visible, localAgent, $emit,];
        } });
const __VLS_7 = ({ close: {} },
    { onClose: (__VLS_ctx.onClose) });
var __VLS_8 = {};
const { default: __VLS_9 } = __VLS_3.slots;
if (__VLS_ctx.localAgent) {
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "space-y-6" },
    });
    /** @type {__VLS_StyleScopedClasses['space-y-6']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "p-4 rounded-lg flex items-center justify-between border" },
        ...{ class: (__VLS_ctx.localAgent.is_online ? 'bg-green-50/50 border-green-200 dark:bg-green-900/10 dark:border-green-900/30' : 'bg-slate-50 border-slate-200 dark:bg-[#14161A] dark:border-slate-800') },
    });
    /** @type {__VLS_StyleScopedClasses['p-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['rounded-lg']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['justify-between']} */ ;
    /** @type {__VLS_StyleScopedClasses['border']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex items-center gap-3" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-3']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "w-3 h-3 rounded-full" },
        ...{ class: (__VLS_ctx.localAgent.is_online ? 'bg-green-500' : 'bg-slate-400') },
    });
    /** @type {__VLS_StyleScopedClasses['w-3']} */ ;
    /** @type {__VLS_StyleScopedClasses['h-3']} */ ;
    /** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
        ...{ class: "font-medium" },
        ...{ class: (__VLS_ctx.localAgent.is_online ? 'text-green-700 dark:text-green-400' : 'text-slate-600 dark:text-slate-400') },
    });
    /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
    (__VLS_ctx.localAgent.is_online ? '在线 (Online)' : '离线 (Offline)');
    if (__VLS_ctx.localAgent.is_disabled) {
        let __VLS_10;
        /** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
        aTag;
        // @ts-ignore
        const __VLS_11 = __VLS_asFunctionalComponent1(__VLS_10, new __VLS_10({
            color: "error",
        }));
        const __VLS_12 = __VLS_11({
            color: "error",
        }, ...__VLS_functionalComponentArgsRest(__VLS_11));
        const { default: __VLS_15 } = __VLS_13.slots;
        // @ts-ignore
        [localAgent, localAgent, localAgent, localAgent, localAgent, localAgent, onClose,];
        var __VLS_13;
    }
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "text-xs text-slate-500" },
    });
    /** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
    (__VLS_ctx.formatTimestamp(__VLS_ctx.localAgent.last_seen));
    let __VLS_16;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptions | typeof __VLS_components.ADescriptions | typeof __VLS_components.aDescriptions | typeof __VLS_components.ADescriptions} */
    aDescriptions;
    // @ts-ignore
    const __VLS_17 = __VLS_asFunctionalComponent1(__VLS_16, new __VLS_16({
        title: "基础信息",
        column: (2),
        bordered: true,
        size: "small",
        ...{ class: "bg-white dark:bg-[#1C1E22]" },
    }));
    const __VLS_18 = __VLS_17({
        title: "基础信息",
        column: (2),
        bordered: true,
        size: "small",
        ...{ class: "bg-white dark:bg-[#1C1E22]" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_17));
    /** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[#1C1E22]']} */ ;
    const { default: __VLS_21 } = __VLS_19.slots;
    let __VLS_22;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_23 = __VLS_asFunctionalComponent1(__VLS_22, new __VLS_22({
        label: "主机名",
    }));
    const __VLS_24 = __VLS_23({
        label: "主机名",
    }, ...__VLS_functionalComponentArgsRest(__VLS_23));
    const { default: __VLS_27 } = __VLS_25.slots;
    (__VLS_ctx.localAgent.hostname);
    // @ts-ignore
    [localAgent, localAgent, formatTimestamp,];
    var __VLS_25;
    let __VLS_28;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_29 = __VLS_asFunctionalComponent1(__VLS_28, new __VLS_28({
        label: "用户名",
    }));
    const __VLS_30 = __VLS_29({
        label: "用户名",
    }, ...__VLS_functionalComponentArgsRest(__VLS_29));
    const { default: __VLS_33 } = __VLS_31.slots;
    (__VLS_ctx.localAgent.username);
    // @ts-ignore
    [localAgent,];
    var __VLS_31;
    let __VLS_34;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_35 = __VLS_asFunctionalComponent1(__VLS_34, new __VLS_34({
        label: "操作系统",
    }));
    const __VLS_36 = __VLS_35({
        label: "操作系统",
    }, ...__VLS_functionalComponentArgsRest(__VLS_35));
    const { default: __VLS_39 } = __VLS_37.slots;
    (__VLS_ctx.localAgent.os);
    // @ts-ignore
    [localAgent,];
    var __VLS_37;
    let __VLS_40;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_41 = __VLS_asFunctionalComponent1(__VLS_40, new __VLS_40({
        label: "架构",
    }));
    const __VLS_42 = __VLS_41({
        label: "架构",
    }, ...__VLS_functionalComponentArgsRest(__VLS_41));
    const { default: __VLS_45 } = __VLS_43.slots;
    (__VLS_ctx.localAgent.arch);
    // @ts-ignore
    [localAgent,];
    var __VLS_43;
    let __VLS_46;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_47 = __VLS_asFunctionalComponent1(__VLS_46, new __VLS_46({
        label: "进程 ID",
    }));
    const __VLS_48 = __VLS_47({
        label: "进程 ID",
    }, ...__VLS_functionalComponentArgsRest(__VLS_47));
    const { default: __VLS_51 } = __VLS_49.slots;
    (__VLS_ctx.localAgent.pid);
    // @ts-ignore
    [localAgent,];
    var __VLS_49;
    let __VLS_52;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_53 = __VLS_asFunctionalComponent1(__VLS_52, new __VLS_52({
        label: "会话 ID",
    }));
    const __VLS_54 = __VLS_53({
        label: "会话 ID",
    }, ...__VLS_functionalComponentArgsRest(__VLS_53));
    const { default: __VLS_57 } = __VLS_55.slots;
    (__VLS_ctx.localAgent.session_id !== null ? __VLS_ctx.localAgent.session_id : '-');
    // @ts-ignore
    [localAgent, localAgent,];
    var __VLS_55;
    let __VLS_58;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_59 = __VLS_asFunctionalComponent1(__VLS_58, new __VLS_58({
        label: "标签",
        span: (2),
    }));
    const __VLS_60 = __VLS_59({
        label: "标签",
        span: (2),
    }, ...__VLS_functionalComponentArgsRest(__VLS_59));
    const { default: __VLS_63 } = __VLS_61.slots;
    if (__VLS_ctx.localAgent.tags && __VLS_ctx.localAgent.tags.length > 0) {
        for (const [tag] of __VLS_vFor((__VLS_ctx.localAgent.tags))) {
            let __VLS_64;
            /** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
            aTag;
            // @ts-ignore
            const __VLS_65 = __VLS_asFunctionalComponent1(__VLS_64, new __VLS_64({
                key: (tag),
                color: "blue",
            }));
            const __VLS_66 = __VLS_65({
                key: (tag),
                color: "blue",
            }, ...__VLS_functionalComponentArgsRest(__VLS_65));
            const { default: __VLS_69 } = __VLS_67.slots;
            (tag);
            // @ts-ignore
            [localAgent, localAgent, localAgent,];
            var __VLS_67;
            // @ts-ignore
            [];
        }
    }
    else {
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "text-slate-400" },
        });
        /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
    }
    // @ts-ignore
    [];
    var __VLS_61;
    // @ts-ignore
    [];
    var __VLS_19;
    let __VLS_70;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptions | typeof __VLS_components.ADescriptions | typeof __VLS_components.aDescriptions | typeof __VLS_components.ADescriptions} */
    aDescriptions;
    // @ts-ignore
    const __VLS_71 = __VLS_asFunctionalComponent1(__VLS_70, new __VLS_70({
        title: "网络配置",
        column: (2),
        bordered: true,
        size: "small",
        ...{ class: "bg-white dark:bg-[#1C1E22]" },
    }));
    const __VLS_72 = __VLS_71({
        title: "网络配置",
        column: (2),
        bordered: true,
        size: "small",
        ...{ class: "bg-white dark:bg-[#1C1E22]" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_71));
    /** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[#1C1E22]']} */ ;
    const { default: __VLS_75 } = __VLS_73.slots;
    let __VLS_76;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_77 = __VLS_asFunctionalComponent1(__VLS_76, new __VLS_76({
        label: "内部 IP",
    }));
    const __VLS_78 = __VLS_77({
        label: "内部 IP",
    }, ...__VLS_functionalComponentArgsRest(__VLS_77));
    const { default: __VLS_81 } = __VLS_79.slots;
    (__VLS_ctx.localAgent.internal_ip);
    // @ts-ignore
    [localAgent,];
    var __VLS_79;
    let __VLS_82;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_83 = __VLS_asFunctionalComponent1(__VLS_82, new __VLS_82({
        label: "外部 IP",
    }));
    const __VLS_84 = __VLS_83({
        label: "外部 IP",
    }, ...__VLS_functionalComponentArgsRest(__VLS_83));
    const { default: __VLS_87 } = __VLS_85.slots;
    (__VLS_ctx.localAgent.external_ip);
    // @ts-ignore
    [localAgent,];
    var __VLS_85;
    let __VLS_88;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_89 = __VLS_asFunctionalComponent1(__VLS_88, new __VLS_88({
        label: "对端地址",
        span: (2),
    }));
    const __VLS_90 = __VLS_89({
        label: "对端地址",
        span: (2),
    }, ...__VLS_functionalComponentArgsRest(__VLS_89));
    const { default: __VLS_93 } = __VLS_91.slots;
    (__VLS_ctx.localAgent.peer_addr);
    // @ts-ignore
    [localAgent,];
    var __VLS_91;
    // @ts-ignore
    [];
    var __VLS_73;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "border border-gray-200 dark:border-[#14161A] rounded-lg overflow-hidden" },
    });
    /** @type {__VLS_StyleScopedClasses['border']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:border-[#14161A]']} */ ;
    /** @type {__VLS_StyleScopedClasses['rounded-lg']} */ ;
    /** @type {__VLS_StyleScopedClasses['overflow-hidden']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "bg-slate-50 dark:bg-[#14161A] px-4 py-2 border-b border-gray-200 dark:border-[#14161A] font-medium text-slate-800 dark:text-slate-200" },
    });
    /** @type {__VLS_StyleScopedClasses['bg-slate-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[#14161A]']} */ ;
    /** @type {__VLS_StyleScopedClasses['px-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-b']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:border-[#14161A]']} */ ;
    /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-800']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:text-slate-200']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "p-4 bg-white dark:bg-[#1C1E22]" },
    });
    /** @type {__VLS_StyleScopedClasses['p-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[#1C1E22]']} */ ;
    let __VLS_94;
    /** @ts-ignore @type {typeof __VLS_components.aForm | typeof __VLS_components.AForm | typeof __VLS_components.aForm | typeof __VLS_components.AForm} */
    aForm;
    // @ts-ignore
    const __VLS_95 = __VLS_asFunctionalComponent1(__VLS_94, new __VLS_94({
        layout: "vertical",
        ...{ class: "flex gap-4" },
    }));
    const __VLS_96 = __VLS_95({
        layout: "vertical",
        ...{ class: "flex gap-4" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_95));
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-4']} */ ;
    const { default: __VLS_99 } = __VLS_97.slots;
    let __VLS_100;
    /** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
    aFormItem;
    // @ts-ignore
    const __VLS_101 = __VLS_asFunctionalComponent1(__VLS_100, new __VLS_100({
        label: "休眠间隔 (秒)",
        ...{ class: "flex-1 mb-0" },
    }));
    const __VLS_102 = __VLS_101({
        label: "休眠间隔 (秒)",
        ...{ class: "flex-1 mb-0" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_101));
    /** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-0']} */ ;
    const { default: __VLS_105 } = __VLS_103.slots;
    let __VLS_106;
    /** @ts-ignore @type {typeof __VLS_components.aInputNumber | typeof __VLS_components.AInputNumber} */
    aInputNumber;
    // @ts-ignore
    const __VLS_107 = __VLS_asFunctionalComponent1(__VLS_106, new __VLS_106({
        value: (__VLS_ctx.beaconForm.sleep_interval),
        min: (1),
        ...{ class: "w-full" },
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }));
    const __VLS_108 = __VLS_107({
        value: (__VLS_ctx.beaconForm.sleep_interval),
        min: (1),
        ...{ class: "w-full" },
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }, ...__VLS_functionalComponentArgsRest(__VLS_107));
    /** @type {__VLS_StyleScopedClasses['w-full']} */ ;
    // @ts-ignore
    [localAgent, localAgent, beaconForm,];
    var __VLS_103;
    let __VLS_111;
    /** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
    aFormItem;
    // @ts-ignore
    const __VLS_112 = __VLS_asFunctionalComponent1(__VLS_111, new __VLS_111({
        label: "抖动 (Jitter %)",
        ...{ class: "flex-1 mb-0" },
    }));
    const __VLS_113 = __VLS_112({
        label: "抖动 (Jitter %)",
        ...{ class: "flex-1 mb-0" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_112));
    /** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-0']} */ ;
    const { default: __VLS_116 } = __VLS_114.slots;
    let __VLS_117;
    /** @ts-ignore @type {typeof __VLS_components.aInputNumber | typeof __VLS_components.AInputNumber} */
    aInputNumber;
    // @ts-ignore
    const __VLS_118 = __VLS_asFunctionalComponent1(__VLS_117, new __VLS_117({
        value: (__VLS_ctx.beaconForm.jitter),
        min: (0),
        max: (100),
        ...{ class: "w-full" },
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }));
    const __VLS_119 = __VLS_118({
        value: (__VLS_ctx.beaconForm.jitter),
        min: (0),
        max: (100),
        ...{ class: "w-full" },
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }, ...__VLS_functionalComponentArgsRest(__VLS_118));
    /** @type {__VLS_StyleScopedClasses['w-full']} */ ;
    // @ts-ignore
    [localAgent, localAgent, beaconForm,];
    var __VLS_114;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex items-end mb-0" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-end']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-0']} */ ;
    let __VLS_122;
    /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
    aButton;
    // @ts-ignore
    const __VLS_123 = __VLS_asFunctionalComponent1(__VLS_122, new __VLS_122({
        ...{ 'onClick': {} },
        type: "primary",
        loading: (__VLS_ctx.beaconUpdating),
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }));
    const __VLS_124 = __VLS_123({
        ...{ 'onClick': {} },
        type: "primary",
        loading: (__VLS_ctx.beaconUpdating),
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }, ...__VLS_functionalComponentArgsRest(__VLS_123));
    let __VLS_127;
    const __VLS_128 = ({ click: {} },
        { onClick: (__VLS_ctx.handleUpdateBeacon) });
    const { default: __VLS_129 } = __VLS_125.slots;
    // @ts-ignore
    [localAgent, localAgent, beaconUpdating, handleUpdateBeacon,];
    var __VLS_125;
    var __VLS_126;
    // @ts-ignore
    [];
    var __VLS_97;
    if (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "mt-2 text-xs text-orange-500" },
        });
        /** @type {__VLS_StyleScopedClasses['mt-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-orange-500']} */ ;
    }
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex gap-2 justify-end pt-4 border-t border-gray-200 dark:border-[#14161A]" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['justify-end']} */ ;
    /** @type {__VLS_StyleScopedClasses['pt-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-t']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:border-[#14161A]']} */ ;
    let __VLS_130;
    /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
    aButton;
    // @ts-ignore
    const __VLS_131 = __VLS_asFunctionalComponent1(__VLS_130, new __VLS_130({
        ...{ 'onClick': {} },
        type: "primary",
        disabled: (__VLS_ctx.localAgent.is_disabled),
    }));
    const __VLS_132 = __VLS_131({
        ...{ 'onClick': {} },
        type: "primary",
        disabled: (__VLS_ctx.localAgent.is_disabled),
    }, ...__VLS_functionalComponentArgsRest(__VLS_131));
    let __VLS_135;
    const __VLS_136 = ({ click: {} },
        { onClick: (__VLS_ctx.openTaskModal) });
    const { default: __VLS_137 } = __VLS_133.slots;
    // @ts-ignore
    [localAgent, localAgent, localAgent, openTaskModal,];
    var __VLS_133;
    var __VLS_134;
    let __VLS_138;
    /** @ts-ignore @type {typeof __VLS_components.aDropdown | typeof __VLS_components.ADropdown | typeof __VLS_components.aDropdown | typeof __VLS_components.ADropdown} */
    aDropdown;
    // @ts-ignore
    const __VLS_139 = __VLS_asFunctionalComponent1(__VLS_138, new __VLS_138({
        placement: "topRight",
    }));
    const __VLS_140 = __VLS_139({
        placement: "topRight",
    }, ...__VLS_functionalComponentArgsRest(__VLS_139));
    const { default: __VLS_143 } = __VLS_141.slots;
    let __VLS_144;
    /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
    aButton;
    // @ts-ignore
    const __VLS_145 = __VLS_asFunctionalComponent1(__VLS_144, new __VLS_144({}));
    const __VLS_146 = __VLS_145({}, ...__VLS_functionalComponentArgsRest(__VLS_145));
    const { default: __VLS_149 } = __VLS_147.slots;
    let __VLS_150;
    /** @ts-ignore @type {typeof __VLS_components.DownOutlined} */
    DownOutlined;
    // @ts-ignore
    const __VLS_151 = __VLS_asFunctionalComponent1(__VLS_150, new __VLS_150({}));
    const __VLS_152 = __VLS_151({}, ...__VLS_functionalComponentArgsRest(__VLS_151));
    // @ts-ignore
    [];
    var __VLS_147;
    {
        const { overlay: __VLS_155 } = __VLS_141.slots;
        let __VLS_156;
        /** @ts-ignore @type {typeof __VLS_components.aMenu | typeof __VLS_components.AMenu | typeof __VLS_components.aMenu | typeof __VLS_components.AMenu} */
        aMenu;
        // @ts-ignore
        const __VLS_157 = __VLS_asFunctionalComponent1(__VLS_156, new __VLS_156({}));
        const __VLS_158 = __VLS_157({}, ...__VLS_functionalComponentArgsRest(__VLS_157));
        const { default: __VLS_161 } = __VLS_159.slots;
        let __VLS_162;
        /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
        aMenuItem;
        // @ts-ignore
        const __VLS_163 = __VLS_asFunctionalComponent1(__VLS_162, new __VLS_162({
            ...{ 'onClick': {} },
            key: "disconnect",
            disabled: (!__VLS_ctx.localAgent.is_online),
        }));
        const __VLS_164 = __VLS_163({
            ...{ 'onClick': {} },
            key: "disconnect",
            disabled: (!__VLS_ctx.localAgent.is_online),
        }, ...__VLS_functionalComponentArgsRest(__VLS_163));
        let __VLS_167;
        const __VLS_168 = ({ click: {} },
            { onClick: (...[$event]) => {
                    if (!(__VLS_ctx.localAgent))
                        return;
                    __VLS_ctx.handleAction('disconnect');
                    // @ts-ignore
                    [localAgent, handleAction,];
                } });
        const { default: __VLS_169 } = __VLS_165.slots;
        // @ts-ignore
        [];
        var __VLS_165;
        var __VLS_166;
        if (!__VLS_ctx.localAgent.is_disabled) {
            let __VLS_170;
            /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
            aMenuItem;
            // @ts-ignore
            const __VLS_171 = __VLS_asFunctionalComponent1(__VLS_170, new __VLS_170({
                ...{ 'onClick': {} },
                key: "disable",
            }));
            const __VLS_172 = __VLS_171({
                ...{ 'onClick': {} },
                key: "disable",
            }, ...__VLS_functionalComponentArgsRest(__VLS_171));
            let __VLS_175;
            const __VLS_176 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!(__VLS_ctx.localAgent))
                            return;
                        if (!(!__VLS_ctx.localAgent.is_disabled))
                            return;
                        __VLS_ctx.handleAction('disable');
                        // @ts-ignore
                        [localAgent, handleAction,];
                    } });
            const { default: __VLS_177 } = __VLS_173.slots;
            // @ts-ignore
            [];
            var __VLS_173;
            var __VLS_174;
        }
        if (__VLS_ctx.localAgent.is_disabled) {
            let __VLS_178;
            /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
            aMenuItem;
            // @ts-ignore
            const __VLS_179 = __VLS_asFunctionalComponent1(__VLS_178, new __VLS_178({
                ...{ 'onClick': {} },
                key: "enable",
            }));
            const __VLS_180 = __VLS_179({
                ...{ 'onClick': {} },
                key: "enable",
            }, ...__VLS_functionalComponentArgsRest(__VLS_179));
            let __VLS_183;
            const __VLS_184 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!(__VLS_ctx.localAgent))
                            return;
                        if (!(__VLS_ctx.localAgent.is_disabled))
                            return;
                        __VLS_ctx.handleAction('enable');
                        // @ts-ignore
                        [localAgent, handleAction,];
                    } });
            const { default: __VLS_185 } = __VLS_181.slots;
            // @ts-ignore
            [];
            var __VLS_181;
            var __VLS_182;
        }
        let __VLS_186;
        /** @ts-ignore @type {typeof __VLS_components.aMenuDivider | typeof __VLS_components.AMenuDivider} */
        aMenuDivider;
        // @ts-ignore
        const __VLS_187 = __VLS_asFunctionalComponent1(__VLS_186, new __VLS_186({}));
        const __VLS_188 = __VLS_187({}, ...__VLS_functionalComponentArgsRest(__VLS_187));
        let __VLS_191;
        /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
        aMenuItem;
        // @ts-ignore
        const __VLS_192 = __VLS_asFunctionalComponent1(__VLS_191, new __VLS_191({
            ...{ 'onClick': {} },
            key: "delete",
            disabled: (__VLS_ctx.localAgent.is_online),
            danger: true,
        }));
        const __VLS_193 = __VLS_192({
            ...{ 'onClick': {} },
            key: "delete",
            disabled: (__VLS_ctx.localAgent.is_online),
            danger: true,
        }, ...__VLS_functionalComponentArgsRest(__VLS_192));
        let __VLS_196;
        const __VLS_197 = ({ click: {} },
            { onClick: (...[$event]) => {
                    if (!(__VLS_ctx.localAgent))
                        return;
                    __VLS_ctx.handleAction('delete');
                    // @ts-ignore
                    [localAgent, handleAction,];
                } });
        const { default: __VLS_198 } = __VLS_194.slots;
        // @ts-ignore
        [];
        var __VLS_194;
        var __VLS_195;
        // @ts-ignore
        [];
        var __VLS_159;
        // @ts-ignore
        [];
    }
    // @ts-ignore
    [];
    var __VLS_141;
}
else {
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex justify-center py-10" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-10']} */ ;
    let __VLS_199;
    /** @ts-ignore @type {typeof __VLS_components.aSpin | typeof __VLS_components.ASpin} */
    aSpin;
    // @ts-ignore
    const __VLS_200 = __VLS_asFunctionalComponent1(__VLS_199, new __VLS_199({}));
    const __VLS_201 = __VLS_200({}, ...__VLS_functionalComponentArgsRest(__VLS_200));
}
// @ts-ignore
[];
var __VLS_3;
var __VLS_4;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({
    emits: {},
    __typeProps: {},
});
export default {};
