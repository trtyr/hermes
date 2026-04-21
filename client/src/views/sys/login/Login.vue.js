/// <reference types="../../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import loginBoxBg from '@/assets/login-box-bg.svg';
import { reactive, ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { UserOutlined, LockOutlined, GlobalOutlined, ExclamationCircleOutlined, } from '@ant-design/icons-vue';
import { useAppStore } from '@/store/app';
import { useConnectionStore } from '@/store/connection';
import { loginToBackend } from '@/api/connection';
const router = useRouter();
const loading = ref(false);
const errorMsg = ref('');
const appStore = useAppStore();
const connectionStore = useConnectionStore();
const formState = reactive({
    serverUrl: '',
    username: '',
    password: '',
});
const toggleDark = () => {
    appStore.toggleTheme();
};
onMounted(() => {
    // Auto-dark for login page
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        if (!appStore.isDark) {
            appStore.toggleTheme();
        }
    }
    // Pre-fill from last active profile
    if (connectionStore.activeProfile) {
        formState.serverUrl = connectionStore.activeProfile.server_url;
    }
});
const handleFinish = async () => {
    errorMsg.value = '';
    loading.value = true;
    try {
        const result = await loginToBackend(formState.serverUrl, formState.username, formState.password);
        if (!result.success) {
            errorMsg.value = result.error || '连接失败';
            loading.value = false;
            return;
        }
        // Save or update connection profile with session token
        const normalizedUrl = connectionStore.normalizeUrl(formState.serverUrl);
        const existing = connectionStore.profiles.find((p) => p.server_url === normalizedUrl);
        if (existing) {
            connectionStore.updateProfile(existing.id, {
                api_token: result.session_token || '',
                connection_name: formState.username,
            });
            connectionStore.setActiveProfile(existing.id);
        }
        else {
            const profile = connectionStore.addProfile({
                connection_name: formState.username,
                server_url: normalizedUrl,
                api_token: result.session_token || '',
            });
            connectionStore.setActiveProfile(profile.id);
        }
        router.push('/dashboard');
    }
    catch {
        errorMsg.value = '连接异常，请检查服务器地址';
    }
    finally {
        loading.value = false;
    }
};
const __VLS_ctx = {
    ...{},
    ...{},
};
let __VLS_components;
let __VLS_intrinsics;
let __VLS_directives;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "relative w-full h-screen overflow-hidden bg-white dark:bg-[var(--bg-page)] flex justify-center" },
});
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['h-screen']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-page)]']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "absolute top-4 right-4 flex items-center space-x-4 z-50" },
});
/** @type {__VLS_StyleScopedClasses['absolute']} */ ;
/** @type {__VLS_StyleScopedClasses['top-4']} */ ;
/** @type {__VLS_StyleScopedClasses['right-4']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['space-x-4']} */ ;
/** @type {__VLS_StyleScopedClasses['z-50']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ onClick: (__VLS_ctx.toggleDark) },
    ...{ class: "cursor-pointer text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white transition-colors" },
});
/** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
/** @type {__VLS_StyleScopedClasses['text-gray-500']} */ ;
/** @type {__VLS_StyleScopedClasses['hover:text-gray-900']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-gray-400']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:hover:text-white']} */ ;
/** @type {__VLS_StyleScopedClasses['transition-colors']} */ ;
if (!__VLS_ctx.appStore.isDark) {
    __VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
        ...{ class: "w-5 h-5" },
        fill: "none",
        stroke: "currentColor",
        viewBox: "0 0 24 24",
    });
    /** @type {__VLS_StyleScopedClasses['w-5']} */ ;
    /** @type {__VLS_StyleScopedClasses['h-5']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.path)({
        'stroke-linecap': "round",
        'stroke-linejoin': "round",
        'stroke-width': "2",
        d: "M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z",
    });
}
else {
    __VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
        ...{ class: "w-5 h-5" },
        fill: "none",
        stroke: "currentColor",
        viewBox: "0 0 24 24",
    });
    /** @type {__VLS_StyleScopedClasses['w-5']} */ ;
    /** @type {__VLS_StyleScopedClasses['h-5']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.path)({
        'stroke-linecap': "round",
        'stroke-linejoin': "round",
        'stroke-width': "2",
        d: "M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z",
    });
}
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex w-full h-full max-w-[1200px] shadow-2xl overflow-hidden rounded-none md:rounded-2xl md:h-[620px] md:my-auto md:w-4/5 lg:w-[1000px] bg-white dark:bg-[var(--bg-card)]" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['h-full']} */ ;
/** @type {__VLS_StyleScopedClasses['max-w-[1200px]']} */ ;
/** @type {__VLS_StyleScopedClasses['shadow-2xl']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-none']} */ ;
/** @type {__VLS_StyleScopedClasses['md:rounded-2xl']} */ ;
/** @type {__VLS_StyleScopedClasses['md:h-[620px]']} */ ;
/** @type {__VLS_StyleScopedClasses['md:my-auto']} */ ;
/** @type {__VLS_StyleScopedClasses['md:w-4/5']} */ ;
/** @type {__VLS_StyleScopedClasses['lg:w-[1000px]']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-card)]']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "hidden md:flex flex-col justify-center items-center w-1/2 bg-blue-600 relative overflow-hidden text-white p-10" },
});
/** @type {__VLS_StyleScopedClasses['hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['md:flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['w-1/2']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-blue-600']} */ ;
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['text-white']} */ ;
/** @type {__VLS_StyleScopedClasses['p-10']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "absolute top-[-10%] left-[-10%] w-64 h-64 bg-blue-500 rounded-full mix-blend-multiply filter blur-2xl opacity-70 animate-blob" },
});
/** @type {__VLS_StyleScopedClasses['absolute']} */ ;
/** @type {__VLS_StyleScopedClasses['top-[-10%]']} */ ;
/** @type {__VLS_StyleScopedClasses['left-[-10%]']} */ ;
/** @type {__VLS_StyleScopedClasses['w-64']} */ ;
/** @type {__VLS_StyleScopedClasses['h-64']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-blue-500']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
/** @type {__VLS_StyleScopedClasses['mix-blend-multiply']} */ ;
/** @type {__VLS_StyleScopedClasses['filter']} */ ;
/** @type {__VLS_StyleScopedClasses['blur-2xl']} */ ;
/** @type {__VLS_StyleScopedClasses['opacity-70']} */ ;
/** @type {__VLS_StyleScopedClasses['animate-blob']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "absolute bottom-[-10%] right-[-10%] w-64 h-64 bg-indigo-500 rounded-full mix-blend-multiply filter blur-2xl opacity-70 animate-blob animation-delay-2000" },
});
/** @type {__VLS_StyleScopedClasses['absolute']} */ ;
/** @type {__VLS_StyleScopedClasses['bottom-[-10%]']} */ ;
/** @type {__VLS_StyleScopedClasses['right-[-10%]']} */ ;
/** @type {__VLS_StyleScopedClasses['w-64']} */ ;
/** @type {__VLS_StyleScopedClasses['h-64']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-indigo-500']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
/** @type {__VLS_StyleScopedClasses['mix-blend-multiply']} */ ;
/** @type {__VLS_StyleScopedClasses['filter']} */ ;
/** @type {__VLS_StyleScopedClasses['blur-2xl']} */ ;
/** @type {__VLS_StyleScopedClasses['opacity-70']} */ ;
/** @type {__VLS_StyleScopedClasses['animate-blob']} */ ;
/** @type {__VLS_StyleScopedClasses['animation-delay-2000']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "relative z-10 w-full" },
});
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
/** @type {__VLS_StyleScopedClasses['z-10']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex items-center space-x-3 mb-6" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['space-x-3']} */ ;
/** @type {__VLS_StyleScopedClasses['mb-6']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    ...{ class: "w-12 h-12 text-white" },
    fill: "none",
    stroke: "currentColor",
    viewBox: "0 0 24 24",
});
/** @type {__VLS_StyleScopedClasses['w-12']} */ ;
/** @type {__VLS_StyleScopedClasses['h-12']} */ ;
/** @type {__VLS_StyleScopedClasses['text-white']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.path)({
    'stroke-linecap': "round",
    'stroke-linejoin': "round",
    'stroke-width': "2",
    d: "M13 10V3L4 14h7v7l9-11h-7z",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "text-4xl font-bold tracking-wider" },
});
/** @type {__VLS_StyleScopedClasses['text-4xl']} */ ;
/** @type {__VLS_StyleScopedClasses['font-bold']} */ ;
/** @type {__VLS_StyleScopedClasses['tracking-wider']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "text-lg text-blue-100 mt-4 leading-relaxed max-w-sm" },
});
/** @type {__VLS_StyleScopedClasses['text-lg']} */ ;
/** @type {__VLS_StyleScopedClasses['text-blue-100']} */ ;
/** @type {__VLS_StyleScopedClasses['mt-4']} */ ;
/** @type {__VLS_StyleScopedClasses['leading-relaxed']} */ ;
/** @type {__VLS_StyleScopedClasses['max-w-sm']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.br)({});
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "relative z-10 mt-12 w-full flex justify-center" },
});
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
/** @type {__VLS_StyleScopedClasses['z-10']} */ ;
/** @type {__VLS_StyleScopedClasses['mt-12']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.img)({
    src: (__VLS_ctx.loginBoxBg),
    ...{ class: "w-4/5 object-contain opacity-90" },
    alt: "Login Illustration",
});
/** @type {__VLS_StyleScopedClasses['w-4/5']} */ ;
/** @type {__VLS_StyleScopedClasses['object-contain']} */ ;
/** @type {__VLS_StyleScopedClasses['opacity-90']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "w-full md:w-1/2 flex flex-col justify-center p-8 lg:p-14 bg-white dark:bg-[var(--bg-card)] transition-colors duration-300" },
});
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['md:w-1/2']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
/** @type {__VLS_StyleScopedClasses['p-8']} */ ;
/** @type {__VLS_StyleScopedClasses['lg:p-14']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-card)]']} */ ;
/** @type {__VLS_StyleScopedClasses['transition-colors']} */ ;
/** @type {__VLS_StyleScopedClasses['duration-300']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "w-full max-w-md mx-auto" },
});
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['max-w-md']} */ ;
/** @type {__VLS_StyleScopedClasses['mx-auto']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex items-center space-x-3 mb-8 md:hidden justify-center text-gray-900 dark:text-white" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['space-x-3']} */ ;
/** @type {__VLS_StyleScopedClasses['mb-8']} */ ;
/** @type {__VLS_StyleScopedClasses['md:hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
/** @type {__VLS_StyleScopedClasses['text-gray-900']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-white']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    ...{ class: "w-10 h-10 text-blue-600 dark:text-blue-400" },
    fill: "none",
    stroke: "currentColor",
    viewBox: "0 0 24 24",
});
/** @type {__VLS_StyleScopedClasses['w-10']} */ ;
/** @type {__VLS_StyleScopedClasses['h-10']} */ ;
/** @type {__VLS_StyleScopedClasses['text-blue-600']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-blue-400']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.path)({
    'stroke-linecap': "round",
    'stroke-linejoin': "round",
    'stroke-width': "2",
    d: "M13 10V3L4 14h7v7l9-11h-7z",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "text-3xl font-bold" },
});
/** @type {__VLS_StyleScopedClasses['text-3xl']} */ ;
/** @type {__VLS_StyleScopedClasses['font-bold']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.h2, __VLS_intrinsics.h2)({
    ...{ class: "text-2xl font-semibold text-gray-800 dark:text-gray-100 mb-2" },
});
/** @type {__VLS_StyleScopedClasses['text-2xl']} */ ;
/** @type {__VLS_StyleScopedClasses['font-semibold']} */ ;
/** @type {__VLS_StyleScopedClasses['text-gray-800']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-gray-100']} */ ;
/** @type {__VLS_StyleScopedClasses['mb-2']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.p, __VLS_intrinsics.p)({
    ...{ class: "text-sm text-gray-500 dark:text-gray-400 mb-8" },
});
/** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
/** @type {__VLS_StyleScopedClasses['text-gray-500']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-gray-400']} */ ;
/** @type {__VLS_StyleScopedClasses['mb-8']} */ ;
let __VLS_0;
/** @ts-ignore @type {typeof __VLS_components.aForm | typeof __VLS_components.AForm | typeof __VLS_components.aForm | typeof __VLS_components.AForm} */
aForm;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    ...{ 'onFinish': {} },
    model: (__VLS_ctx.formState),
    layout: "vertical",
    ...{ class: "w-full" },
}));
const __VLS_2 = __VLS_1({
    ...{ 'onFinish': {} },
    model: (__VLS_ctx.formState),
    layout: "vertical",
    ...{ class: "w-full" },
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
let __VLS_5;
const __VLS_6 = ({ finish: {} },
    { onFinish: (__VLS_ctx.handleFinish) });
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
const { default: __VLS_7 } = __VLS_3.slots;
let __VLS_8;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_9 = __VLS_asFunctionalComponent1(__VLS_8, new __VLS_8({
    label: "服务器地址",
    name: "serverUrl",
    rules: ([{ required: true, message: '请输入服务器地址' }]),
}));
const __VLS_10 = __VLS_9({
    label: "服务器地址",
    name: "serverUrl",
    rules: ([{ required: true, message: '请输入服务器地址' }]),
}, ...__VLS_functionalComponentArgsRest(__VLS_9));
const { default: __VLS_13 } = __VLS_11.slots;
let __VLS_14;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput | typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_15 = __VLS_asFunctionalComponent1(__VLS_14, new __VLS_14({
    value: (__VLS_ctx.formState.serverUrl),
    size: "large",
    placeholder: "127.0.0.1:3000",
}));
const __VLS_16 = __VLS_15({
    value: (__VLS_ctx.formState.serverUrl),
    size: "large",
    placeholder: "127.0.0.1:3000",
}, ...__VLS_functionalComponentArgsRest(__VLS_15));
const { default: __VLS_19 } = __VLS_17.slots;
{
    const { prefix: __VLS_20 } = __VLS_17.slots;
    let __VLS_21;
    /** @ts-ignore @type {typeof __VLS_components.GlobalOutlined} */
    GlobalOutlined;
    // @ts-ignore
    const __VLS_22 = __VLS_asFunctionalComponent1(__VLS_21, new __VLS_21({
        ...{ class: "text-gray-400" },
    }));
    const __VLS_23 = __VLS_22({
        ...{ class: "text-gray-400" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_22));
    /** @type {__VLS_StyleScopedClasses['text-gray-400']} */ ;
    // @ts-ignore
    [toggleDark, appStore, loginBoxBg, formState, formState, handleFinish,];
}
// @ts-ignore
[];
var __VLS_17;
// @ts-ignore
[];
var __VLS_11;
let __VLS_26;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_27 = __VLS_asFunctionalComponent1(__VLS_26, new __VLS_26({
    label: "用户名",
    name: "username",
    rules: ([{ required: true, message: '请输入用户名' }]),
}));
const __VLS_28 = __VLS_27({
    label: "用户名",
    name: "username",
    rules: ([{ required: true, message: '请输入用户名' }]),
}, ...__VLS_functionalComponentArgsRest(__VLS_27));
const { default: __VLS_31 } = __VLS_29.slots;
let __VLS_32;
/** @ts-ignore @type {typeof __VLS_components.aInput | typeof __VLS_components.AInput | typeof __VLS_components.aInput | typeof __VLS_components.AInput} */
aInput;
// @ts-ignore
const __VLS_33 = __VLS_asFunctionalComponent1(__VLS_32, new __VLS_32({
    value: (__VLS_ctx.formState.username),
    size: "large",
    placeholder: "admin",
}));
const __VLS_34 = __VLS_33({
    value: (__VLS_ctx.formState.username),
    size: "large",
    placeholder: "admin",
}, ...__VLS_functionalComponentArgsRest(__VLS_33));
const { default: __VLS_37 } = __VLS_35.slots;
{
    const { prefix: __VLS_38 } = __VLS_35.slots;
    let __VLS_39;
    /** @ts-ignore @type {typeof __VLS_components.UserOutlined} */
    UserOutlined;
    // @ts-ignore
    const __VLS_40 = __VLS_asFunctionalComponent1(__VLS_39, new __VLS_39({
        ...{ class: "text-gray-400" },
    }));
    const __VLS_41 = __VLS_40({
        ...{ class: "text-gray-400" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_40));
    /** @type {__VLS_StyleScopedClasses['text-gray-400']} */ ;
    // @ts-ignore
    [formState,];
}
// @ts-ignore
[];
var __VLS_35;
// @ts-ignore
[];
var __VLS_29;
let __VLS_44;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_45 = __VLS_asFunctionalComponent1(__VLS_44, new __VLS_44({
    label: "密码",
    name: "password",
    rules: ([{ required: true, message: '请输入密码' }]),
}));
const __VLS_46 = __VLS_45({
    label: "密码",
    name: "password",
    rules: ([{ required: true, message: '请输入密码' }]),
}, ...__VLS_functionalComponentArgsRest(__VLS_45));
const { default: __VLS_49 } = __VLS_47.slots;
let __VLS_50;
/** @ts-ignore @type {typeof __VLS_components.aInputPassword | typeof __VLS_components.AInputPassword | typeof __VLS_components.aInputPassword | typeof __VLS_components.AInputPassword} */
aInputPassword;
// @ts-ignore
const __VLS_51 = __VLS_asFunctionalComponent1(__VLS_50, new __VLS_50({
    ...{ 'onPressEnter': {} },
    value: (__VLS_ctx.formState.password),
    size: "large",
    placeholder: "••••••",
}));
const __VLS_52 = __VLS_51({
    ...{ 'onPressEnter': {} },
    value: (__VLS_ctx.formState.password),
    size: "large",
    placeholder: "••••••",
}, ...__VLS_functionalComponentArgsRest(__VLS_51));
let __VLS_55;
const __VLS_56 = ({ pressEnter: {} },
    { onPressEnter: (__VLS_ctx.handleFinish) });
const { default: __VLS_57 } = __VLS_53.slots;
{
    const { prefix: __VLS_58 } = __VLS_53.slots;
    let __VLS_59;
    /** @ts-ignore @type {typeof __VLS_components.LockOutlined} */
    LockOutlined;
    // @ts-ignore
    const __VLS_60 = __VLS_asFunctionalComponent1(__VLS_59, new __VLS_59({
        ...{ class: "text-gray-400" },
    }));
    const __VLS_61 = __VLS_60({
        ...{ class: "text-gray-400" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_60));
    /** @type {__VLS_StyleScopedClasses['text-gray-400']} */ ;
    // @ts-ignore
    [formState, handleFinish,];
}
// @ts-ignore
[];
var __VLS_53;
var __VLS_54;
// @ts-ignore
[];
var __VLS_47;
if (__VLS_ctx.errorMsg) {
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "text-red-500 text-sm mb-4 flex items-center" },
    });
    /** @type {__VLS_StyleScopedClasses['text-red-500']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    let __VLS_64;
    /** @ts-ignore @type {typeof __VLS_components.ExclamationCircleOutlined} */
    ExclamationCircleOutlined;
    // @ts-ignore
    const __VLS_65 = __VLS_asFunctionalComponent1(__VLS_64, new __VLS_64({
        ...{ class: "mr-1.5" },
    }));
    const __VLS_66 = __VLS_65({
        ...{ class: "mr-1.5" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_65));
    /** @type {__VLS_StyleScopedClasses['mr-1.5']} */ ;
    (__VLS_ctx.errorMsg);
}
let __VLS_69;
/** @ts-ignore @type {typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem | typeof __VLS_components.aFormItem | typeof __VLS_components.AFormItem} */
aFormItem;
// @ts-ignore
const __VLS_70 = __VLS_asFunctionalComponent1(__VLS_69, new __VLS_69({
    ...{ class: "mb-0 mt-6" },
}));
const __VLS_71 = __VLS_70({
    ...{ class: "mb-0 mt-6" },
}, ...__VLS_functionalComponentArgsRest(__VLS_70));
/** @type {__VLS_StyleScopedClasses['mb-0']} */ ;
/** @type {__VLS_StyleScopedClasses['mt-6']} */ ;
const { default: __VLS_74 } = __VLS_72.slots;
let __VLS_75;
/** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
aButton;
// @ts-ignore
const __VLS_76 = __VLS_asFunctionalComponent1(__VLS_75, new __VLS_75({
    type: "primary",
    htmlType: "submit",
    size: "large",
    ...{ class: "w-full h-12 text-lg tracking-widest" },
    loading: (__VLS_ctx.loading),
}));
const __VLS_77 = __VLS_76({
    type: "primary",
    htmlType: "submit",
    size: "large",
    ...{ class: "w-full h-12 text-lg tracking-widest" },
    loading: (__VLS_ctx.loading),
}, ...__VLS_functionalComponentArgsRest(__VLS_76));
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['h-12']} */ ;
/** @type {__VLS_StyleScopedClasses['text-lg']} */ ;
/** @type {__VLS_StyleScopedClasses['tracking-widest']} */ ;
const { default: __VLS_80 } = __VLS_78.slots;
// @ts-ignore
[errorMsg, errorMsg, loading,];
var __VLS_78;
// @ts-ignore
[];
var __VLS_72;
// @ts-ignore
[];
var __VLS_3;
var __VLS_4;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
