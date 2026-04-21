/// <reference types="../../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, reactive, watch, nextTick } from 'vue';
import { message } from 'ant-design-vue';
import { DownOutlined, PlusOutlined } from '@ant-design/icons-vue';
import { fetchAgentDetail, updateBeaconConfig, updateAgentTags } from '@/api/agent';
import { formatTimestamp } from '@/utils/format';
const props = defineProps();
const emit = defineEmits(['update:visible', 'open-task', 'action', 'update:agent']);
const localAgent = ref(null);
const beaconUpdating = ref(false);
const beaconForm = reactive({
    sleep_interval: 10,
    jitter: 20
});
const tagInputVisible = ref(false);
const tagInputValue = ref('');
const tagInputRef = ref(null);
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
function showTagInput() {
    tagInputVisible.value = true;
    tagInputValue.value = '';
    nextTick(() => {
        tagInputRef.value?.focus();
    });
}
async function handleTagInputConfirm() {
    tagInputVisible.value = false;
    const newTag = tagInputValue.value.trim();
    if (!newTag || !localAgent.value)
        return;
    const tags = [...(localAgent.value.tags || [])];
    if (tags.includes(newTag))
        return;
    tags.push(newTag);
    await saveTags(tags);
}
async function removeTag(tag) {
    if (!localAgent.value)
        return;
    const tags = (localAgent.value.tags || []).filter(t => t !== tag);
    await saveTags(tags);
}
async function saveTags(tags) {
    if (!localAgent.value)
        return;
    try {
        const res = await updateAgentTags(localAgent.value.agent_id, tags);
        if (res.success) {
            localAgent.value = { ...localAgent.value, tags };
            emit('update:agent', localAgent.value);
            message.success('标签已更新');
        }
    }
    catch (e) {
        message.error(e.message);
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
        ...{ class: (__VLS_ctx.localAgent.is_online ? 'bg-green-50/50 border-green-200 dark:bg-green-900/10 dark:border-green-900/30' : 'bg-slate-50 border-slate-200 dark:bg-[var(--bg-sub)] dark:border-slate-800') },
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
        ...{ class: (__VLS_ctx.localAgent.is_online ? 'text-green-700 dark:text-green-400' : 'text-slate-600 dark:text-[var(--text-secondary)]') },
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
        ...{ class: "bg-white dark:bg-[var(--bg-card)]" },
    }));
    const __VLS_18 = __VLS_17({
        title: "基础信息",
        column: (2),
        bordered: true,
        size: "small",
        ...{ class: "bg-white dark:bg-[var(--bg-card)]" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_17));
    /** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-card)]']} */ ;
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
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex items-center gap-1 flex-wrap" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-1']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex-wrap']} */ ;
    for (const [tag] of __VLS_vFor((__VLS_ctx.localAgent.tags))) {
        let __VLS_64;
        /** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
        aTag;
        // @ts-ignore
        const __VLS_65 = __VLS_asFunctionalComponent1(__VLS_64, new __VLS_64({
            ...{ 'onClose': {} },
            key: (tag),
            color: "blue",
            closable: true,
        }));
        const __VLS_66 = __VLS_65({
            ...{ 'onClose': {} },
            key: (tag),
            color: "blue",
            closable: true,
        }, ...__VLS_functionalComponentArgsRest(__VLS_65));
        let __VLS_69;
        const __VLS_70 = ({ close: {} },
            { onClose: (...[$event]) => {
                    if (!(__VLS_ctx.localAgent))
                        return;
                    __VLS_ctx.removeTag(tag);
                    // @ts-ignore
                    [localAgent, removeTag,];
                } });
        const { default: __VLS_71 } = __VLS_67.slots;
        (tag);
        // @ts-ignore
        [];
        var __VLS_67;
        var __VLS_68;
        // @ts-ignore
        [];
    }
    if (__VLS_ctx.tagInputVisible) {
        let __VLS_72;
        /** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
        aInput;
        // @ts-ignore
        const __VLS_73 = __VLS_asFunctionalComponent1(__VLS_72, new __VLS_72({
            ...{ 'onBlur': {} },
            ...{ 'onKeyup': {} },
            ref: "tagInputRef",
            value: (__VLS_ctx.tagInputValue),
            size: "small",
            ...{ style: {} },
            placeholder: "输入标签",
        }));
        const __VLS_74 = __VLS_73({
            ...{ 'onBlur': {} },
            ...{ 'onKeyup': {} },
            ref: "tagInputRef",
            value: (__VLS_ctx.tagInputValue),
            size: "small",
            ...{ style: {} },
            placeholder: "输入标签",
        }, ...__VLS_functionalComponentArgsRest(__VLS_73));
        let __VLS_77;
        const __VLS_78 = ({ blur: {} },
            { onBlur: (__VLS_ctx.handleTagInputConfirm) });
        const __VLS_79 = ({ keyup: {} },
            { onKeyup: (__VLS_ctx.handleTagInputConfirm) });
        var __VLS_80 = {};
        var __VLS_75;
        var __VLS_76;
    }
    else {
        let __VLS_82;
        /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
        aButton;
        // @ts-ignore
        const __VLS_83 = __VLS_asFunctionalComponent1(__VLS_82, new __VLS_82({
            ...{ 'onClick': {} },
            size: "small",
            type: "dashed",
        }));
        const __VLS_84 = __VLS_83({
            ...{ 'onClick': {} },
            size: "small",
            type: "dashed",
        }, ...__VLS_functionalComponentArgsRest(__VLS_83));
        let __VLS_87;
        const __VLS_88 = ({ click: {} },
            { onClick: (__VLS_ctx.showTagInput) });
        const { default: __VLS_89 } = __VLS_85.slots;
        {
            const { icon: __VLS_90 } = __VLS_85.slots;
            let __VLS_91;
            /** @ts-ignore @type {typeof __VLS_components.PlusOutlined} */
            PlusOutlined;
            // @ts-ignore
            const __VLS_92 = __VLS_asFunctionalComponent1(__VLS_91, new __VLS_91({}));
            const __VLS_93 = __VLS_92({}, ...__VLS_functionalComponentArgsRest(__VLS_92));
            // @ts-ignore
            [tagInputVisible, tagInputValue, handleTagInputConfirm, handleTagInputConfirm, showTagInput,];
        }
        // @ts-ignore
        [];
        var __VLS_85;
        var __VLS_86;
    }
    // @ts-ignore
    [];
    var __VLS_61;
    // @ts-ignore
    [];
    var __VLS_19;
    let __VLS_96;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptions | typeof __VLS_components.ADescriptions | typeof __VLS_components.aDescriptions | typeof __VLS_components.ADescriptions} */
    aDescriptions;
    // @ts-ignore
    const __VLS_97 = __VLS_asFunctionalComponent1(__VLS_96, new __VLS_96({
        title: "网络配置",
        column: (2),
        bordered: true,
        size: "small",
        ...{ class: "bg-white dark:bg-[var(--bg-card)]" },
    }));
    const __VLS_98 = __VLS_97({
        title: "网络配置",
        column: (2),
        bordered: true,
        size: "small",
        ...{ class: "bg-white dark:bg-[var(--bg-card)]" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_97));
    /** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-card)]']} */ ;
    const { default: __VLS_101 } = __VLS_99.slots;
    let __VLS_102;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_103 = __VLS_asFunctionalComponent1(__VLS_102, new __VLS_102({
        label: "内部 IP",
    }));
    const __VLS_104 = __VLS_103({
        label: "内部 IP",
    }, ...__VLS_functionalComponentArgsRest(__VLS_103));
    const { default: __VLS_107 } = __VLS_105.slots;
    (__VLS_ctx.localAgent.internal_ip);
    // @ts-ignore
    [localAgent,];
    var __VLS_105;
    let __VLS_108;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_109 = __VLS_asFunctionalComponent1(__VLS_108, new __VLS_108({
        label: "外部 IP",
    }));
    const __VLS_110 = __VLS_109({
        label: "外部 IP",
    }, ...__VLS_functionalComponentArgsRest(__VLS_109));
    const { default: __VLS_113 } = __VLS_111.slots;
    (__VLS_ctx.localAgent.external_ip);
    // @ts-ignore
    [localAgent,];
    var __VLS_111;
    let __VLS_114;
    /** @ts-ignore @type {typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem | typeof __VLS_components.aDescriptionsItem | typeof __VLS_components.ADescriptionsItem} */
    aDescriptionsItem;
    // @ts-ignore
    const __VLS_115 = __VLS_asFunctionalComponent1(__VLS_114, new __VLS_114({
        label: "对端地址",
        span: (2),
    }));
    const __VLS_116 = __VLS_115({
        label: "对端地址",
        span: (2),
    }, ...__VLS_functionalComponentArgsRest(__VLS_115));
    const { default: __VLS_119 } = __VLS_117.slots;
    (__VLS_ctx.localAgent.peer_addr);
    // @ts-ignore
    [localAgent,];
    var __VLS_117;
    // @ts-ignore
    [];
    var __VLS_99;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "border border-gray-200 dark:border-[var(--border-default)] rounded-lg overflow-hidden" },
    });
    /** @type {__VLS_StyleScopedClasses['border']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:border-[var(--border-default)]']} */ ;
    /** @type {__VLS_StyleScopedClasses['rounded-lg']} */ ;
    /** @type {__VLS_StyleScopedClasses['overflow-hidden']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "bg-slate-50 dark:bg-[var(--bg-sub)] px-4 py-2 border-b border-gray-200 dark:border-[var(--border-default)] font-medium text-slate-800 dark:text-[var(--text-primary)]" },
    });
    /** @type {__VLS_StyleScopedClasses['bg-slate-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-sub)]']} */ ;
    /** @type {__VLS_StyleScopedClasses['px-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-b']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:border-[var(--border-default)]']} */ ;
    /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-800']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:text-[var(--text-primary)]']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "p-4 bg-white dark:bg-[var(--bg-card)]" },
    });
    /** @type {__VLS_StyleScopedClasses['p-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-card)]']} */ ;
    let __VLS_120;
    /** @ts-ignore @type {typeof __VLS_components.aForm | typeof __VLS_components.AForm | typeof __VLS_components.aForm | typeof __VLS_components.AForm} */
    aForm;
    // @ts-ignore
    const __VLS_121 = __VLS_asFunctionalComponent1(__VLS_120, new __VLS_120({
        layout: "vertical",
        ...{ class: "flex gap-4" },
    }));
    const __VLS_122 = __VLS_121({
        layout: "vertical",
        ...{ class: "flex gap-4" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_121));
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-4']} */ ;
    const { default: __VLS_125 } = __VLS_123.slots;
    let __VLS_126;
    /** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
    aFormItem;
    // @ts-ignore
    const __VLS_127 = __VLS_asFunctionalComponent1(__VLS_126, new __VLS_126({
        label: "休眠间隔 (秒)",
        ...{ class: "flex-1 mb-0" },
    }));
    const __VLS_128 = __VLS_127({
        label: "休眠间隔 (秒)",
        ...{ class: "flex-1 mb-0" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_127));
    /** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-0']} */ ;
    const { default: __VLS_131 } = __VLS_129.slots;
    let __VLS_132;
    /** @ts-ignore @type {typeof __VLS_components.aInputNumber | typeof __VLS_components.AInputNumber} */
    aInputNumber;
    // @ts-ignore
    const __VLS_133 = __VLS_asFunctionalComponent1(__VLS_132, new __VLS_132({
        value: (__VLS_ctx.beaconForm.sleep_interval),
        min: (1),
        ...{ class: "w-full" },
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }));
    const __VLS_134 = __VLS_133({
        value: (__VLS_ctx.beaconForm.sleep_interval),
        min: (1),
        ...{ class: "w-full" },
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }, ...__VLS_functionalComponentArgsRest(__VLS_133));
    /** @type {__VLS_StyleScopedClasses['w-full']} */ ;
    // @ts-ignore
    [localAgent, localAgent, beaconForm,];
    var __VLS_129;
    let __VLS_137;
    /** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
    aFormItem;
    // @ts-ignore
    const __VLS_138 = __VLS_asFunctionalComponent1(__VLS_137, new __VLS_137({
        label: "抖动 (Jitter %)",
        ...{ class: "flex-1 mb-0" },
    }));
    const __VLS_139 = __VLS_138({
        label: "抖动 (Jitter %)",
        ...{ class: "flex-1 mb-0" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_138));
    /** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-0']} */ ;
    const { default: __VLS_142 } = __VLS_140.slots;
    let __VLS_143;
    /** @ts-ignore @type {typeof __VLS_components.aInputNumber | typeof __VLS_components.AInputNumber} */
    aInputNumber;
    // @ts-ignore
    const __VLS_144 = __VLS_asFunctionalComponent1(__VLS_143, new __VLS_143({
        value: (__VLS_ctx.beaconForm.jitter),
        min: (0),
        max: (100),
        ...{ class: "w-full" },
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }));
    const __VLS_145 = __VLS_144({
        value: (__VLS_ctx.beaconForm.jitter),
        min: (0),
        max: (100),
        ...{ class: "w-full" },
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }, ...__VLS_functionalComponentArgsRest(__VLS_144));
    /** @type {__VLS_StyleScopedClasses['w-full']} */ ;
    // @ts-ignore
    [localAgent, localAgent, beaconForm,];
    var __VLS_140;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex items-end mb-0" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-end']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-0']} */ ;
    let __VLS_148;
    /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
    aButton;
    // @ts-ignore
    const __VLS_149 = __VLS_asFunctionalComponent1(__VLS_148, new __VLS_148({
        ...{ 'onClick': {} },
        type: "primary",
        loading: (__VLS_ctx.beaconUpdating),
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }));
    const __VLS_150 = __VLS_149({
        ...{ 'onClick': {} },
        type: "primary",
        loading: (__VLS_ctx.beaconUpdating),
        disabled: (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled),
    }, ...__VLS_functionalComponentArgsRest(__VLS_149));
    let __VLS_153;
    const __VLS_154 = ({ click: {} },
        { onClick: (__VLS_ctx.handleUpdateBeacon) });
    const { default: __VLS_155 } = __VLS_151.slots;
    // @ts-ignore
    [localAgent, localAgent, beaconUpdating, handleUpdateBeacon,];
    var __VLS_151;
    var __VLS_152;
    // @ts-ignore
    [];
    var __VLS_123;
    if (!__VLS_ctx.localAgent.is_online || __VLS_ctx.localAgent.is_disabled) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "mt-2 text-xs text-orange-500" },
        });
        /** @type {__VLS_StyleScopedClasses['mt-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-orange-500']} */ ;
    }
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex gap-2 justify-end pt-4 border-t border-gray-200 dark:border-[var(--border-default)]" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['justify-end']} */ ;
    /** @type {__VLS_StyleScopedClasses['pt-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-t']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:border-[var(--border-default)]']} */ ;
    let __VLS_156;
    /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
    aButton;
    // @ts-ignore
    const __VLS_157 = __VLS_asFunctionalComponent1(__VLS_156, new __VLS_156({
        ...{ 'onClick': {} },
        type: "primary",
        disabled: (__VLS_ctx.localAgent.is_disabled),
    }));
    const __VLS_158 = __VLS_157({
        ...{ 'onClick': {} },
        type: "primary",
        disabled: (__VLS_ctx.localAgent.is_disabled),
    }, ...__VLS_functionalComponentArgsRest(__VLS_157));
    let __VLS_161;
    const __VLS_162 = ({ click: {} },
        { onClick: (__VLS_ctx.openTaskModal) });
    const { default: __VLS_163 } = __VLS_159.slots;
    // @ts-ignore
    [localAgent, localAgent, localAgent, openTaskModal,];
    var __VLS_159;
    var __VLS_160;
    let __VLS_164;
    /** @ts-ignore @type {typeof __VLS_components.aDropdown | typeof __VLS_components.ADropdown | typeof __VLS_components.aDropdown | typeof __VLS_components.ADropdown} */
    aDropdown;
    // @ts-ignore
    const __VLS_165 = __VLS_asFunctionalComponent1(__VLS_164, new __VLS_164({
        placement: "topRight",
    }));
    const __VLS_166 = __VLS_165({
        placement: "topRight",
    }, ...__VLS_functionalComponentArgsRest(__VLS_165));
    const { default: __VLS_169 } = __VLS_167.slots;
    let __VLS_170;
    /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
    aButton;
    // @ts-ignore
    const __VLS_171 = __VLS_asFunctionalComponent1(__VLS_170, new __VLS_170({}));
    const __VLS_172 = __VLS_171({}, ...__VLS_functionalComponentArgsRest(__VLS_171));
    const { default: __VLS_175 } = __VLS_173.slots;
    let __VLS_176;
    /** @ts-ignore @type {typeof __VLS_components.DownOutlined} */
    DownOutlined;
    // @ts-ignore
    const __VLS_177 = __VLS_asFunctionalComponent1(__VLS_176, new __VLS_176({}));
    const __VLS_178 = __VLS_177({}, ...__VLS_functionalComponentArgsRest(__VLS_177));
    // @ts-ignore
    [];
    var __VLS_173;
    {
        const { overlay: __VLS_181 } = __VLS_167.slots;
        let __VLS_182;
        /** @ts-ignore @type {typeof __VLS_components.aMenu | typeof __VLS_components.AMenu | typeof __VLS_components.aMenu | typeof __VLS_components.AMenu} */
        aMenu;
        // @ts-ignore
        const __VLS_183 = __VLS_asFunctionalComponent1(__VLS_182, new __VLS_182({}));
        const __VLS_184 = __VLS_183({}, ...__VLS_functionalComponentArgsRest(__VLS_183));
        const { default: __VLS_187 } = __VLS_185.slots;
        let __VLS_188;
        /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
        aMenuItem;
        // @ts-ignore
        const __VLS_189 = __VLS_asFunctionalComponent1(__VLS_188, new __VLS_188({
            ...{ 'onClick': {} },
            key: "disconnect",
            disabled: (!__VLS_ctx.localAgent.is_online),
        }));
        const __VLS_190 = __VLS_189({
            ...{ 'onClick': {} },
            key: "disconnect",
            disabled: (!__VLS_ctx.localAgent.is_online),
        }, ...__VLS_functionalComponentArgsRest(__VLS_189));
        let __VLS_193;
        const __VLS_194 = ({ click: {} },
            { onClick: (...[$event]) => {
                    if (!(__VLS_ctx.localAgent))
                        return;
                    __VLS_ctx.handleAction('disconnect');
                    // @ts-ignore
                    [localAgent, handleAction,];
                } });
        const { default: __VLS_195 } = __VLS_191.slots;
        // @ts-ignore
        [];
        var __VLS_191;
        var __VLS_192;
        if (!__VLS_ctx.localAgent.is_disabled) {
            let __VLS_196;
            /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
            aMenuItem;
            // @ts-ignore
            const __VLS_197 = __VLS_asFunctionalComponent1(__VLS_196, new __VLS_196({
                ...{ 'onClick': {} },
                key: "disable",
            }));
            const __VLS_198 = __VLS_197({
                ...{ 'onClick': {} },
                key: "disable",
            }, ...__VLS_functionalComponentArgsRest(__VLS_197));
            let __VLS_201;
            const __VLS_202 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!(__VLS_ctx.localAgent))
                            return;
                        if (!(!__VLS_ctx.localAgent.is_disabled))
                            return;
                        __VLS_ctx.handleAction('disable');
                        // @ts-ignore
                        [localAgent, handleAction,];
                    } });
            const { default: __VLS_203 } = __VLS_199.slots;
            // @ts-ignore
            [];
            var __VLS_199;
            var __VLS_200;
        }
        if (__VLS_ctx.localAgent.is_disabled) {
            let __VLS_204;
            /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
            aMenuItem;
            // @ts-ignore
            const __VLS_205 = __VLS_asFunctionalComponent1(__VLS_204, new __VLS_204({
                ...{ 'onClick': {} },
                key: "enable",
            }));
            const __VLS_206 = __VLS_205({
                ...{ 'onClick': {} },
                key: "enable",
            }, ...__VLS_functionalComponentArgsRest(__VLS_205));
            let __VLS_209;
            const __VLS_210 = ({ click: {} },
                { onClick: (...[$event]) => {
                        if (!(__VLS_ctx.localAgent))
                            return;
                        if (!(__VLS_ctx.localAgent.is_disabled))
                            return;
                        __VLS_ctx.handleAction('enable');
                        // @ts-ignore
                        [localAgent, handleAction,];
                    } });
            const { default: __VLS_211 } = __VLS_207.slots;
            // @ts-ignore
            [];
            var __VLS_207;
            var __VLS_208;
        }
        let __VLS_212;
        /** @ts-ignore @type {typeof __VLS_components.aMenuDivider | typeof __VLS_components.AMenuDivider} */
        aMenuDivider;
        // @ts-ignore
        const __VLS_213 = __VLS_asFunctionalComponent1(__VLS_212, new __VLS_212({}));
        const __VLS_214 = __VLS_213({}, ...__VLS_functionalComponentArgsRest(__VLS_213));
        let __VLS_217;
        /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
        aMenuItem;
        // @ts-ignore
        const __VLS_218 = __VLS_asFunctionalComponent1(__VLS_217, new __VLS_217({
            ...{ 'onClick': {} },
            key: "delete",
            disabled: (__VLS_ctx.localAgent.is_online),
            danger: true,
        }));
        const __VLS_219 = __VLS_218({
            ...{ 'onClick': {} },
            key: "delete",
            disabled: (__VLS_ctx.localAgent.is_online),
            danger: true,
        }, ...__VLS_functionalComponentArgsRest(__VLS_218));
        let __VLS_222;
        const __VLS_223 = ({ click: {} },
            { onClick: (...[$event]) => {
                    if (!(__VLS_ctx.localAgent))
                        return;
                    __VLS_ctx.handleAction('delete');
                    // @ts-ignore
                    [localAgent, handleAction,];
                } });
        const { default: __VLS_224 } = __VLS_220.slots;
        // @ts-ignore
        [];
        var __VLS_220;
        var __VLS_221;
        // @ts-ignore
        [];
        var __VLS_185;
        // @ts-ignore
        [];
    }
    // @ts-ignore
    [];
    var __VLS_167;
}
else {
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex justify-center py-10" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-10']} */ ;
    let __VLS_225;
    /** @ts-ignore @type {typeof __VLS_components.aSpin | typeof __VLS_components.ASpin} */
    aSpin;
    // @ts-ignore
    const __VLS_226 = __VLS_asFunctionalComponent1(__VLS_225, new __VLS_225({}));
    const __VLS_227 = __VLS_226({}, ...__VLS_functionalComponentArgsRest(__VLS_226));
}
// @ts-ignore
[];
var __VLS_3;
var __VLS_4;
// @ts-ignore
var __VLS_81 = __VLS_80;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({
    emits: {},
    __typeProps: {},
});
export default {};
