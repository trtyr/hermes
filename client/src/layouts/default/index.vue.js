/// <reference types="../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import AppSidebar from './components/AppSidebar.vue';
import AppHeader from './components/AppHeader.vue';
import AppTabs from './components/AppTabs.vue';
const route = useRoute();
const contentRefreshKey = ref(0);
const refreshing = ref(false);
const refreshCurrentView = () => {
    refreshing.value = true;
    contentRefreshKey.value += 1;
    window.setTimeout(() => {
        refreshing.value = false;
    }, 450);
};
const __VLS_ctx = {
    ...{},
    ...{},
};
let __VLS_components;
let __VLS_intrinsics;
let __VLS_directives;
let __VLS_0;
/** @ts-ignore @type {typeof __VLS_components.aLayout | typeof __VLS_components.ALayout | typeof __VLS_components.aLayout | typeof __VLS_components.ALayout} */
aLayout;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    ...{ class: "h-screen overflow-hidden transition-colors duration-300" },
}));
const __VLS_2 = __VLS_1({
    ...{ class: "h-screen overflow-hidden transition-colors duration-300" },
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
var __VLS_5 = {};
/** @type {__VLS_StyleScopedClasses['h-screen']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['transition-colors']} */ ;
/** @type {__VLS_StyleScopedClasses['duration-300']} */ ;
const { default: __VLS_6 } = __VLS_3.slots;
const __VLS_7 = AppSidebar;
// @ts-ignore
const __VLS_8 = __VLS_asFunctionalComponent1(__VLS_7, new __VLS_7({}));
const __VLS_9 = __VLS_8({}, ...__VLS_functionalComponentArgsRest(__VLS_8));
let __VLS_12;
/** @ts-ignore @type {typeof __VLS_components.aLayout | typeof __VLS_components.ALayout | typeof __VLS_components.aLayout | typeof __VLS_components.ALayout} */
aLayout;
// @ts-ignore
const __VLS_13 = __VLS_asFunctionalComponent1(__VLS_12, new __VLS_12({
    ...{ class: "h-screen bg-[#f0f2f5] dark:bg-[var(--bg-page)] flex flex-col" },
}));
const __VLS_14 = __VLS_13({
    ...{ class: "h-screen bg-[#f0f2f5] dark:bg-[var(--bg-page)] flex flex-col" },
}, ...__VLS_functionalComponentArgsRest(__VLS_13));
/** @type {__VLS_StyleScopedClasses['h-screen']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-[#f0f2f5]']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-page)]']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
const { default: __VLS_17 } = __VLS_15.slots;
const __VLS_18 = AppHeader;
// @ts-ignore
const __VLS_19 = __VLS_asFunctionalComponent1(__VLS_18, new __VLS_18({
    ...{ 'onRefresh': {} },
    isRefreshing: (__VLS_ctx.refreshing),
}));
const __VLS_20 = __VLS_19({
    ...{ 'onRefresh': {} },
    isRefreshing: (__VLS_ctx.refreshing),
}, ...__VLS_functionalComponentArgsRest(__VLS_19));
let __VLS_23;
const __VLS_24 = ({ refresh: {} },
    { onRefresh: (__VLS_ctx.refreshCurrentView) });
var __VLS_21;
var __VLS_22;
const __VLS_25 = AppTabs;
// @ts-ignore
const __VLS_26 = __VLS_asFunctionalComponent1(__VLS_25, new __VLS_25({}));
const __VLS_27 = __VLS_26({}, ...__VLS_functionalComponentArgsRest(__VLS_26));
let __VLS_30;
/** @ts-ignore @type {typeof __VLS_components.aLayoutContent | typeof __VLS_components.ALayoutContent | typeof __VLS_components.aLayoutContent | typeof __VLS_components.ALayoutContent} */
aLayoutContent;
// @ts-ignore
const __VLS_31 = __VLS_asFunctionalComponent1(__VLS_30, new __VLS_30({
    ...{ class: "flex-1 w-full p-4 overflow-hidden relative flex flex-col" },
}));
const __VLS_32 = __VLS_31({
    ...{ class: "flex-1 w-full p-4 overflow-hidden relative flex flex-col" },
}, ...__VLS_functionalComponentArgsRest(__VLS_31));
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['p-4']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
const { default: __VLS_35 } = __VLS_33.slots;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex-1 w-full bg-white dark:bg-[var(--bg-card)] rounded-md shadow-sm border border-gray-200 dark:border-[var(--border-default)] overflow-y-auto relative" },
});
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[var(--bg-card)]']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-md']} */ ;
/** @type {__VLS_StyleScopedClasses['shadow-sm']} */ ;
/** @type {__VLS_StyleScopedClasses['border']} */ ;
/** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:border-[var(--border-default)]']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-y-auto']} */ ;
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
let __VLS_36;
/** @ts-ignore @type {typeof __VLS_components.routerView | typeof __VLS_components.RouterView | typeof __VLS_components.routerView | typeof __VLS_components.RouterView} */
routerView;
// @ts-ignore
const __VLS_37 = __VLS_asFunctionalComponent1(__VLS_36, new __VLS_36({}));
const __VLS_38 = __VLS_37({}, ...__VLS_functionalComponentArgsRest(__VLS_37));
{
    const { default: __VLS_41 } = __VLS_39.slots;
    const [{ Component }] = __VLS_vSlot(__VLS_41);
    let __VLS_42;
    /** @ts-ignore @type {typeof __VLS_components.transition | typeof __VLS_components.Transition | typeof __VLS_components.transition | typeof __VLS_components.Transition} */
    transition;
    // @ts-ignore
    const __VLS_43 = __VLS_asFunctionalComponent1(__VLS_42, new __VLS_42({
        name: "fade-slide",
        mode: "out-in",
    }));
    const __VLS_44 = __VLS_43({
        name: "fade-slide",
        mode: "out-in",
    }, ...__VLS_functionalComponentArgsRest(__VLS_43));
    const { default: __VLS_47 } = __VLS_45.slots;
    let __VLS_48;
    /** @ts-ignore @type {typeof __VLS_components.keepAlive | typeof __VLS_components.KeepAlive | typeof __VLS_components.keepAlive | typeof __VLS_components.KeepAlive} */
    keepAlive;
    // @ts-ignore
    const __VLS_49 = __VLS_asFunctionalComponent1(__VLS_48, new __VLS_48({}));
    const __VLS_50 = __VLS_49({}, ...__VLS_functionalComponentArgsRest(__VLS_49));
    const { default: __VLS_53 } = __VLS_51.slots;
    const __VLS_54 = (Component);
    // @ts-ignore
    const __VLS_55 = __VLS_asFunctionalComponent1(__VLS_54, new __VLS_54({
        key: (__VLS_ctx.route.path + __VLS_ctx.contentRefreshKey),
    }));
    const __VLS_56 = __VLS_55({
        key: (__VLS_ctx.route.path + __VLS_ctx.contentRefreshKey),
    }, ...__VLS_functionalComponentArgsRest(__VLS_55));
    // @ts-ignore
    [refreshing, refreshCurrentView, route, contentRefreshKey,];
    var __VLS_51;
    // @ts-ignore
    [];
    var __VLS_45;
    // @ts-ignore
    [];
    __VLS_39.slots['' /* empty slot name completion */];
}
var __VLS_39;
// @ts-ignore
[];
var __VLS_33;
// @ts-ignore
[];
var __VLS_15;
// @ts-ignore
[];
var __VLS_3;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
