/// <reference types="../../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, watch } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useAppStore } from '@/store/app';
import AppLogo from './AppLogo.vue';
import { ApiOutlined, CodeOutlined, DashboardOutlined, FileTextOutlined, RobotOutlined, } from '@ant-design/icons-vue';
const router = useRouter();
const route = useRoute();
const appStore = useAppStore();
const selectedKeys = ref(['dashboard']);
watch(() => route.path, (newPath) => {
    const key = newPath.split('/')[1] || 'dashboard';
    selectedKeys.value = [key];
}, { immediate: true });
const handleMenuClick = ({ key }) => {
    router.push(`/${key}`);
};
const __VLS_ctx = {
    ...{},
    ...{},
};
let __VLS_components;
let __VLS_intrinsics;
let __VLS_directives;
let __VLS_0;
/** @ts-ignore @type {typeof __VLS_components.aLayoutSider | typeof __VLS_components.ALayoutSider | typeof __VLS_components.aLayoutSider | typeof __VLS_components.ALayoutSider} */
aLayoutSider;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    collapsed: (__VLS_ctx.appStore.sidebarCollapsed),
    trigger: (null),
    collapsible: true,
    theme: "dark",
    width: "200",
    collapsedWidth: "64",
    ...{ class: "h-screen z-20 relative border-r border-[var(--border-default)] dark:border-[var(--border-default)]" },
    ...{ style: ({ background: __VLS_ctx.appStore.isDark ? 'var(--bg-sidebar)' : 'var(--bg-sidebar)' }) },
}));
const __VLS_2 = __VLS_1({
    collapsed: (__VLS_ctx.appStore.sidebarCollapsed),
    trigger: (null),
    collapsible: true,
    theme: "dark",
    width: "200",
    collapsedWidth: "64",
    ...{ class: "h-screen z-20 relative border-r border-[var(--border-default)] dark:border-[var(--border-default)]" },
    ...{ style: ({ background: __VLS_ctx.appStore.isDark ? 'var(--bg-sidebar)' : 'var(--bg-sidebar)' }) },
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
var __VLS_5 = {};
/** @type {__VLS_StyleScopedClasses['h-screen']} */ ;
/** @type {__VLS_StyleScopedClasses['z-20']} */ ;
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
/** @type {__VLS_StyleScopedClasses['border-r']} */ ;
/** @type {__VLS_StyleScopedClasses['border-[var(--border-default)]']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:border-[var(--border-default)]']} */ ;
const { default: __VLS_6 } = __VLS_3.slots;
const __VLS_7 = AppLogo;
// @ts-ignore
const __VLS_8 = __VLS_asFunctionalComponent1(__VLS_7, new __VLS_7({}));
const __VLS_9 = __VLS_8({}, ...__VLS_functionalComponentArgsRest(__VLS_8));
let __VLS_12;
/** @ts-ignore @type {typeof __VLS_components.aMenu | typeof __VLS_components.AMenu | typeof __VLS_components.aMenu | typeof __VLS_components.AMenu} */
aMenu;
// @ts-ignore
const __VLS_13 = __VLS_asFunctionalComponent1(__VLS_12, new __VLS_12({
    ...{ 'onClick': {} },
    selectedKeys: (__VLS_ctx.selectedKeys),
    theme: "dark",
    mode: "inline",
    ...{ style: ({ background: 'transparent' }) },
    ...{ class: "h-[calc(100vh-48px)] overflow-y-auto overflow-x-hidden pt-2 !border-none" },
}));
const __VLS_14 = __VLS_13({
    ...{ 'onClick': {} },
    selectedKeys: (__VLS_ctx.selectedKeys),
    theme: "dark",
    mode: "inline",
    ...{ style: ({ background: 'transparent' }) },
    ...{ class: "h-[calc(100vh-48px)] overflow-y-auto overflow-x-hidden pt-2 !border-none" },
}, ...__VLS_functionalComponentArgsRest(__VLS_13));
let __VLS_17;
const __VLS_18 = ({ click: {} },
    { onClick: (__VLS_ctx.handleMenuClick) });
/** @type {__VLS_StyleScopedClasses['h-[calc(100vh-48px)]']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-y-auto']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-x-hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['pt-2']} */ ;
/** @type {__VLS_StyleScopedClasses['!border-none']} */ ;
const { default: __VLS_19 } = __VLS_15.slots;
let __VLS_20;
/** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
aMenuItem;
// @ts-ignore
const __VLS_21 = __VLS_asFunctionalComponent1(__VLS_20, new __VLS_20({
    key: "dashboard",
}));
const __VLS_22 = __VLS_21({
    key: "dashboard",
}, ...__VLS_functionalComponentArgsRest(__VLS_21));
const { default: __VLS_25 } = __VLS_23.slots;
{
    const { icon: __VLS_26 } = __VLS_23.slots;
    let __VLS_27;
    /** @ts-ignore @type {typeof __VLS_components.DashboardOutlined} */
    DashboardOutlined;
    // @ts-ignore
    const __VLS_28 = __VLS_asFunctionalComponent1(__VLS_27, new __VLS_27({}));
    const __VLS_29 = __VLS_28({}, ...__VLS_functionalComponentArgsRest(__VLS_28));
    // @ts-ignore
    [appStore, appStore, selectedKeys, handleMenuClick,];
}
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({});
// @ts-ignore
[];
var __VLS_23;
let __VLS_32;
/** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
aMenuItem;
// @ts-ignore
const __VLS_33 = __VLS_asFunctionalComponent1(__VLS_32, new __VLS_32({
    key: "agent",
}));
const __VLS_34 = __VLS_33({
    key: "agent",
}, ...__VLS_functionalComponentArgsRest(__VLS_33));
const { default: __VLS_37 } = __VLS_35.slots;
{
    const { icon: __VLS_38 } = __VLS_35.slots;
    let __VLS_39;
    /** @ts-ignore @type {typeof __VLS_components.RobotOutlined} */
    RobotOutlined;
    // @ts-ignore
    const __VLS_40 = __VLS_asFunctionalComponent1(__VLS_39, new __VLS_39({}));
    const __VLS_41 = __VLS_40({}, ...__VLS_functionalComponentArgsRest(__VLS_40));
    // @ts-ignore
    [];
}
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({});
// @ts-ignore
[];
var __VLS_35;
let __VLS_44;
/** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
aMenuItem;
// @ts-ignore
const __VLS_45 = __VLS_asFunctionalComponent1(__VLS_44, new __VLS_44({
    key: "listener",
}));
const __VLS_46 = __VLS_45({
    key: "listener",
}, ...__VLS_functionalComponentArgsRest(__VLS_45));
const { default: __VLS_49 } = __VLS_47.slots;
{
    const { icon: __VLS_50 } = __VLS_47.slots;
    let __VLS_51;
    /** @ts-ignore @type {typeof __VLS_components.ApiOutlined} */
    ApiOutlined;
    // @ts-ignore
    const __VLS_52 = __VLS_asFunctionalComponent1(__VLS_51, new __VLS_51({}));
    const __VLS_53 = __VLS_52({}, ...__VLS_functionalComponentArgsRest(__VLS_52));
    // @ts-ignore
    [];
}
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({});
// @ts-ignore
[];
var __VLS_47;
let __VLS_56;
/** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
aMenuItem;
// @ts-ignore
const __VLS_57 = __VLS_asFunctionalComponent1(__VLS_56, new __VLS_56({
    key: "payload",
}));
const __VLS_58 = __VLS_57({
    key: "payload",
}, ...__VLS_functionalComponentArgsRest(__VLS_57));
const { default: __VLS_61 } = __VLS_59.slots;
{
    const { icon: __VLS_62 } = __VLS_59.slots;
    let __VLS_63;
    /** @ts-ignore @type {typeof __VLS_components.CodeOutlined} */
    CodeOutlined;
    // @ts-ignore
    const __VLS_64 = __VLS_asFunctionalComponent1(__VLS_63, new __VLS_63({}));
    const __VLS_65 = __VLS_64({}, ...__VLS_functionalComponentArgsRest(__VLS_64));
    // @ts-ignore
    [];
}
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({});
// @ts-ignore
[];
var __VLS_59;
let __VLS_68;
/** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
aMenuItem;
// @ts-ignore
const __VLS_69 = __VLS_asFunctionalComponent1(__VLS_68, new __VLS_68({
    key: "log",
}));
const __VLS_70 = __VLS_69({
    key: "log",
}, ...__VLS_functionalComponentArgsRest(__VLS_69));
const { default: __VLS_73 } = __VLS_71.slots;
{
    const { icon: __VLS_74 } = __VLS_71.slots;
    let __VLS_75;
    /** @ts-ignore @type {typeof __VLS_components.FileTextOutlined} */
    FileTextOutlined;
    // @ts-ignore
    const __VLS_76 = __VLS_asFunctionalComponent1(__VLS_75, new __VLS_75({}));
    const __VLS_77 = __VLS_76({}, ...__VLS_functionalComponentArgsRest(__VLS_76));
    // @ts-ignore
    [];
}
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({});
// @ts-ignore
[];
var __VLS_71;
// @ts-ignore
[];
var __VLS_15;
var __VLS_16;
// @ts-ignore
[];
var __VLS_3;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
