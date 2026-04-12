/// <reference types="../../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useAppStore } from '@/store/app';
import { DashboardOutlined, RobotOutlined, ApiOutlined, CodeOutlined, FileTextOutlined, CloseOutlined } from '@ant-design/icons-vue';
const routeIcons = {
    dashboard: DashboardOutlined,
    agent: RobotOutlined,
    listener: ApiOutlined,
    payload: CodeOutlined,
    log: FileTextOutlined,
};
const route = useRoute();
const router = useRouter();
const appStore = useAppStore();
watch(() => route.path, () => {
    if (route.name && route.meta?.title) {
        appStore.addView({
            path: route.path,
            name: route.name,
            title: route.meta.title.split(' - ')[0]
        });
    }
}, { immediate: true });
const closeTab = (path) => {
    appStore.removeView(path);
    if (route.path === path) {
        const lastView = appStore.visitedViews[appStore.visitedViews.length - 1];
        if (lastView)
            router.push(lastView.path);
        else
            router.push('/dashboard');
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
    ...{ class: "flex-shrink-0 w-full bg-slate-100 dark:bg-[#14161A] border-b border-gray-200 dark:border-[#14161A] pt-2 px-2 flex items-end space-x-1 overflow-x-auto z-10 transition-colors duration-300 h-[42px]" },
});
/** @type {__VLS_StyleScopedClasses['flex-shrink-0']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-slate-100']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[#14161A]']} */ ;
/** @type {__VLS_StyleScopedClasses['border-b']} */ ;
/** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:border-[#14161A]']} */ ;
/** @type {__VLS_StyleScopedClasses['pt-2']} */ ;
/** @type {__VLS_StyleScopedClasses['px-2']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-end']} */ ;
/** @type {__VLS_StyleScopedClasses['space-x-1']} */ ;
/** @type {__VLS_StyleScopedClasses['overflow-x-auto']} */ ;
/** @type {__VLS_StyleScopedClasses['z-10']} */ ;
/** @type {__VLS_StyleScopedClasses['transition-colors']} */ ;
/** @type {__VLS_StyleScopedClasses['duration-300']} */ ;
/** @type {__VLS_StyleScopedClasses['h-[42px]']} */ ;
for (const [tab] of __VLS_vFor((__VLS_ctx.appStore.visitedViews))) {
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ onClick: (...[$event]) => {
                __VLS_ctx.router.push(tab.path);
                // @ts-ignore
                [appStore, router,];
            } },
        key: (tab.path),
        ...{ class: ([
                'group relative flex items-center h-[34px] px-4 min-w-[120px] max-w-[200px] cursor-pointer select-none transition-all duration-200',
                'rounded-t-lg mx-[-1px]',
                __VLS_ctx.route.path === tab.path
                    ? 'bg-white dark:bg-[#1C1E22] text-primary font-medium z-10'
                    : 'bg-transparent text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200 hover:bg-slate-200/50 dark:hover:bg-[#1C1E22]/50'
            ]) },
    });
    /** @type {__VLS_StyleScopedClasses['group']} */ ;
    /** @type {__VLS_StyleScopedClasses['relative']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['h-[34px]']} */ ;
    /** @type {__VLS_StyleScopedClasses['px-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['min-w-[120px]']} */ ;
    /** @type {__VLS_StyleScopedClasses['max-w-[200px]']} */ ;
    /** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
    /** @type {__VLS_StyleScopedClasses['select-none']} */ ;
    /** @type {__VLS_StyleScopedClasses['transition-all']} */ ;
    /** @type {__VLS_StyleScopedClasses['duration-200']} */ ;
    /** @type {__VLS_StyleScopedClasses['rounded-t-lg']} */ ;
    /** @type {__VLS_StyleScopedClasses['mx-[-1px]']} */ ;
    if (__VLS_ctx.route.path !== tab.path) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "absolute right-0 top-1/2 -translate-y-1/2 w-[1px] h-4 bg-slate-300 dark:bg-slate-700 group-hover:hidden" },
        });
        /** @type {__VLS_StyleScopedClasses['absolute']} */ ;
        /** @type {__VLS_StyleScopedClasses['right-0']} */ ;
        /** @type {__VLS_StyleScopedClasses['top-1/2']} */ ;
        /** @type {__VLS_StyleScopedClasses['-translate-y-1/2']} */ ;
        /** @type {__VLS_StyleScopedClasses['w-[1px]']} */ ;
        /** @type {__VLS_StyleScopedClasses['h-4']} */ ;
        /** @type {__VLS_StyleScopedClasses['bg-slate-300']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:bg-slate-700']} */ ;
        /** @type {__VLS_StyleScopedClasses['group-hover:hidden']} */ ;
    }
    if (__VLS_ctx.route.path === tab.path) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "absolute top-0 left-0 w-full h-[2px] bg-primary rounded-t-lg" },
        });
        /** @type {__VLS_StyleScopedClasses['absolute']} */ ;
        /** @type {__VLS_StyleScopedClasses['top-0']} */ ;
        /** @type {__VLS_StyleScopedClasses['left-0']} */ ;
        /** @type {__VLS_StyleScopedClasses['w-full']} */ ;
        /** @type {__VLS_StyleScopedClasses['h-[2px]']} */ ;
        /** @type {__VLS_StyleScopedClasses['bg-primary']} */ ;
        /** @type {__VLS_StyleScopedClasses['rounded-t-lg']} */ ;
    }
    const __VLS_0 = (__VLS_ctx.routeIcons[(tab.path.split('/')[1] || 'dashboard')] || __VLS_ctx.DashboardOutlined);
    // @ts-ignore
    const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
        ...{ class: "mr-2 text-[14px]" },
        ...{ class: (__VLS_ctx.route.path === tab.path ? 'text-primary' : 'text-slate-400') },
    }));
    const __VLS_2 = __VLS_1({
        ...{ class: "mr-2 text-[14px]" },
        ...{ class: (__VLS_ctx.route.path === tab.path ? 'text-primary' : 'text-slate-400') },
    }, ...__VLS_functionalComponentArgsRest(__VLS_1));
    /** @type {__VLS_StyleScopedClasses['mr-2']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-[14px]']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
        ...{ class: "flex-1 truncate text-xs" },
    });
    /** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
    /** @type {__VLS_StyleScopedClasses['truncate']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
    (tab.title);
    if (__VLS_ctx.appStore.visitedViews.length > 1) {
        let __VLS_5;
        /** @ts-ignore @type {typeof __VLS_components.CloseOutlined} */
        CloseOutlined;
        // @ts-ignore
        const __VLS_6 = __VLS_asFunctionalComponent1(__VLS_5, new __VLS_5({
            ...{ 'onClick': {} },
            ...{ class: "ml-2 text-[10px] p-0.5 rounded-full hover:bg-slate-200 dark:hover:bg-slate-700 text-slate-400 hover:text-red-500 transition-all opacity-0 group-hover:opacity-100" },
            ...{ class: (__VLS_ctx.route.path === tab.path ? '!opacity-100' : '') },
        }));
        const __VLS_7 = __VLS_6({
            ...{ 'onClick': {} },
            ...{ class: "ml-2 text-[10px] p-0.5 rounded-full hover:bg-slate-200 dark:hover:bg-slate-700 text-slate-400 hover:text-red-500 transition-all opacity-0 group-hover:opacity-100" },
            ...{ class: (__VLS_ctx.route.path === tab.path ? '!opacity-100' : '') },
        }, ...__VLS_functionalComponentArgsRest(__VLS_6));
        let __VLS_10;
        const __VLS_11 = ({ click: {} },
            { onClick: (...[$event]) => {
                    if (!(__VLS_ctx.appStore.visitedViews.length > 1))
                        return;
                    __VLS_ctx.closeTab(tab.path);
                    // @ts-ignore
                    [appStore, route, route, route, route, route, routeIcons, routeIcons, DashboardOutlined, closeTab,];
                } });
        /** @type {__VLS_StyleScopedClasses['ml-2']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-[10px]']} */ ;
        /** @type {__VLS_StyleScopedClasses['p-0.5']} */ ;
        /** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
        /** @type {__VLS_StyleScopedClasses['hover:bg-slate-200']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:hover:bg-slate-700']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
        /** @type {__VLS_StyleScopedClasses['hover:text-red-500']} */ ;
        /** @type {__VLS_StyleScopedClasses['transition-all']} */ ;
        /** @type {__VLS_StyleScopedClasses['opacity-0']} */ ;
        /** @type {__VLS_StyleScopedClasses['group-hover:opacity-100']} */ ;
        var __VLS_8;
        var __VLS_9;
    }
    // @ts-ignore
    [];
}
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
