/// <reference types="../../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { DesktopOutlined, DisconnectOutlined, StopOutlined, CheckCircleOutlined, DeleteOutlined, CodeOutlined, FolderOpenOutlined, CameraOutlined } from '@ant-design/icons-vue';
const props = defineProps();
const emit = defineEmits(['action', 'open-task', 'open-terminal', 'close']);
function emitAction(type, act) {
    if (type === 'action') {
        emit('action', { action: act, agent: props.agent });
    }
    else if (type === 'open-task') {
        emit('open-task', props.agent);
    }
    else {
        emit('open-terminal', props.agent);
    }
    emit('close');
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
/** @ts-ignore @type {typeof __VLS_components.Teleport | typeof __VLS_components.Teleport} */
Teleport;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    to: "body",
}));
const __VLS_2 = __VLS_1({
    to: "body",
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
const { default: __VLS_5 } = __VLS_3.slots;
if (__VLS_ctx.visible) {
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ onClick: () => { } },
        ...{ class: "fixed z-50 bg-white dark:bg-[#1C1E22] border border-gray-200 dark:border-gray-700 rounded-md shadow-lg py-1 min-w-[160px]" },
        ...{ style: ({ top: `${__VLS_ctx.y}px`, left: `${__VLS_ctx.x}px` }) },
    });
    /** @type {__VLS_StyleScopedClasses['fixed']} */ ;
    /** @type {__VLS_StyleScopedClasses['z-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[#1C1E22]']} */ ;
    /** @type {__VLS_StyleScopedClasses['border']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:border-gray-700']} */ ;
    /** @type {__VLS_StyleScopedClasses['rounded-md']} */ ;
    /** @type {__VLS_StyleScopedClasses['shadow-lg']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-1']} */ ;
    /** @type {__VLS_StyleScopedClasses['min-w-[160px]']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "px-3 py-1.5 text-xs text-slate-400 border-b border-gray-100 dark:border-gray-800 mb-1" },
    });
    /** @type {__VLS_StyleScopedClasses['px-3']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-1.5']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-b']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-gray-100']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:border-gray-800']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-1']} */ ;
    (__VLS_ctx.agent?.agent_id);
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ onClick: (...[$event]) => {
                if (!(__VLS_ctx.visible))
                    return;
                !__VLS_ctx.agent?.is_disabled && __VLS_ctx.emitAction('open-task');
                // @ts-ignore
                [visible, y, x, agent, agent, emitAction,];
            } },
        ...{ class: "px-3 py-2 hover:bg-blue-50 dark:hover:bg-blue-900/20 cursor-pointer flex items-center gap-2 text-sm text-slate-700 dark:text-slate-200" },
        ...{ class: ({ 'opacity-50 cursor-not-allowed': __VLS_ctx.agent?.is_disabled }) },
    });
    /** @type {__VLS_StyleScopedClasses['px-3']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['hover:bg-blue-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:hover:bg-blue-900/20']} */ ;
    /** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-700']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:text-slate-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['opacity-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['cursor-not-allowed']} */ ;
    let __VLS_6;
    /** @ts-ignore @type {typeof __VLS_components.CodeOutlined} */
    CodeOutlined;
    // @ts-ignore
    const __VLS_7 = __VLS_asFunctionalComponent1(__VLS_6, new __VLS_6({}));
    const __VLS_8 = __VLS_7({}, ...__VLS_functionalComponentArgsRest(__VLS_7));
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ onClick: (...[$event]) => {
                if (!(__VLS_ctx.visible))
                    return;
                !__VLS_ctx.agent?.is_disabled && __VLS_ctx.emitAction('open-terminal');
                // @ts-ignore
                [agent, agent, emitAction,];
            } },
        ...{ class: "px-3 py-2 hover:bg-blue-50 dark:hover:bg-blue-900/20 cursor-pointer flex items-center gap-2 text-sm text-slate-700 dark:text-slate-200" },
        ...{ class: ({ 'opacity-50 cursor-not-allowed': __VLS_ctx.agent?.is_disabled }) },
    });
    /** @type {__VLS_StyleScopedClasses['px-3']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['hover:bg-blue-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:hover:bg-blue-900/20']} */ ;
    /** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-700']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:text-slate-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['opacity-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['cursor-not-allowed']} */ ;
    let __VLS_11;
    /** @ts-ignore @type {typeof __VLS_components.DesktopOutlined} */
    DesktopOutlined;
    // @ts-ignore
    const __VLS_12 = __VLS_asFunctionalComponent1(__VLS_11, new __VLS_11({}));
    const __VLS_13 = __VLS_12({}, ...__VLS_functionalComponentArgsRest(__VLS_12));
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "px-3 py-2 hover:bg-blue-50 dark:hover:bg-blue-900/20 cursor-pointer flex items-center gap-2 text-sm text-slate-700 dark:text-slate-200 opacity-50 cursor-not-allowed" },
        title: "功能开发中",
    });
    /** @type {__VLS_StyleScopedClasses['px-3']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['hover:bg-blue-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:hover:bg-blue-900/20']} */ ;
    /** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-700']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:text-slate-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['opacity-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['cursor-not-allowed']} */ ;
    let __VLS_16;
    /** @ts-ignore @type {typeof __VLS_components.FolderOpenOutlined} */
    FolderOpenOutlined;
    // @ts-ignore
    const __VLS_17 = __VLS_asFunctionalComponent1(__VLS_16, new __VLS_16({}));
    const __VLS_18 = __VLS_17({}, ...__VLS_functionalComponentArgsRest(__VLS_17));
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "px-3 py-2 hover:bg-blue-50 dark:hover:bg-blue-900/20 cursor-pointer flex items-center gap-2 text-sm text-slate-700 dark:text-slate-200 opacity-50 cursor-not-allowed" },
        title: "功能开发中",
    });
    /** @type {__VLS_StyleScopedClasses['px-3']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['hover:bg-blue-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:hover:bg-blue-900/20']} */ ;
    /** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-700']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:text-slate-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['opacity-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['cursor-not-allowed']} */ ;
    let __VLS_21;
    /** @ts-ignore @type {typeof __VLS_components.CameraOutlined} */
    CameraOutlined;
    // @ts-ignore
    const __VLS_22 = __VLS_asFunctionalComponent1(__VLS_21, new __VLS_21({}));
    const __VLS_23 = __VLS_22({}, ...__VLS_functionalComponentArgsRest(__VLS_22));
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "h-[1px] bg-gray-100 dark:bg-gray-800 my-1" },
    });
    /** @type {__VLS_StyleScopedClasses['h-[1px]']} */ ;
    /** @type {__VLS_StyleScopedClasses['bg-gray-100']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-gray-800']} */ ;
    /** @type {__VLS_StyleScopedClasses['my-1']} */ ;
    if (!__VLS_ctx.agent?.is_disabled) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ onClick: (...[$event]) => {
                    if (!(__VLS_ctx.visible))
                        return;
                    if (!(!__VLS_ctx.agent?.is_disabled))
                        return;
                    __VLS_ctx.agent && __VLS_ctx.emitAction('action', 'disable');
                    // @ts-ignore
                    [agent, agent, agent, emitAction,];
                } },
            ...{ class: "px-3 py-2 hover:bg-orange-50 dark:hover:bg-orange-900/20 cursor-pointer flex items-center gap-2 text-sm text-orange-600 dark:text-orange-400" },
        });
        /** @type {__VLS_StyleScopedClasses['px-3']} */ ;
        /** @type {__VLS_StyleScopedClasses['py-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['hover:bg-orange-50']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:hover:bg-orange-900/20']} */ ;
        /** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-orange-600']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-orange-400']} */ ;
        let __VLS_26;
        /** @ts-ignore @type {typeof __VLS_components.StopOutlined} */
        StopOutlined;
        // @ts-ignore
        const __VLS_27 = __VLS_asFunctionalComponent1(__VLS_26, new __VLS_26({}));
        const __VLS_28 = __VLS_27({}, ...__VLS_functionalComponentArgsRest(__VLS_27));
    }
    if (__VLS_ctx.agent?.is_disabled) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ onClick: (...[$event]) => {
                    if (!(__VLS_ctx.visible))
                        return;
                    if (!(__VLS_ctx.agent?.is_disabled))
                        return;
                    __VLS_ctx.agent && __VLS_ctx.emitAction('action', 'enable');
                    // @ts-ignore
                    [agent, agent, emitAction,];
                } },
            ...{ class: "px-3 py-2 hover:bg-green-50 dark:hover:bg-green-900/20 cursor-pointer flex items-center gap-2 text-sm text-green-600 dark:text-green-400" },
        });
        /** @type {__VLS_StyleScopedClasses['px-3']} */ ;
        /** @type {__VLS_StyleScopedClasses['py-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['hover:bg-green-50']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:hover:bg-green-900/20']} */ ;
        /** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-green-600']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-green-400']} */ ;
        let __VLS_31;
        /** @ts-ignore @type {typeof __VLS_components.CheckCircleOutlined} */
        CheckCircleOutlined;
        // @ts-ignore
        const __VLS_32 = __VLS_asFunctionalComponent1(__VLS_31, new __VLS_31({}));
        const __VLS_33 = __VLS_32({}, ...__VLS_functionalComponentArgsRest(__VLS_32));
    }
    if (__VLS_ctx.agent?.is_online) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ onClick: (...[$event]) => {
                    if (!(__VLS_ctx.visible))
                        return;
                    if (!(__VLS_ctx.agent?.is_online))
                        return;
                    __VLS_ctx.agent && __VLS_ctx.emitAction('action', 'disconnect');
                    // @ts-ignore
                    [agent, agent, emitAction,];
                } },
            ...{ class: "px-3 py-2 hover:bg-red-50 dark:hover:bg-red-900/20 cursor-pointer flex items-center gap-2 text-sm text-red-600 dark:text-red-400" },
        });
        /** @type {__VLS_StyleScopedClasses['px-3']} */ ;
        /** @type {__VLS_StyleScopedClasses['py-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['hover:bg-red-50']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:hover:bg-red-900/20']} */ ;
        /** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-red-600']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-red-400']} */ ;
        let __VLS_36;
        /** @ts-ignore @type {typeof __VLS_components.DisconnectOutlined} */
        DisconnectOutlined;
        // @ts-ignore
        const __VLS_37 = __VLS_asFunctionalComponent1(__VLS_36, new __VLS_36({}));
        const __VLS_38 = __VLS_37({}, ...__VLS_functionalComponentArgsRest(__VLS_37));
    }
    if (!__VLS_ctx.agent?.is_online) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ onClick: (...[$event]) => {
                    if (!(__VLS_ctx.visible))
                        return;
                    if (!(!__VLS_ctx.agent?.is_online))
                        return;
                    __VLS_ctx.agent && __VLS_ctx.emitAction('action', 'delete');
                    // @ts-ignore
                    [agent, agent, emitAction,];
                } },
            ...{ class: "px-3 py-2 hover:bg-red-50 dark:hover:bg-red-900/20 cursor-pointer flex items-center gap-2 text-sm text-red-600 dark:text-red-400" },
        });
        /** @type {__VLS_StyleScopedClasses['px-3']} */ ;
        /** @type {__VLS_StyleScopedClasses['py-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['hover:bg-red-50']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:hover:bg-red-900/20']} */ ;
        /** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-red-600']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-red-400']} */ ;
        let __VLS_41;
        /** @ts-ignore @type {typeof __VLS_components.DeleteOutlined} */
        DeleteOutlined;
        // @ts-ignore
        const __VLS_42 = __VLS_asFunctionalComponent1(__VLS_41, new __VLS_41({}));
        const __VLS_43 = __VLS_42({}, ...__VLS_functionalComponentArgsRest(__VLS_42));
    }
}
// @ts-ignore
[];
var __VLS_3;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({
    emits: {},
    __typeProps: {},
});
export default {};
