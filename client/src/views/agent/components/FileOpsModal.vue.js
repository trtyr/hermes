/// <reference types="../../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, reactive } from 'vue';
import { message } from 'ant-design-vue';
import { uploadFile, downloadFile } from '@/api/agent';
const props = defineProps();
const emit = defineEmits(['update:visible']);
const activeTab = ref('upload');
const uploading = ref(false);
const downloading = ref(false);
const uploadForm = reactive({
    remotePath: '',
    selectedFile: null,
    fileBase64: '',
});
const downloadForm = reactive({
    remotePath: '',
});
function onFileSelect(e) {
    const target = e.target;
    const file = target.files?.[0];
    if (!file)
        return;
    if (file.size > 10 * 1024 * 1024) {
        message.warning('文件大小不能超过 10MB');
        return;
    }
    uploadForm.selectedFile = file;
    const reader = new FileReader();
    reader.onload = () => {
        const result = reader.result;
        uploadForm.fileBase64 = result.split(',')[1] || '';
    };
    reader.readAsDataURL(file);
}
async function doUpload() {
    if (!props.agent || !uploadForm.selectedFile || !uploadForm.remotePath)
        return;
    uploading.value = true;
    try {
        const res = await uploadFile(props.agent.agent_id, uploadForm.remotePath, uploadForm.fileBase64);
        if (res.success) {
            message.success(`上传任务已下发 (task: ${res.task_id || '-'})`);
            uploadForm.remotePath = '';
            uploadForm.selectedFile = null;
            uploadForm.fileBase64 = '';
        }
        else {
            message.error(res.detail || '上传失败');
        }
    }
    catch (e) {
        message.error(e.message);
    }
    finally {
        uploading.value = false;
    }
}
async function doDownload() {
    if (!props.agent || !downloadForm.remotePath)
        return;
    downloading.value = true;
    try {
        const res = await downloadFile(props.agent.agent_id, downloadForm.remotePath);
        if (res.success) {
            message.success(`下载任务已下发 (task: ${res.task_id || '-'})，结果将通过任务系统返回`);
            downloadForm.remotePath = '';
        }
        else {
            message.error(res.detail || '下载失败');
        }
    }
    catch (e) {
        message.error(e.message);
    }
    finally {
        downloading.value = false;
    }
}
function formatSize(bytes) {
    if (bytes < 1024)
        return `${bytes} B`;
    if (bytes < 1024 * 1024)
        return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
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
    ...{ 'onUpdate:visible': {} },
    visible: (__VLS_ctx.visible),
    title: "文件管理",
    footer: (null),
    width: "520",
    destroyOnClose: true,
}));
const __VLS_2 = __VLS_1({
    ...{ 'onUpdate:visible': {} },
    visible: (__VLS_ctx.visible),
    title: "文件管理",
    footer: (null),
    width: "520",
    destroyOnClose: true,
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
let __VLS_5;
const __VLS_6 = ({ 'update:visible': {} },
    { 'onUpdate:visible': (...[$event]) => {
            __VLS_ctx.$emit('update:visible', $event);
            // @ts-ignore
            [visible, $emit,];
        } });
var __VLS_7 = {};
const { default: __VLS_8 } = __VLS_3.slots;
let __VLS_9;
/** @ts-ignore @type {typeof __VLS_components.aTabs | typeof __VLS_components.ATabs | typeof __VLS_components.aTabs | typeof __VLS_components.ATabs} */
aTabs;
// @ts-ignore
const __VLS_10 = __VLS_asFunctionalComponent1(__VLS_9, new __VLS_9({
    activeKey: (__VLS_ctx.activeTab),
}));
const __VLS_11 = __VLS_10({
    activeKey: (__VLS_ctx.activeTab),
}, ...__VLS_functionalComponentArgsRest(__VLS_10));
const { default: __VLS_14 } = __VLS_12.slots;
let __VLS_15;
/** @ts-ignore @type {typeof __VLS_components.aTabPane | typeof __VLS_components.ATabPane | typeof __VLS_components.aTabPane | typeof __VLS_components.ATabPane} */
aTabPane;
// @ts-ignore
const __VLS_16 = __VLS_asFunctionalComponent1(__VLS_15, new __VLS_15({
    key: "upload",
    tab: "上传文件",
}));
const __VLS_17 = __VLS_16({
    key: "upload",
    tab: "上传文件",
}, ...__VLS_functionalComponentArgsRest(__VLS_16));
const { default: __VLS_20 } = __VLS_18.slots;
let __VLS_21;
/** @ts-ignore @type {typeof __VLS_components.aForm | typeof __VLS_components.AForm | typeof __VLS_components.aForm | typeof __VLS_components.AForm} */
aForm;
// @ts-ignore
const __VLS_22 = __VLS_asFunctionalComponent1(__VLS_21, new __VLS_21({
    layout: "vertical",
    ...{ class: "mt-4" },
}));
const __VLS_23 = __VLS_22({
    layout: "vertical",
    ...{ class: "mt-4" },
}, ...__VLS_functionalComponentArgsRest(__VLS_22));
/** @type {__VLS_StyleScopedClasses['mt-4']} */ ;
const { default: __VLS_26 } = __VLS_24.slots;
let __VLS_27;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_28 = __VLS_asFunctionalComponent1(__VLS_27, new __VLS_27({
    label: "远程路径",
}));
const __VLS_29 = __VLS_28({
    label: "远程路径",
}, ...__VLS_functionalComponentArgsRest(__VLS_28));
const { default: __VLS_32 } = __VLS_30.slots;
let __VLS_33;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_34 = __VLS_asFunctionalComponent1(__VLS_33, new __VLS_33({
    value: (__VLS_ctx.uploadForm.remotePath),
    placeholder: "\u4f8b\u5982\u003a\u0020\u0043\u003a\u005c\u0055\u0073\u0065\u0072\u0073\u005c\u0074\u0061\u0072\u0067\u0065\u0074\u005c\u0070\u0061\u0079\u006c\u006f\u0061\u0064\u002e\u0065\u0078\u0065\u0020\u6216\u0020\u002f\u0074\u006d\u0070\u002f\u0070\u0061\u0079\u006c\u006f\u0061\u0064",
}));
const __VLS_35 = __VLS_34({
    value: (__VLS_ctx.uploadForm.remotePath),
    placeholder: "\u4f8b\u5982\u003a\u0020\u0043\u003a\u005c\u0055\u0073\u0065\u0072\u0073\u005c\u0074\u0061\u0072\u0067\u0065\u0074\u005c\u0070\u0061\u0079\u006c\u006f\u0061\u0064\u002e\u0065\u0078\u0065\u0020\u6216\u0020\u002f\u0074\u006d\u0070\u002f\u0070\u0061\u0079\u006c\u006f\u0061\u0064",
}, ...__VLS_functionalComponentArgsRest(__VLS_34));
// @ts-ignore
[activeTab, uploadForm,];
var __VLS_30;
let __VLS_38;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_39 = __VLS_asFunctionalComponent1(__VLS_38, new __VLS_38({
    label: "选择文件",
}));
const __VLS_40 = __VLS_39({
    label: "选择文件",
}, ...__VLS_functionalComponentArgsRest(__VLS_39));
const { default: __VLS_43 } = __VLS_41.slots;
__VLS_asFunctionalElement1(__VLS_intrinsics.input)({
    ...{ onChange: (__VLS_ctx.onFileSelect) },
    type: "file",
    ...{ class: "\u0062\u006c\u006f\u0063\u006b\u0020\u0077\u002d\u0066\u0075\u006c\u006c\u0020\u0074\u0065\u0078\u0074\u002d\u0073\u006d\u0020\u0074\u0065\u0078\u0074\u002d\u0073\u006c\u0061\u0074\u0065\u002d\u0035\u0030\u0030\u0020\u0064\u0061\u0072\u006b\u003a\u0074\u0065\u0078\u0074\u002d\u005b\u0076\u0061\u0072\u0028\u002d\u002d\u0074\u0065\u0078\u0074\u002d\u0073\u0065\u0063\u006f\u006e\u0064\u0061\u0072\u0079\u0029\u005d\u000a\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0066\u0069\u006c\u0065\u003a\u006d\u0072\u002d\u0033\u0020\u0066\u0069\u006c\u0065\u003a\u0070\u0079\u002d\u0031\u002e\u0035\u0020\u0066\u0069\u006c\u0065\u003a\u0070\u0078\u002d\u0033\u000a\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0066\u0069\u006c\u0065\u003a\u0072\u006f\u0075\u006e\u0064\u0065\u0064\u002d\u006d\u0064\u0020\u0066\u0069\u006c\u0065\u003a\u0062\u006f\u0072\u0064\u0065\u0072\u002d\u0030\u000a\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0066\u0069\u006c\u0065\u003a\u0074\u0065\u0078\u0074\u002d\u0073\u006d\u0020\u0066\u0069\u006c\u0065\u003a\u0066\u006f\u006e\u0074\u002d\u006d\u0065\u0064\u0069\u0075\u006d\u000a\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0066\u0069\u006c\u0065\u003a\u0062\u0067\u002d\u0062\u006c\u0075\u0065\u002d\u0035\u0030\u0020\u0066\u0069\u006c\u0065\u003a\u0074\u0065\u0078\u0074\u002d\u0062\u006c\u0075\u0065\u002d\u0036\u0030\u0030\u000a\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0064\u0061\u0072\u006b\u003a\u0066\u0069\u006c\u0065\u003a\u0062\u0067\u002d\u0062\u006c\u0075\u0065\u002d\u0039\u0030\u0030\u002f\u0032\u0030\u0020\u0064\u0061\u0072\u006b\u003a\u0066\u0069\u006c\u0065\u003a\u0074\u0065\u0078\u0074\u002d\u0062\u006c\u0075\u0065\u002d\u0034\u0030\u0030\u000a\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0068\u006f\u0076\u0065\u0072\u003a\u0066\u0069\u006c\u0065\u003a\u0062\u0067\u002d\u0062\u006c\u0075\u0065\u002d\u0031\u0030\u0030\u0020\u0064\u0061\u0072\u006b\u003a\u0068\u006f\u0076\u0065\u0072\u003a\u0066\u0069\u006c\u0065\u003a\u0062\u0067\u002d\u0062\u006c\u0075\u0065\u002d\u0039\u0030\u0030\u002f\u0033\u0030\u000a\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0020\u0063\u0075\u0072\u0073\u006f\u0072\u002d\u0070\u006f\u0069\u006e\u0074\u0065\u0072" },
});
/** @type {__VLS_StyleScopedClasses['block']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
/** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-[var(--text-secondary)]
']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['file:mr-3']} */ ;
/** @type {__VLS_StyleScopedClasses['file:py-1.5']} */ ;
/** @type {__VLS_StyleScopedClasses['file:px-3
']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['file:rounded-md']} */ ;
/** @type {__VLS_StyleScopedClasses['file:border-0
']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['file:text-sm']} */ ;
/** @type {__VLS_StyleScopedClasses['file:font-medium
']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['file:bg-blue-50']} */ ;
/** @type {__VLS_StyleScopedClasses['file:text-blue-600
']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:file:bg-blue-900/20']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:file:text-blue-400
']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['hover:file:bg-blue-100']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:hover:file:bg-blue-900/30
']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['']} */ ;
/** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
// @ts-ignore
[onFileSelect,];
var __VLS_41;
if (__VLS_ctx.uploadForm.selectedFile) {
    let __VLS_44;
    /** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
    aFormItem;
    // @ts-ignore
    const __VLS_45 = __VLS_asFunctionalComponent1(__VLS_44, new __VLS_44({
        label: ('已选择: ' + __VLS_ctx.uploadForm.selectedFile.name + ' (' + __VLS_ctx.formatSize(__VLS_ctx.uploadForm.selectedFile.size) + ')'),
    }));
    const __VLS_46 = __VLS_45({
        label: ('已选择: ' + __VLS_ctx.uploadForm.selectedFile.name + ' (' + __VLS_ctx.formatSize(__VLS_ctx.uploadForm.selectedFile.size) + ')'),
    }, ...__VLS_functionalComponentArgsRest(__VLS_45));
}
let __VLS_49;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_50 = __VLS_asFunctionalComponent1(__VLS_49, new __VLS_49({}));
const __VLS_51 = __VLS_50({}, ...__VLS_functionalComponentArgsRest(__VLS_50));
const { default: __VLS_54 } = __VLS_52.slots;
let __VLS_55;
/** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
aButton;
// @ts-ignore
const __VLS_56 = __VLS_asFunctionalComponent1(__VLS_55, new __VLS_55({
    ...{ 'onClick': {} },
    type: "primary",
    loading: (__VLS_ctx.uploading),
    disabled: (!__VLS_ctx.uploadForm.selectedFile || !__VLS_ctx.uploadForm.remotePath),
}));
const __VLS_57 = __VLS_56({
    ...{ 'onClick': {} },
    type: "primary",
    loading: (__VLS_ctx.uploading),
    disabled: (!__VLS_ctx.uploadForm.selectedFile || !__VLS_ctx.uploadForm.remotePath),
}, ...__VLS_functionalComponentArgsRest(__VLS_56));
let __VLS_60;
const __VLS_61 = ({ click: {} },
    { onClick: (__VLS_ctx.doUpload) });
const { default: __VLS_62 } = __VLS_58.slots;
// @ts-ignore
[uploadForm, uploadForm, uploadForm, uploadForm, uploadForm, formatSize, uploading, doUpload,];
var __VLS_58;
var __VLS_59;
// @ts-ignore
[];
var __VLS_52;
// @ts-ignore
[];
var __VLS_24;
// @ts-ignore
[];
var __VLS_18;
let __VLS_63;
/** @ts-ignore @type {typeof __VLS_components.aTabPane | typeof __VLS_components.ATabPane | typeof __VLS_components.aTabPane | typeof __VLS_components.ATabPane} */
aTabPane;
// @ts-ignore
const __VLS_64 = __VLS_asFunctionalComponent1(__VLS_63, new __VLS_63({
    key: "download",
    tab: "下载文件",
}));
const __VLS_65 = __VLS_64({
    key: "download",
    tab: "下载文件",
}, ...__VLS_functionalComponentArgsRest(__VLS_64));
const { default: __VLS_68 } = __VLS_66.slots;
let __VLS_69;
/** @ts-ignore @type {typeof __VLS_components.aForm | typeof __VLS_components.AForm | typeof __VLS_components.aForm | typeof __VLS_components.AForm} */
aForm;
// @ts-ignore
const __VLS_70 = __VLS_asFunctionalComponent1(__VLS_69, new __VLS_69({
    layout: "vertical",
    ...{ class: "mt-4" },
}));
const __VLS_71 = __VLS_70({
    layout: "vertical",
    ...{ class: "mt-4" },
}, ...__VLS_functionalComponentArgsRest(__VLS_70));
/** @type {__VLS_StyleScopedClasses['mt-4']} */ ;
const { default: __VLS_74 } = __VLS_72.slots;
let __VLS_75;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_76 = __VLS_asFunctionalComponent1(__VLS_75, new __VLS_75({
    label: "远程文件路径",
}));
const __VLS_77 = __VLS_76({
    label: "远程文件路径",
}, ...__VLS_functionalComponentArgsRest(__VLS_76));
const { default: __VLS_80 } = __VLS_78.slots;
let __VLS_81;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_82 = __VLS_asFunctionalComponent1(__VLS_81, new __VLS_81({
    value: (__VLS_ctx.downloadForm.remotePath),
    placeholder: "\u4f8b\u5982\u003a\u0020\u0043\u003a\u005c\u0055\u0073\u0065\u0072\u0073\u005c\u0074\u0061\u0072\u0067\u0065\u0074\u005c\u0064\u006f\u0063\u0075\u006d\u0065\u006e\u0074\u0073\u005c\u0073\u0065\u0063\u0072\u0065\u0074\u002e\u0064\u006f\u0063\u0078\u0020\u6216\u0020\u002f\u0065\u0074\u0063\u002f\u0070\u0061\u0073\u0073\u0077\u0064",
}));
const __VLS_83 = __VLS_82({
    value: (__VLS_ctx.downloadForm.remotePath),
    placeholder: "\u4f8b\u5982\u003a\u0020\u0043\u003a\u005c\u0055\u0073\u0065\u0072\u0073\u005c\u0074\u0061\u0072\u0067\u0065\u0074\u005c\u0064\u006f\u0063\u0075\u006d\u0065\u006e\u0074\u0073\u005c\u0073\u0065\u0063\u0072\u0065\u0074\u002e\u0064\u006f\u0063\u0078\u0020\u6216\u0020\u002f\u0065\u0074\u0063\u002f\u0070\u0061\u0073\u0073\u0077\u0064",
}, ...__VLS_functionalComponentArgsRest(__VLS_82));
// @ts-ignore
[downloadForm,];
var __VLS_78;
let __VLS_86;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_87 = __VLS_asFunctionalComponent1(__VLS_86, new __VLS_86({}));
const __VLS_88 = __VLS_87({}, ...__VLS_functionalComponentArgsRest(__VLS_87));
const { default: __VLS_91 } = __VLS_89.slots;
let __VLS_92;
/** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
aButton;
// @ts-ignore
const __VLS_93 = __VLS_asFunctionalComponent1(__VLS_92, new __VLS_92({
    ...{ 'onClick': {} },
    type: "primary",
    loading: (__VLS_ctx.downloading),
    disabled: (!__VLS_ctx.downloadForm.remotePath),
}));
const __VLS_94 = __VLS_93({
    ...{ 'onClick': {} },
    type: "primary",
    loading: (__VLS_ctx.downloading),
    disabled: (!__VLS_ctx.downloadForm.remotePath),
}, ...__VLS_functionalComponentArgsRest(__VLS_93));
let __VLS_97;
const __VLS_98 = ({ click: {} },
    { onClick: (__VLS_ctx.doDownload) });
const { default: __VLS_99 } = __VLS_95.slots;
// @ts-ignore
[downloadForm, downloading, doDownload,];
var __VLS_95;
var __VLS_96;
// @ts-ignore
[];
var __VLS_89;
// @ts-ignore
[];
var __VLS_72;
// @ts-ignore
[];
var __VLS_66;
// @ts-ignore
[];
var __VLS_12;
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
