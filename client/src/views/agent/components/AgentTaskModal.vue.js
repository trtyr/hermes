/// <reference types="../../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, reactive, watch } from 'vue';
import { message } from 'ant-design-vue';
import { dispatchTask } from '@/api/agent';
const props = defineProps();
const emit = defineEmits(['update:visible']);
const taskSubmitting = ref(false);
const taskForm = reactive({ cmd: '', args: '' });
watch(() => props.visible, (newVal) => {
    if (newVal) {
        taskForm.cmd = '';
        taskForm.args = '';
    }
});
async function submitTask() {
    if (!props.agent || !taskForm.cmd) {
        message.warning('请填写完整的命令');
        return;
    }
    taskSubmitting.value = true;
    try {
        const argsArray = taskForm.args.trim().split(/\s+/).filter(Boolean);
        const payload = { kind: 'shell', cmd: taskForm.cmd, args: argsArray };
        await dispatchTask(props.agent.agent_id, payload);
        message.success('任务下发成功');
        emit('update:visible', false);
    }
    catch (e) {
        message.error(e.message);
    }
    finally {
        taskSubmitting.value = false;
    }
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
/** @ts-ignore @type {typeof __VLS_components.aModal | typeof __VLS_components.AModal | typeof __VLS_components.aModal | typeof __VLS_components.AModal} */
aModal;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    ...{ 'onOk': {} },
    ...{ 'onUpdate:open': {} },
    open: (__VLS_ctx.visible),
    title: (`快速下发任务至 ${__VLS_ctx.agent?.agent_id || ''}`),
    confirmLoading: (__VLS_ctx.taskSubmitting),
}));
const __VLS_2 = __VLS_1({
    ...{ 'onOk': {} },
    ...{ 'onUpdate:open': {} },
    open: (__VLS_ctx.visible),
    title: (`快速下发任务至 ${__VLS_ctx.agent?.agent_id || ''}`),
    confirmLoading: (__VLS_ctx.taskSubmitting),
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
let __VLS_5;
const __VLS_6 = ({ ok: {} },
    { onOk: (__VLS_ctx.submitTask) });
const __VLS_7 = ({ 'update:open': {} },
    { 'onUpdate:open': (...[$event]) => {
            __VLS_ctx.$emit('update:visible', $event);
            // @ts-ignore
            [visible, agent, taskSubmitting, submitTask, $emit,];
        } });
var __VLS_8 = {};
const { default: __VLS_9 } = __VLS_3.slots;
let __VLS_10;
/** @ts-ignore @type {typeof __VLS_components.aForm | typeof __VLS_components.AForm | typeof __VLS_components.aForm | typeof __VLS_components.AForm} */
aForm;
// @ts-ignore
const __VLS_11 = __VLS_asFunctionalComponent1(__VLS_10, new __VLS_10({
    layout: "vertical",
    ...{ class: "mt-4" },
}));
const __VLS_12 = __VLS_11({
    layout: "vertical",
    ...{ class: "mt-4" },
}, ...__VLS_functionalComponentArgsRest(__VLS_11));
/** @type {__VLS_StyleScopedClasses['mt-4']} */ ;
const { default: __VLS_15 } = __VLS_13.slots;
let __VLS_16;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_17 = __VLS_asFunctionalComponent1(__VLS_16, new __VLS_16({
    label: "快捷命令模板",
}));
const __VLS_18 = __VLS_17({
    label: "快捷命令模板",
}, ...__VLS_functionalComponentArgsRest(__VLS_17));
const { default: __VLS_21 } = __VLS_19.slots;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex gap-2 mb-2" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
/** @type {__VLS_StyleScopedClasses['mb-2']} */ ;
let __VLS_22;
/** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
aTag;
// @ts-ignore
const __VLS_23 = __VLS_asFunctionalComponent1(__VLS_22, new __VLS_22({
    ...{ 'onClick': {} },
    color: "blue",
    ...{ class: "cursor-pointer" },
}));
const __VLS_24 = __VLS_23({
    ...{ 'onClick': {} },
    color: "blue",
    ...{ class: "cursor-pointer" },
}, ...__VLS_functionalComponentArgsRest(__VLS_23));
let __VLS_27;
const __VLS_28 = ({ click: {} },
    { onClick: (...[$event]) => {
            __VLS_ctx.taskForm.cmd = 'whoami';
            __VLS_ctx.taskForm.args = '';
            // @ts-ignore
            [taskForm, taskForm,];
        } });
/** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
const { default: __VLS_29 } = __VLS_25.slots;
// @ts-ignore
[];
var __VLS_25;
var __VLS_26;
let __VLS_30;
/** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
aTag;
// @ts-ignore
const __VLS_31 = __VLS_asFunctionalComponent1(__VLS_30, new __VLS_30({
    ...{ 'onClick': {} },
    color: "blue",
    ...{ class: "cursor-pointer" },
}));
const __VLS_32 = __VLS_31({
    ...{ 'onClick': {} },
    color: "blue",
    ...{ class: "cursor-pointer" },
}, ...__VLS_functionalComponentArgsRest(__VLS_31));
let __VLS_35;
const __VLS_36 = ({ click: {} },
    { onClick: (...[$event]) => {
            __VLS_ctx.taskForm.cmd = 'hostname';
            __VLS_ctx.taskForm.args = '';
            // @ts-ignore
            [taskForm, taskForm,];
        } });
/** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
const { default: __VLS_37 } = __VLS_33.slots;
// @ts-ignore
[];
var __VLS_33;
var __VLS_34;
let __VLS_38;
/** @ts-ignore @type {typeof __VLS_components.aTag | typeof __VLS_components.ATag | typeof __VLS_components.aTag | typeof __VLS_components.ATag} */
aTag;
// @ts-ignore
const __VLS_39 = __VLS_asFunctionalComponent1(__VLS_38, new __VLS_38({
    ...{ 'onClick': {} },
    color: "blue",
    ...{ class: "cursor-pointer" },
}));
const __VLS_40 = __VLS_39({
    ...{ 'onClick': {} },
    color: "blue",
    ...{ class: "cursor-pointer" },
}, ...__VLS_functionalComponentArgsRest(__VLS_39));
let __VLS_43;
const __VLS_44 = ({ click: {} },
    { onClick: (...[$event]) => {
            __VLS_ctx.taskForm.cmd = 'ipconfig';
            __VLS_ctx.taskForm.args = '';
            // @ts-ignore
            [taskForm, taskForm,];
        } });
/** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
const { default: __VLS_45 } = __VLS_41.slots;
// @ts-ignore
[];
var __VLS_41;
var __VLS_42;
// @ts-ignore
[];
var __VLS_19;
let __VLS_46;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_47 = __VLS_asFunctionalComponent1(__VLS_46, new __VLS_46({
    label: "执行命令 (Command)",
    required: true,
}));
const __VLS_48 = __VLS_47({
    label: "执行命令 (Command)",
    required: true,
}, ...__VLS_functionalComponentArgsRest(__VLS_47));
const { default: __VLS_51 } = __VLS_49.slots;
let __VLS_52;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_53 = __VLS_asFunctionalComponent1(__VLS_52, new __VLS_52({
    value: (__VLS_ctx.taskForm.cmd),
    placeholder: "例如: whoami",
}));
const __VLS_54 = __VLS_53({
    value: (__VLS_ctx.taskForm.cmd),
    placeholder: "例如: whoami",
}, ...__VLS_functionalComponentArgsRest(__VLS_53));
// @ts-ignore
[taskForm,];
var __VLS_49;
let __VLS_57;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_58 = __VLS_asFunctionalComponent1(__VLS_57, new __VLS_57({
    label: "参数 (Args)",
}));
const __VLS_59 = __VLS_58({
    label: "参数 (Args)",
}, ...__VLS_functionalComponentArgsRest(__VLS_58));
const { default: __VLS_62 } = __VLS_60.slots;
let __VLS_63;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_64 = __VLS_asFunctionalComponent1(__VLS_63, new __VLS_63({
    value: (__VLS_ctx.taskForm.args),
    placeholder: "以空格分隔的参数列表",
}));
const __VLS_65 = __VLS_64({
    value: (__VLS_ctx.taskForm.args),
    placeholder: "以空格分隔的参数列表",
}, ...__VLS_functionalComponentArgsRest(__VLS_64));
// @ts-ignore
[taskForm,];
var __VLS_60;
// @ts-ignore
[];
var __VLS_13;
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
