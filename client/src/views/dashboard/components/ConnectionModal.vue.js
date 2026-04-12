/// <reference types="../../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, reactive } from 'vue';
import { message } from 'ant-design-vue';
import { DeleteOutlined, EditOutlined, InfoCircleOutlined } from '@ant-design/icons-vue';
import { useConnectionStore } from '@/store/connection';
import { testConnection } from '@/api/connection';
const props = defineProps();
const emit = defineEmits(['update:visible']);
const connectionStore = useConnectionStore();
const isEditing = ref(false);
const editingId = ref(null);
const isTesting = ref(false);
const testMessage = ref('');
const testStatus = reactive({
    reachability: 'pending',
    auth: 'pending'
});
const formState = reactive({
    connection_name: '',
    server_url: '',
    api_token: ''
});
function resetForm() {
    formState.connection_name = '';
    formState.server_url = '';
    formState.api_token = '';
    editingId.value = null;
    testMessage.value = '';
    testStatus.reachability = 'pending';
    testStatus.auth = 'pending';
}
function startNewConnection() {
    resetForm();
    isEditing.value = true;
}
function editProfile(profile) {
    formState.connection_name = profile.connection_name;
    formState.server_url = profile.server_url;
    formState.api_token = profile.api_token;
    editingId.value = profile.id;
    isEditing.value = true;
    testMessage.value = '';
    testStatus.reachability = 'pending';
    testStatus.auth = 'pending';
}
function cancelEdit() {
    isEditing.value = false;
    resetForm();
}
async function runTest() {
    if (!formState.server_url || !formState.api_token) {
        message.warning('请先填写服务端地址和 API Token');
        return false;
    }
    isTesting.value = true;
    testMessage.value = '';
    testStatus.reachability = 'pending';
    testStatus.auth = 'pending';
    try {
        const result = await testConnection(formState.server_url, formState.api_token);
        if (result.success) {
            testStatus.reachability = 'success';
            testStatus.auth = 'success';
            message.success('连接测试成功！');
            return true;
        }
        else {
            if (result.errorType === 'network') {
                testStatus.reachability = 'error';
                testStatus.auth = 'pending';
            }
            else {
                testStatus.reachability = 'success';
                testStatus.auth = 'error';
            }
            testMessage.value = result.message || '连接失败';
            return false;
        }
    }
    catch (error) {
        testStatus.reachability = 'error';
        testStatus.auth = 'pending';
        testMessage.value = error.message || '网络请求异常';
        return false;
    }
    finally {
        isTesting.value = false;
    }
}
function selectAndConnect(id) {
    connectionStore.setActiveProfile(id);
    emit('update:visible', false);
}
async function saveConnection() {
    if (!formState.server_url || !formState.api_token) {
        message.warning('请先填写服务端地址和 API Token');
        return;
    }
    formState.server_url = connectionStore.normalizeUrl(formState.server_url);
    const testPassed = await runTest();
    if (!testPassed)
        return;
    if (editingId.value) {
        connectionStore.updateProfile(editingId.value, {
            connection_name: formState.connection_name,
            server_url: formState.server_url,
            api_token: formState.api_token
        });
        connectionStore.setActiveProfile(editingId.value);
    }
    else {
        const newProfile = connectionStore.addProfile({
            connection_name: formState.connection_name,
            server_url: formState.server_url,
            api_token: formState.api_token
        });
        connectionStore.setActiveProfile(newProfile.id);
    }
    message.success('已保存并连接到后端！');
    isEditing.value = false;
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
/** @ts-ignore @type {typeof __VLS_components.aModal | typeof __VLS_components.AModal | typeof __VLS_components.aModal | typeof __VLS_components.AModal} */
aModal;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    ...{ 'onUpdate:open': {} },
    open: (__VLS_ctx.visible),
    title: "后端连接管理",
    footer: (null),
    width: "600px",
    destroyOnClose: true,
}));
const __VLS_2 = __VLS_1({
    ...{ 'onUpdate:open': {} },
    open: (__VLS_ctx.visible),
    title: "后端连接管理",
    footer: (null),
    width: "600px",
    destroyOnClose: true,
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
let __VLS_5;
const __VLS_6 = ({ 'update:open': {} },
    { 'onUpdate:open': (...[$event]) => {
            __VLS_ctx.$emit('update:visible', $event);
            // @ts-ignore
            [visible, $emit,];
        } });
var __VLS_7 = {};
const { default: __VLS_8 } = __VLS_3.slots;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "mt-4" },
});
/** @type {__VLS_StyleScopedClasses['mt-4']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "mb-6" },
});
/** @type {__VLS_StyleScopedClasses['mb-6']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex justify-between items-center mb-2" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-between']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['mb-2']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.h3, __VLS_intrinsics.h3)({
    ...{ class: "text-sm font-medium text-slate-700 dark:text-slate-300" },
});
/** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
/** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
/** @type {__VLS_StyleScopedClasses['text-slate-700']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-slate-300']} */ ;
let __VLS_9;
/** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
aButton;
// @ts-ignore
const __VLS_10 = __VLS_asFunctionalComponent1(__VLS_9, new __VLS_9({
    ...{ 'onClick': {} },
    type: "link",
    size: "small",
}));
const __VLS_11 = __VLS_10({
    ...{ 'onClick': {} },
    type: "link",
    size: "small",
}, ...__VLS_functionalComponentArgsRest(__VLS_10));
let __VLS_14;
const __VLS_15 = ({ click: {} },
    { onClick: (__VLS_ctx.startNewConnection) });
const { default: __VLS_16 } = __VLS_12.slots;
// @ts-ignore
[startNewConnection,];
var __VLS_12;
var __VLS_13;
if (__VLS_ctx.connectionStore.profiles.length === 0) {
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "text-center py-4 text-slate-400 bg-slate-50 dark:bg-[#14161A] rounded-md border border-dashed border-slate-200 dark:border-slate-700" },
    });
    /** @type {__VLS_StyleScopedClasses['text-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['py-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
    /** @type {__VLS_StyleScopedClasses['bg-slate-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[#14161A]']} */ ;
    /** @type {__VLS_StyleScopedClasses['rounded-md']} */ ;
    /** @type {__VLS_StyleScopedClasses['border']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-dashed']} */ ;
    /** @type {__VLS_StyleScopedClasses['border-slate-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:border-slate-700']} */ ;
}
else {
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "space-y-2 max-h-48 overflow-y-auto pr-1" },
    });
    /** @type {__VLS_StyleScopedClasses['space-y-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['max-h-48']} */ ;
    /** @type {__VLS_StyleScopedClasses['overflow-y-auto']} */ ;
    /** @type {__VLS_StyleScopedClasses['pr-1']} */ ;
    for (const [profile] of __VLS_vFor((__VLS_ctx.connectionStore.profiles))) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ onClick: (...[$event]) => {
                    if (!!(__VLS_ctx.connectionStore.profiles.length === 0))
                        return;
                    __VLS_ctx.selectAndConnect(profile.id);
                    // @ts-ignore
                    [connectionStore, connectionStore, selectAndConnect,];
                } },
            key: (profile.id),
            ...{ class: "flex items-center justify-between p-3 rounded-md border transition-colors cursor-pointer" },
            ...{ class: (__VLS_ctx.connectionStore.activeProfileId === profile.id ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/10' : 'border-slate-200 dark:border-[#14161A] bg-white dark:bg-[#1C1E22] hover:border-blue-300') },
        });
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['justify-between']} */ ;
        /** @type {__VLS_StyleScopedClasses['p-3']} */ ;
        /** @type {__VLS_StyleScopedClasses['rounded-md']} */ ;
        /** @type {__VLS_StyleScopedClasses['border']} */ ;
        /** @type {__VLS_StyleScopedClasses['transition-colors']} */ ;
        /** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "flex-1 min-w-0" },
        });
        /** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
        /** @type {__VLS_StyleScopedClasses['min-w-0']} */ ;
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "flex items-center space-x-2" },
        });
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['space-x-2']} */ ;
        __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
            ...{ class: "font-medium text-slate-800 dark:text-slate-100 truncate" },
        });
        /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-slate-800']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-slate-100']} */ ;
        /** @type {__VLS_StyleScopedClasses['truncate']} */ ;
        (profile.connection_name || '未命名连接');
        if (__VLS_ctx.connectionStore.activeProfileId === profile.id) {
            __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
                ...{ class: "text-[10px] px-1.5 py-0.5 bg-blue-100 text-blue-600 dark:bg-blue-800 dark:text-blue-300 rounded text-nowrap" },
            });
            /** @type {__VLS_StyleScopedClasses['text-[10px]']} */ ;
            /** @type {__VLS_StyleScopedClasses['px-1.5']} */ ;
            /** @type {__VLS_StyleScopedClasses['py-0.5']} */ ;
            /** @type {__VLS_StyleScopedClasses['bg-blue-100']} */ ;
            /** @type {__VLS_StyleScopedClasses['text-blue-600']} */ ;
            /** @type {__VLS_StyleScopedClasses['dark:bg-blue-800']} */ ;
            /** @type {__VLS_StyleScopedClasses['dark:text-blue-300']} */ ;
            /** @type {__VLS_StyleScopedClasses['rounded']} */ ;
            /** @type {__VLS_StyleScopedClasses['text-nowrap']} */ ;
        }
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "text-xs text-slate-500 truncate mt-1" },
        });
        /** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
        /** @type {__VLS_StyleScopedClasses['truncate']} */ ;
        /** @type {__VLS_StyleScopedClasses['mt-1']} */ ;
        (profile.server_url);
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ onClick: (() => { }) },
            ...{ class: "flex space-x-1 ml-4" },
        });
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['space-x-1']} */ ;
        /** @type {__VLS_StyleScopedClasses['ml-4']} */ ;
        let __VLS_17;
        /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
        aButton;
        // @ts-ignore
        const __VLS_18 = __VLS_asFunctionalComponent1(__VLS_17, new __VLS_17({
            ...{ 'onClick': {} },
            type: "text",
            size: "small",
        }));
        const __VLS_19 = __VLS_18({
            ...{ 'onClick': {} },
            type: "text",
            size: "small",
        }, ...__VLS_functionalComponentArgsRest(__VLS_18));
        let __VLS_22;
        const __VLS_23 = ({ click: {} },
            { onClick: (...[$event]) => {
                    if (!!(__VLS_ctx.connectionStore.profiles.length === 0))
                        return;
                    __VLS_ctx.editProfile(profile);
                    // @ts-ignore
                    [connectionStore, connectionStore, editProfile,];
                } });
        const { default: __VLS_24 } = __VLS_20.slots;
        let __VLS_25;
        /** @ts-ignore @type {typeof __VLS_components.EditOutlined} */
        EditOutlined;
        // @ts-ignore
        const __VLS_26 = __VLS_asFunctionalComponent1(__VLS_25, new __VLS_25({}));
        const __VLS_27 = __VLS_26({}, ...__VLS_functionalComponentArgsRest(__VLS_26));
        // @ts-ignore
        [];
        var __VLS_20;
        var __VLS_21;
        let __VLS_30;
        /** @ts-ignore @type {typeof __VLS_components.aPopconfirm | typeof __VLS_components.APopconfirm | typeof __VLS_components.aPopconfirm | typeof __VLS_components.APopconfirm} */
        aPopconfirm;
        // @ts-ignore
        const __VLS_31 = __VLS_asFunctionalComponent1(__VLS_30, new __VLS_30({
            ...{ 'onConfirm': {} },
            title: "确定要删除这个连接配置吗？",
        }));
        const __VLS_32 = __VLS_31({
            ...{ 'onConfirm': {} },
            title: "确定要删除这个连接配置吗？",
        }, ...__VLS_functionalComponentArgsRest(__VLS_31));
        let __VLS_35;
        const __VLS_36 = ({ confirm: {} },
            { onConfirm: (...[$event]) => {
                    if (!!(__VLS_ctx.connectionStore.profiles.length === 0))
                        return;
                    __VLS_ctx.connectionStore.deleteProfile(profile.id);
                    // @ts-ignore
                    [connectionStore,];
                } });
        const { default: __VLS_37 } = __VLS_33.slots;
        let __VLS_38;
        /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
        aButton;
        // @ts-ignore
        const __VLS_39 = __VLS_asFunctionalComponent1(__VLS_38, new __VLS_38({
            ...{ 'onClick': {} },
            type: "text",
            danger: true,
            size: "small",
        }));
        const __VLS_40 = __VLS_39({
            ...{ 'onClick': {} },
            type: "text",
            danger: true,
            size: "small",
        }, ...__VLS_functionalComponentArgsRest(__VLS_39));
        let __VLS_43;
        const __VLS_44 = ({ click: {} },
            { onClick: (() => { }) });
        const { default: __VLS_45 } = __VLS_41.slots;
        let __VLS_46;
        /** @ts-ignore @type {typeof __VLS_components.DeleteOutlined} */
        DeleteOutlined;
        // @ts-ignore
        const __VLS_47 = __VLS_asFunctionalComponent1(__VLS_46, new __VLS_46({}));
        const __VLS_48 = __VLS_47({}, ...__VLS_functionalComponentArgsRest(__VLS_47));
        // @ts-ignore
        [];
        var __VLS_41;
        var __VLS_42;
        // @ts-ignore
        [];
        var __VLS_33;
        var __VLS_34;
        // @ts-ignore
        [];
    }
}
if (__VLS_ctx.isEditing || __VLS_ctx.connectionStore.profiles.length === 0) {
    let __VLS_51;
    /** @ts-ignore @type {typeof __VLS_components.aDivider | typeof __VLS_components.ADivider} */
    aDivider;
    // @ts-ignore
    const __VLS_52 = __VLS_asFunctionalComponent1(__VLS_51, new __VLS_51({}));
    const __VLS_53 = __VLS_52({}, ...__VLS_functionalComponentArgsRest(__VLS_52));
}
if (__VLS_ctx.isEditing || __VLS_ctx.connectionStore.profiles.length === 0) {
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({});
    __VLS_asFunctionalElement1(__VLS_intrinsics.h3, __VLS_intrinsics.h3)({
        ...{ class: "text-sm font-medium text-slate-700 dark:text-slate-300 mb-4" },
    });
    /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
    /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-700']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:text-slate-300']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-4']} */ ;
    (__VLS_ctx.editingId ? '编辑连接' : '新建后端连接');
    let __VLS_56;
    /** @ts-ignore @type {typeof __VLS_components.aForm | typeof __VLS_components.AForm | typeof __VLS_components.aForm | typeof __VLS_components.AForm} */
    aForm;
    // @ts-ignore
    const __VLS_57 = __VLS_asFunctionalComponent1(__VLS_56, new __VLS_56({
        model: (__VLS_ctx.formState),
        layout: "vertical",
    }));
    const __VLS_58 = __VLS_57({
        model: (__VLS_ctx.formState),
        layout: "vertical",
    }, ...__VLS_functionalComponentArgsRest(__VLS_57));
    const { default: __VLS_61 } = __VLS_59.slots;
    let __VLS_62;
    /** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
    aFormItem;
    // @ts-ignore
    const __VLS_63 = __VLS_asFunctionalComponent1(__VLS_62, new __VLS_62({
        label: "连接名称 (可选)",
    }));
    const __VLS_64 = __VLS_63({
        label: "连接名称 (可选)",
    }, ...__VLS_functionalComponentArgsRest(__VLS_63));
    const { default: __VLS_67 } = __VLS_65.slots;
    let __VLS_68;
    /** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
    aInput;
    // @ts-ignore
    const __VLS_69 = __VLS_asFunctionalComponent1(__VLS_68, new __VLS_68({
        value: (__VLS_ctx.formState.connection_name),
        placeholder: "例如: Local Dev, Teamserver",
    }));
    const __VLS_70 = __VLS_69({
        value: (__VLS_ctx.formState.connection_name),
        placeholder: "例如: Local Dev, Teamserver",
    }, ...__VLS_functionalComponentArgsRest(__VLS_69));
    // @ts-ignore
    [connectionStore, connectionStore, isEditing, isEditing, editingId, formState, formState,];
    var __VLS_65;
    let __VLS_73;
    /** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
    aFormItem;
    // @ts-ignore
    const __VLS_74 = __VLS_asFunctionalComponent1(__VLS_73, new __VLS_73({
        label: "服务端地址",
        required: true,
    }));
    const __VLS_75 = __VLS_74({
        label: "服务端地址",
        required: true,
    }, ...__VLS_functionalComponentArgsRest(__VLS_74));
    const { default: __VLS_78 } = __VLS_76.slots;
    let __VLS_79;
    /** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
    aInput;
    // @ts-ignore
    const __VLS_80 = __VLS_asFunctionalComponent1(__VLS_79, new __VLS_79({
        value: (__VLS_ctx.formState.server_url),
        placeholder: "例如: http://127.0.0.1:3000",
    }));
    const __VLS_81 = __VLS_80({
        value: (__VLS_ctx.formState.server_url),
        placeholder: "例如: http://127.0.0.1:3000",
    }, ...__VLS_functionalComponentArgsRest(__VLS_80));
    // @ts-ignore
    [formState,];
    var __VLS_76;
    let __VLS_84;
    /** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
    aFormItem;
    // @ts-ignore
    const __VLS_85 = __VLS_asFunctionalComponent1(__VLS_84, new __VLS_84({
        label: "API Token",
        required: true,
    }));
    const __VLS_86 = __VLS_85({
        label: "API Token",
        required: true,
    }, ...__VLS_functionalComponentArgsRest(__VLS_85));
    const { default: __VLS_89 } = __VLS_87.slots;
    let __VLS_90;
    /** @ts-ignore @type {typeof __VLS_components.aInputPassword | typeof __VLS_components.AInputPassword} */
    aInputPassword;
    // @ts-ignore
    const __VLS_91 = __VLS_asFunctionalComponent1(__VLS_90, new __VLS_90({
        value: (__VLS_ctx.formState.api_token),
        placeholder: "输入服务端的 config.toml 中的 api_token",
    }));
    const __VLS_92 = __VLS_91({
        value: (__VLS_ctx.formState.api_token),
        placeholder: "输入服务端的 config.toml 中的 api_token",
    }, ...__VLS_functionalComponentArgsRest(__VLS_91));
    // @ts-ignore
    [formState,];
    var __VLS_87;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "bg-slate-50 dark:bg-[#14161A] p-3 rounded-md mb-4 text-xs text-slate-500 dark:text-slate-400" },
    });
    /** @type {__VLS_StyleScopedClasses['bg-slate-50']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:bg-[#14161A]']} */ ;
    /** @type {__VLS_StyleScopedClasses['p-3']} */ ;
    /** @type {__VLS_StyleScopedClasses['rounded-md']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:text-slate-400']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "mb-1 text-slate-700 dark:text-slate-300 flex items-center space-x-1.5 font-medium" },
    });
    /** @type {__VLS_StyleScopedClasses['mb-1']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-700']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:text-slate-300']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['space-x-1.5']} */ ;
    /** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
    let __VLS_95;
    /** @ts-ignore @type {typeof __VLS_components.InfoCircleOutlined} */
    InfoCircleOutlined;
    // @ts-ignore
    const __VLS_96 = __VLS_asFunctionalComponent1(__VLS_95, new __VLS_95({}));
    const __VLS_97 = __VLS_96({}, ...__VLS_functionalComponentArgsRest(__VLS_96));
    __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({});
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex flex-col space-y-1 mt-2" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
    /** @type {__VLS_StyleScopedClasses['space-y-1']} */ ;
    /** @type {__VLS_StyleScopedClasses['mt-2']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex items-center space-x-2" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['space-x-2']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
        ...{ class: "w-2 h-2 rounded-full" },
        ...{ class: (__VLS_ctx.testStatus.reachability === 'success' ? 'bg-green-500' : __VLS_ctx.testStatus.reachability === 'error' ? 'bg-red-500' : 'bg-slate-300') },
    });
    /** @type {__VLS_StyleScopedClasses['w-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['h-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({});
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex items-center space-x-2" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['space-x-2']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
        ...{ class: "w-2 h-2 rounded-full" },
        ...{ class: (__VLS_ctx.testStatus.auth === 'success' ? 'bg-green-500' : __VLS_ctx.testStatus.auth === 'error' ? 'bg-red-500' : 'bg-slate-300') },
    });
    /** @type {__VLS_StyleScopedClasses['w-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['h-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({});
    if (__VLS_ctx.testMessage) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "mt-2 text-red-500" },
        });
        /** @type {__VLS_StyleScopedClasses['mt-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-red-500']} */ ;
        (__VLS_ctx.testMessage);
    }
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex justify-end gap-2" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['justify-end']} */ ;
    /** @type {__VLS_StyleScopedClasses['gap-2']} */ ;
    if (__VLS_ctx.connectionStore.profiles.length > 0) {
        let __VLS_100;
        /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
        aButton;
        // @ts-ignore
        const __VLS_101 = __VLS_asFunctionalComponent1(__VLS_100, new __VLS_100({
            ...{ 'onClick': {} },
        }));
        const __VLS_102 = __VLS_101({
            ...{ 'onClick': {} },
        }, ...__VLS_functionalComponentArgsRest(__VLS_101));
        let __VLS_105;
        const __VLS_106 = ({ click: {} },
            { onClick: (__VLS_ctx.cancelEdit) });
        const { default: __VLS_107 } = __VLS_103.slots;
        // @ts-ignore
        [connectionStore, testStatus, testStatus, testStatus, testStatus, testMessage, testMessage, cancelEdit,];
        var __VLS_103;
        var __VLS_104;
    }
    let __VLS_108;
    /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
    aButton;
    // @ts-ignore
    const __VLS_109 = __VLS_asFunctionalComponent1(__VLS_108, new __VLS_108({
        ...{ 'onClick': {} },
        loading: (__VLS_ctx.isTesting),
    }));
    const __VLS_110 = __VLS_109({
        ...{ 'onClick': {} },
        loading: (__VLS_ctx.isTesting),
    }, ...__VLS_functionalComponentArgsRest(__VLS_109));
    let __VLS_113;
    const __VLS_114 = ({ click: {} },
        { onClick: (__VLS_ctx.runTest) });
    const { default: __VLS_115 } = __VLS_111.slots;
    // @ts-ignore
    [isTesting, runTest,];
    var __VLS_111;
    var __VLS_112;
    let __VLS_116;
    /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
    aButton;
    // @ts-ignore
    const __VLS_117 = __VLS_asFunctionalComponent1(__VLS_116, new __VLS_116({
        ...{ 'onClick': {} },
        type: "primary",
        disabled: (!__VLS_ctx.formState.server_url || !__VLS_ctx.formState.api_token),
    }));
    const __VLS_118 = __VLS_117({
        ...{ 'onClick': {} },
        type: "primary",
        disabled: (!__VLS_ctx.formState.server_url || !__VLS_ctx.formState.api_token),
    }, ...__VLS_functionalComponentArgsRest(__VLS_117));
    let __VLS_121;
    const __VLS_122 = ({ click: {} },
        { onClick: (__VLS_ctx.saveConnection) });
    const { default: __VLS_123 } = __VLS_119.slots;
    // @ts-ignore
    [formState, formState, saveConnection,];
    var __VLS_119;
    var __VLS_120;
    // @ts-ignore
    [];
    var __VLS_59;
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
