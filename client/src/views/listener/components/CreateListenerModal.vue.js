import { ref, reactive } from 'vue';
import { message } from 'ant-design-vue';
import { spawnListener } from '@/api/listener';
const props = defineProps();
const emit = defineEmits(['update:visible', 'success']);
const formRef = ref();
const submitting = ref(false);
const formState = reactive({
    name: '',
    protocol: 'TCP',
    bind_host: '0.0.0.0',
    bind_port: 1234,
});
const rules = {
    name: [{ required: true, message: '请输入监听器名称' }],
    protocol: [{ required: true, message: '请选择协议' }],
    bind_host: [{ required: true, message: '请输入绑定地址' }],
    bind_port: [{ required: true, message: '请输入合法端口', type: 'number' }],
};
const handleCancel = () => {
    emit('update:visible', false);
    formRef.value?.resetFields();
};
const handleSubmit = async () => {
    try {
        await formRef.value?.validate();
        submitting.value = true;
        await spawnListener(formState);
        message.success('监听器创建成功');
        emit('success');
        handleCancel();
    }
    catch (err) {
        if (err.errorFields)
            return; // Validation failed natively
        message.error(err.message || '操作失败');
    }
    finally {
        submitting.value = false;
    }
};
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
    ...{ 'onUpdate:open': {} },
    ...{ 'onCancel': {} },
    ...{ 'onOk': {} },
    open: (__VLS_ctx.visible),
    title: "新建监听器 (Spawn Listener)",
    confirmLoading: (__VLS_ctx.submitting),
    destroyOnClose: true,
}));
const __VLS_2 = __VLS_1({
    ...{ 'onUpdate:open': {} },
    ...{ 'onCancel': {} },
    ...{ 'onOk': {} },
    open: (__VLS_ctx.visible),
    title: "新建监听器 (Spawn Listener)",
    confirmLoading: (__VLS_ctx.submitting),
    destroyOnClose: true,
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
let __VLS_5;
const __VLS_6 = ({ 'update:open': {} },
    { 'onUpdate:open': (...[$event]) => {
            __VLS_ctx.$emit('update:visible', $event);
            // @ts-ignore
            [visible, submitting, $emit,];
        } });
const __VLS_7 = ({ cancel: {} },
    { onCancel: (__VLS_ctx.handleCancel) });
const __VLS_8 = ({ ok: {} },
    { onOk: (__VLS_ctx.handleSubmit) });
var __VLS_9 = {};
const { default: __VLS_10 } = __VLS_3.slots;
let __VLS_11;
/** @ts-ignore @type {typeof __VLS_components.aForm | typeof __VLS_components.AForm | typeof __VLS_components.aForm | typeof __VLS_components.AForm} */
aForm;
// @ts-ignore
const __VLS_12 = __VLS_asFunctionalComponent1(__VLS_11, new __VLS_11({
    model: (__VLS_ctx.formState),
    rules: (__VLS_ctx.rules),
    ref: "formRef",
    layout: "vertical",
    ...{ class: "mt-4" },
}));
const __VLS_13 = __VLS_12({
    model: (__VLS_ctx.formState),
    rules: (__VLS_ctx.rules),
    ref: "formRef",
    layout: "vertical",
    ...{ class: "mt-4" },
}, ...__VLS_functionalComponentArgsRest(__VLS_12));
var __VLS_16 = {};
/** @type {__VLS_StyleScopedClasses['mt-4']} */ ;
const { default: __VLS_18 } = __VLS_14.slots;
let __VLS_19;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_20 = __VLS_asFunctionalComponent1(__VLS_19, new __VLS_19({
    label: "监听器名称 (Name)",
    name: "name",
}));
const __VLS_21 = __VLS_20({
    label: "监听器名称 (Name)",
    name: "name",
}, ...__VLS_functionalComponentArgsRest(__VLS_20));
const { default: __VLS_24 } = __VLS_22.slots;
let __VLS_25;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_26 = __VLS_asFunctionalComponent1(__VLS_25, new __VLS_25({
    value: (__VLS_ctx.formState.name),
    placeholder: "请输入标识名称，如 HTTPS Beacon US-East",
}));
const __VLS_27 = __VLS_26({
    value: (__VLS_ctx.formState.name),
    placeholder: "请输入标识名称，如 HTTPS Beacon US-East",
}, ...__VLS_functionalComponentArgsRest(__VLS_26));
// @ts-ignore
[handleCancel, handleSubmit, formState, formState, rules,];
var __VLS_22;
let __VLS_30;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_31 = __VLS_asFunctionalComponent1(__VLS_30, new __VLS_30({
    label: "协议类型 (Protocol)",
    name: "protocol",
}));
const __VLS_32 = __VLS_31({
    label: "协议类型 (Protocol)",
    name: "protocol",
}, ...__VLS_functionalComponentArgsRest(__VLS_31));
const { default: __VLS_35 } = __VLS_33.slots;
let __VLS_36;
/** @ts-ignore @type {typeof __VLS_components.aSelect | typeof __VLS_components.ASelect | typeof __VLS_components.aSelect | typeof __VLS_components.ASelect} */
aSelect;
// @ts-ignore
const __VLS_37 = __VLS_asFunctionalComponent1(__VLS_36, new __VLS_36({
    value: (__VLS_ctx.formState.protocol),
    placeholder: "选择通信协议",
}));
const __VLS_38 = __VLS_37({
    value: (__VLS_ctx.formState.protocol),
    placeholder: "选择通信协议",
}, ...__VLS_functionalComponentArgsRest(__VLS_37));
const { default: __VLS_41 } = __VLS_39.slots;
let __VLS_42;
/** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
aSelectOption;
// @ts-ignore
const __VLS_43 = __VLS_asFunctionalComponent1(__VLS_42, new __VLS_42({
    value: "TCP",
}));
const __VLS_44 = __VLS_43({
    value: "TCP",
}, ...__VLS_functionalComponentArgsRest(__VLS_43));
const { default: __VLS_47 } = __VLS_45.slots;
// @ts-ignore
[formState,];
var __VLS_45;
let __VLS_48;
/** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
aSelectOption;
// @ts-ignore
const __VLS_49 = __VLS_asFunctionalComponent1(__VLS_48, new __VLS_48({
    value: "HTTP",
}));
const __VLS_50 = __VLS_49({
    value: "HTTP",
}, ...__VLS_functionalComponentArgsRest(__VLS_49));
const { default: __VLS_53 } = __VLS_51.slots;
// @ts-ignore
[];
var __VLS_51;
let __VLS_54;
/** @ts-ignore @type {typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption | typeof __VLS_components.aSelectOption | typeof __VLS_components.ASelectOption} */
aSelectOption;
// @ts-ignore
const __VLS_55 = __VLS_asFunctionalComponent1(__VLS_54, new __VLS_54({
    value: "HTTPS",
}));
const __VLS_56 = __VLS_55({
    value: "HTTPS",
}, ...__VLS_functionalComponentArgsRest(__VLS_55));
const { default: __VLS_59 } = __VLS_57.slots;
// @ts-ignore
[];
var __VLS_57;
// @ts-ignore
[];
var __VLS_39;
// @ts-ignore
[];
var __VLS_33;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex gap-4" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['gap-4']} */ ;
let __VLS_60;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_61 = __VLS_asFunctionalComponent1(__VLS_60, new __VLS_60({
    label: "绑定地址 (Bind Host)",
    name: "bind_host",
    ...{ class: "flex-1" },
}));
const __VLS_62 = __VLS_61({
    label: "绑定地址 (Bind Host)",
    name: "bind_host",
    ...{ class: "flex-1" },
}, ...__VLS_functionalComponentArgsRest(__VLS_61));
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
const { default: __VLS_65 } = __VLS_63.slots;
let __VLS_66;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_67 = __VLS_asFunctionalComponent1(__VLS_66, new __VLS_66({
    value: (__VLS_ctx.formState.bind_host),
    placeholder: "0.0.0.0",
}));
const __VLS_68 = __VLS_67({
    value: (__VLS_ctx.formState.bind_host),
    placeholder: "0.0.0.0",
}, ...__VLS_functionalComponentArgsRest(__VLS_67));
// @ts-ignore
[formState,];
var __VLS_63;
let __VLS_71;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_72 = __VLS_asFunctionalComponent1(__VLS_71, new __VLS_71({
    label: "绑定端口 (Bind Port)",
    name: "bind_port",
    ...{ class: "flex-1" },
}));
const __VLS_73 = __VLS_72({
    label: "绑定端口 (Bind Port)",
    name: "bind_port",
    ...{ class: "flex-1" },
}, ...__VLS_functionalComponentArgsRest(__VLS_72));
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
const { default: __VLS_76 } = __VLS_74.slots;
let __VLS_77;
/** @ts-ignore @type {typeof __VLS_components.aInputNumber | typeof __VLS_components.AInputNumber} */
aInputNumber;
// @ts-ignore
const __VLS_78 = __VLS_asFunctionalComponent1(__VLS_77, new __VLS_77({
    value: (__VLS_ctx.formState.bind_port),
    min: (1),
    max: (65535),
    ...{ class: "w-full" },
}));
const __VLS_79 = __VLS_78({
    value: (__VLS_ctx.formState.bind_port),
    min: (1),
    max: (65535),
    ...{ class: "w-full" },
}, ...__VLS_functionalComponentArgsRest(__VLS_78));
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
// @ts-ignore
[formState,];
var __VLS_74;
// @ts-ignore
[];
var __VLS_14;
// @ts-ignore
[];
var __VLS_3;
var __VLS_4;
// @ts-ignore
var __VLS_17 = __VLS_16;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({
    emits: {},
    __typeProps: {},
});
export default {};
