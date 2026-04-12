import { computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { message } from 'ant-design-vue';
import { useAppStore } from '@/store/app';
import { DashboardOutlined, RobotOutlined, ApiOutlined, CodeOutlined, FileTextOutlined, UserOutlined, SettingOutlined, LogoutOutlined } from '@ant-design/icons-vue';
const props = defineProps();
const emit = defineEmits(['refresh']);
const router = useRouter();
const route = useRoute();
const appStore = useAppStore();
const routeIcons = {
    dashboard: DashboardOutlined,
    agent: RobotOutlined,
    listener: ApiOutlined,
    payload: CodeOutlined,
    log: FileTextOutlined,
};
const routeMetaTitle = computed(() => route.meta?.title?.toString().split(' - ')[0] || '页面');
const currentRouteKey = computed(() => (route.path.split('/')[1] || 'dashboard'));
const currentRouteIcon = computed(() => routeIcons[currentRouteKey.value] || DashboardOutlined);
const toggleFullscreen = async () => {
    try {
        if (!document.fullscreenElement)
            await document.documentElement.requestFullscreen();
        else
            await document.exitFullscreen();
    }
    catch (err) {
        message.error('不支持全屏');
    }
};
const openAction = (name) => {
    message.info(`${name} 入口已预留`);
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
__VLS_asFunctionalElement1(__VLS_intrinsics.header, __VLS_intrinsics.header)({
    ...{ class: "top-0 flex w-full flex-[0_0_auto] items-center border-b border-gray-200 bg-white pl-2 transition-[margin-top] duration-200 dark:border-[#14161A] dark:bg-[#1C1E22]" },
    ...{ style: {} },
});
/** @type {__VLS_StyleScopedClasses['top-0']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-[0_0_auto]']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['border-b']} */ ;
/** @type {__VLS_StyleScopedClasses['border-gray-200']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
/** @type {__VLS_StyleScopedClasses['pl-2']} */ ;
/** @type {__VLS_StyleScopedClasses['transition-[margin-top]']} */ ;
/** @type {__VLS_StyleScopedClasses['duration-200']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:border-[#14161A]']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[#1C1E22]']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.button, __VLS_intrinsics.button)({
    ...{ onClick: (__VLS_ctx.appStore.toggleCollapse) },
    type: "button",
    ...{ class: "inline-flex items-center justify-center whitespace-nowrap font-medium transition-colors h-8 w-8 text-lg text-slate-500 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-[#2A2D33] rounded-md mr-1" },
});
/** @type {__VLS_StyleScopedClasses['inline-flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
/** @type {__VLS_StyleScopedClasses['whitespace-nowrap']} */ ;
/** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
/** @type {__VLS_StyleScopedClasses['transition-colors']} */ ;
/** @type {__VLS_StyleScopedClasses['h-8']} */ ;
/** @type {__VLS_StyleScopedClasses['w-8']} */ ;
/** @type {__VLS_StyleScopedClasses['text-lg']} */ ;
/** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
/** @type {__VLS_StyleScopedClasses['hover:bg-slate-100']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-slate-400']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:hover:bg-[#2A2D33]']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-md']} */ ;
/** @type {__VLS_StyleScopedClasses['mr-1']} */ ;
if (__VLS_ctx.appStore.sidebarCollapsed) {
    __VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
        'aria-hidden': "true",
        viewBox: "0 0 1024 1024",
        ...{ class: "size-4 fill-current" },
    });
    /** @type {__VLS_StyleScopedClasses['size-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['fill-current']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
        d: "M128 192h768v128H128zm256 256h512v128H384zm-256 256h768v128H128zm576-320 192 128-192 128z",
    });
}
else {
    __VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
        'aria-hidden': "true",
        viewBox: "0 0 1024 1024",
        ...{ class: "size-4 fill-current" },
    });
    /** @type {__VLS_StyleScopedClasses['size-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['fill-current']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
        d: "M896 192H128v128h768zm0 256H384v128h512zm0 256H128v128h768zM320 384L128 512l192 128z",
    });
}
__VLS_asFunctionalElement1(__VLS_intrinsics.button, __VLS_intrinsics.button)({
    ...{ onClick: (...[$event]) => {
            __VLS_ctx.emit('refresh');
            // @ts-ignore
            [appStore, appStore, emit,];
        } },
    type: "button",
    ...{ class: "inline-flex items-center justify-center whitespace-nowrap font-medium transition-colors h-8 w-8 text-lg text-slate-500 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-[#2A2D33] rounded-md mr-1" },
});
/** @type {__VLS_StyleScopedClasses['inline-flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
/** @type {__VLS_StyleScopedClasses['whitespace-nowrap']} */ ;
/** @type {__VLS_StyleScopedClasses['font-medium']} */ ;
/** @type {__VLS_StyleScopedClasses['transition-colors']} */ ;
/** @type {__VLS_StyleScopedClasses['h-8']} */ ;
/** @type {__VLS_StyleScopedClasses['w-8']} */ ;
/** @type {__VLS_StyleScopedClasses['text-lg']} */ ;
/** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
/** @type {__VLS_StyleScopedClasses['hover:bg-slate-100']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-slate-400']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:hover:bg-[#2A2D33]']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-md']} */ ;
/** @type {__VLS_StyleScopedClasses['mr-1']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    'aria-hidden': "true",
    viewBox: "0 0 24 24",
    ...{ class: "size-4 fill-none stroke-current stroke-2" },
    ...{ class: ({ 'animate-spin': __VLS_ctx.isRefreshing }) },
});
/** @type {__VLS_StyleScopedClasses['size-4']} */ ;
/** @type {__VLS_StyleScopedClasses['fill-none']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-current']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-2']} */ ;
/** @type {__VLS_StyleScopedClasses['animate-spin']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
    'stroke-linecap': "round",
    'stroke-linejoin': "round",
    d: "M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
    'stroke-linecap': "round",
    'stroke-linejoin': "round",
    d: "M21 3v5h-5",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "hidden lg:flex items-center mt-[2px]" },
});
/** @type {__VLS_StyleScopedClasses['hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['lg:flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['mt-[2px]']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.nav, __VLS_intrinsics.nav)({
    ...{ class: "ml-2 flex flex-wrap items-center gap-1.5 text-sm m-0 leading-none" },
});
/** @type {__VLS_StyleScopedClasses['ml-2']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['flex-wrap']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['gap-1.5']} */ ;
/** @type {__VLS_StyleScopedClasses['text-sm']} */ ;
/** @type {__VLS_StyleScopedClasses['m-0']} */ ;
/** @type {__VLS_StyleScopedClasses['leading-none']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.a, __VLS_intrinsics.a)({
    ...{ onClick: (...[$event]) => {
            __VLS_ctx.router.push('/dashboard');
            // @ts-ignore
            [isRefreshing, router,];
        } },
    ...{ class: "text-slate-500 hover:text-slate-900 dark:hover:text-slate-100 transition-colors flex items-center leading-none" },
    href: "javascript:void 0",
});
/** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
/** @type {__VLS_StyleScopedClasses['hover:text-slate-900']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:hover:text-slate-100']} */ ;
/** @type {__VLS_StyleScopedClasses['transition-colors']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['leading-none']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    viewBox: "0 0 24 24",
    ...{ class: "mr-1 size-4 fill-none stroke-current stroke-2" },
});
/** @type {__VLS_StyleScopedClasses['mr-1']} */ ;
/** @type {__VLS_StyleScopedClasses['size-4']} */ ;
/** @type {__VLS_StyleScopedClasses['fill-none']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-current']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-2']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.rect, __VLS_intrinsics.rect)({
    x: "3",
    y: "3",
    width: "7",
    height: "9",
    rx: "1",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.rect, __VLS_intrinsics.rect)({
    x: "14",
    y: "3",
    width: "7",
    height: "5",
    rx: "1",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.rect, __VLS_intrinsics.rect)({
    x: "14",
    y: "12",
    width: "7",
    height: "9",
    rx: "1",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.rect, __VLS_intrinsics.rect)({
    x: "3",
    y: "16",
    width: "7",
    height: "5",
    rx: "1",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "translate-y-[0.5px]" },
});
/** @type {__VLS_StyleScopedClasses['translate-y-[0.5px]']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "text-slate-300 dark:text-slate-600 flex items-center" },
});
/** @type {__VLS_StyleScopedClasses['text-slate-300']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-slate-600']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    viewBox: "0 0 24 24",
    ...{ class: "size-3.5 fill-none stroke-current stroke-2" },
});
/** @type {__VLS_StyleScopedClasses['size-3.5']} */ ;
/** @type {__VLS_StyleScopedClasses['fill-none']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-current']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-2']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
    'stroke-linecap': "round",
    'stroke-linejoin': "round",
    d: "m9 18 6-6-6-6",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "text-slate-900 dark:text-slate-100 flex items-center leading-none" },
});
/** @type {__VLS_StyleScopedClasses['text-slate-900']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-slate-100']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['leading-none']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "mr-1 text-[16px] leading-none flex items-center" },
});
/** @type {__VLS_StyleScopedClasses['mr-1']} */ ;
/** @type {__VLS_StyleScopedClasses['text-[16px]']} */ ;
/** @type {__VLS_StyleScopedClasses['leading-none']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
const __VLS_0 = (__VLS_ctx.currentRouteIcon);
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({}));
const __VLS_2 = __VLS_1({}, ...__VLS_functionalComponentArgsRest(__VLS_1));
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "translate-y-[0.5px]" },
});
/** @type {__VLS_StyleScopedClasses['translate-y-[0.5px]']} */ ;
(__VLS_ctx.routeMetaTitle);
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex-1" },
});
/** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "flex items-center" },
});
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ onClick: (...[$event]) => {
            __VLS_ctx.openAction('全局搜索');
            // @ts-ignore
            [currentRouteIcon, routeMetaTitle, openAction,];
        } },
    ...{ class: "mr-1 sm:mr-4 flex items-center gap-3 cursor-pointer rounded-2xl md:bg-slate-100 dark:md:bg-[#2A2D33] px-2 py-0.5 text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100" },
});
/** @type {__VLS_StyleScopedClasses['mr-1']} */ ;
/** @type {__VLS_StyleScopedClasses['sm:mr-4']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['gap-3']} */ ;
/** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-2xl']} */ ;
/** @type {__VLS_StyleScopedClasses['md:bg-slate-100']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:md:bg-[#2A2D33]']} */ ;
/** @type {__VLS_StyleScopedClasses['px-2']} */ ;
/** @type {__VLS_StyleScopedClasses['py-0.5']} */ ;
/** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
/** @type {__VLS_StyleScopedClasses['hover:text-slate-900']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-slate-400']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:hover:text-slate-100']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    viewBox: "0 0 24 24",
    ...{ class: "size-4 fill-none stroke-current stroke-2" },
});
/** @type {__VLS_StyleScopedClasses['size-4']} */ ;
/** @type {__VLS_StyleScopedClasses['fill-none']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-current']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-2']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
    'stroke-linecap': "round",
    'stroke-linejoin': "round",
    d: "m21 21-4.34-4.34",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.circle, __VLS_intrinsics.circle)({
    cx: "11",
    cy: "11",
    r: "8",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "hidden md:block text-xs" },
});
/** @type {__VLS_StyleScopedClasses['hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['md:block']} */ ;
/** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "hidden md:block border bg-white dark:bg-[#1C1E22] border-slate-300 dark:border-slate-600 px-1.5 py-1 text-xs rounded-r-xl" },
});
/** @type {__VLS_StyleScopedClasses['hidden']} */ ;
/** @type {__VLS_StyleScopedClasses['md:block']} */ ;
/** @type {__VLS_StyleScopedClasses['border']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-white']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-[#1C1E22]']} */ ;
/** @type {__VLS_StyleScopedClasses['border-slate-300']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:border-slate-600']} */ ;
/** @type {__VLS_StyleScopedClasses['px-1.5']} */ ;
/** @type {__VLS_StyleScopedClasses['py-1']} */ ;
/** @type {__VLS_StyleScopedClasses['text-xs']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-r-xl']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.button, __VLS_intrinsics.button)({
    ...{ onClick: (...[$event]) => {
            __VLS_ctx.openAction('偏好设置');
            // @ts-ignore
            [openAction,];
        } },
    ...{ class: "menu-btn" },
});
/** @type {__VLS_StyleScopedClasses['menu-btn']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    viewBox: "0 0 24 24",
    ...{ class: "size-4 fill-none stroke-current stroke-2" },
});
/** @type {__VLS_StyleScopedClasses['size-4']} */ ;
/** @type {__VLS_StyleScopedClasses['fill-none']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-current']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-2']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
    'stroke-linecap': "round",
    'stroke-linejoin': "round",
    d: "M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.circle, __VLS_intrinsics.circle)({
    cx: "12",
    cy: "12",
    r: "3",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.button, __VLS_intrinsics.button)({
    ...{ onClick: (__VLS_ctx.appStore.toggleTheme) },
    ...{ class: "menu-btn" },
});
/** @type {__VLS_StyleScopedClasses['menu-btn']} */ ;
if (!__VLS_ctx.appStore.isDark) {
    __VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
        viewBox: "0 0 24 24",
        ...{ class: "size-4" },
    });
    /** @type {__VLS_StyleScopedClasses['size-4']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.circle, __VLS_intrinsics.circle)({
        cx: "12",
        cy: "12",
        r: "5",
        ...{ class: "fill-current" },
    });
    /** @type {__VLS_StyleScopedClasses['fill-current']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.g, __VLS_intrinsics.g)({
        ...{ class: "stroke-current opacity-90" },
        'stroke-width': "2",
    });
    /** @type {__VLS_StyleScopedClasses['stroke-current']} */ ;
    /** @type {__VLS_StyleScopedClasses['opacity-90']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.line, __VLS_intrinsics.line)({
        x1: "12",
        y1: "1",
        x2: "12",
        y2: "3",
    });
    __VLS_asFunctionalElement1(__VLS_intrinsics.line, __VLS_intrinsics.line)({
        x1: "12",
        y1: "21",
        x2: "12",
        y2: "23",
    });
    __VLS_asFunctionalElement1(__VLS_intrinsics.line, __VLS_intrinsics.line)({
        x1: "4.22",
        y1: "4.22",
        x2: "5.64",
        y2: "5.64",
    });
    __VLS_asFunctionalElement1(__VLS_intrinsics.line, __VLS_intrinsics.line)({
        x1: "18.36",
        y1: "18.36",
        x2: "19.78",
        y2: "19.78",
    });
    __VLS_asFunctionalElement1(__VLS_intrinsics.line, __VLS_intrinsics.line)({
        x1: "1",
        y1: "12",
        x2: "3",
        y2: "12",
    });
    __VLS_asFunctionalElement1(__VLS_intrinsics.line, __VLS_intrinsics.line)({
        x1: "21",
        y1: "12",
        x2: "23",
        y2: "12",
    });
    __VLS_asFunctionalElement1(__VLS_intrinsics.line, __VLS_intrinsics.line)({
        x1: "4.22",
        y1: "19.78",
        x2: "5.64",
        y2: "18.36",
    });
    __VLS_asFunctionalElement1(__VLS_intrinsics.line, __VLS_intrinsics.line)({
        x1: "18.36",
        y1: "5.64",
        x2: "19.78",
        y2: "4.22",
    });
}
else {
    __VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
        viewBox: "0 0 24 24",
        ...{ class: "size-4 fill-none stroke-current stroke-2" },
    });
    /** @type {__VLS_StyleScopedClasses['size-4']} */ ;
    /** @type {__VLS_StyleScopedClasses['fill-none']} */ ;
    /** @type {__VLS_StyleScopedClasses['stroke-current']} */ ;
    /** @type {__VLS_StyleScopedClasses['stroke-2']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
        'stroke-linecap': "round",
        'stroke-linejoin': "round",
        d: "M20.354 15.354A9 9 0 0 1 8.646 3.646 9.003 9.003 0 0 0 12 21a9.003 9.003 0 0 0 8.354-5.646z",
    });
}
__VLS_asFunctionalElement1(__VLS_intrinsics.button, __VLS_intrinsics.button)({
    ...{ onClick: (...[$event]) => {
            __VLS_ctx.openAction('多语言切换');
            // @ts-ignore
            [appStore, appStore, openAction,];
        } },
    ...{ class: "menu-btn" },
});
/** @type {__VLS_StyleScopedClasses['menu-btn']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    viewBox: "0 0 24 24",
    ...{ class: "size-4 fill-none stroke-current stroke-2" },
});
/** @type {__VLS_StyleScopedClasses['size-4']} */ ;
/** @type {__VLS_StyleScopedClasses['fill-none']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-current']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-2']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
    'stroke-linecap': "round",
    'stroke-linejoin': "round",
    d: "m5 8 6 6m-7 0 6-6 2-3M2 5h12M7 2h1m14 20-5-10-5 10M14 18h6",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.button, __VLS_intrinsics.button)({
    ...{ onClick: (...[$event]) => {
            __VLS_ctx.openAction('小部件');
            // @ts-ignore
            [openAction,];
        } },
    ...{ class: "menu-btn" },
});
/** @type {__VLS_StyleScopedClasses['menu-btn']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    viewBox: "0 0 2048 2048",
    ...{ class: "size-4 fill-current" },
});
/** @type {__VLS_StyleScopedClasses['size-4']} */ ;
/** @type {__VLS_StyleScopedClasses['fill-current']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
    d: "M896 768H512V256h128v384h256zm1152 640q0 87-22 168t-64 152t-100 130t-128 101t-152 66t-168 23q-134 0-251-49t-205-136t-139-204t-51-251q0-132 50-248t138-204t203-137t249-51q132 0 248 50t204 138t137 203t51 249m-640 512q21 0 37-15t29-40t21-53t15-58t9-53t5-37h-230q1 13 5 37t10 52t15 58t21 54t27 39t36 16m125-384q3-64 3-128q0-63-3-128h-250q-3 65-3 128q0 64 3 128zm-637-128q0 32 4 64t12 64h243q-6-128 0-256H912q-8 32-12 64t-4 64m512-512q-19 0-34 15t-27 40t-21 54t-15 58t-11 53t-5 36h225q-1-11-5-34t-11-52t-16-59t-21-54t-27-41t-32-16m253 384q3 64 3 128t-2 128h242q8-32 12-64t4-64t-4-64t-12-64zm190-128q-43-75-108-131t-145-89q20 53 32 108t20 112zm-637-218q-78 32-142 88t-107 130h200q7-56 18-110t31-108m-249 730q42 73 105 129t142 88q-20-52-30-107t-17-110zm643 215q77-32 139-87t104-128h-198q-5 55-15 109t-30 106M640 0q88 0 170 23t153 64t129 100t100 130t65 153t23 170h-128q0-106-40-199t-110-162t-163-110t-199-41t-199 40t-162 110t-110 163t-41 199t40 199t110 162t163 110t199 41v128q-88 0-170-23t-153-64t-129-100T88 963T23 810T0 640q0-132 50-248t138-204T391 51T640 0",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.button, __VLS_intrinsics.button)({
    ...{ onClick: (__VLS_ctx.toggleFullscreen) },
    ...{ class: "menu-btn" },
});
/** @type {__VLS_StyleScopedClasses['menu-btn']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    viewBox: "0 0 24 24",
    ...{ class: "size-4 fill-none stroke-current stroke-2" },
});
/** @type {__VLS_StyleScopedClasses['size-4']} */ ;
/** @type {__VLS_StyleScopedClasses['fill-none']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-current']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-2']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
    'stroke-linecap': "round",
    'stroke-linejoin': "round",
    d: "M8 3H5a2 2 0 0 0-2 2v3M21 8V5a2 2 0 0 0-2-2h-3M3 16v3a2 2 0 0 0 2 2h3M16 21h3a2 2 0 0 0 2-2v-3",
});
__VLS_asFunctionalElement1(__VLS_intrinsics.button, __VLS_intrinsics.button)({
    ...{ onClick: (...[$event]) => {
            __VLS_ctx.openAction('通知中心');
            // @ts-ignore
            [openAction, toggleFullscreen,];
        } },
    ...{ class: "menu-btn relative mx-1" },
});
/** @type {__VLS_StyleScopedClasses['menu-btn']} */ ;
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
/** @type {__VLS_StyleScopedClasses['mx-1']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "absolute top-1.5 right-1.5 size-2 rounded-sm bg-[#0960bd]" },
});
/** @type {__VLS_StyleScopedClasses['absolute']} */ ;
/** @type {__VLS_StyleScopedClasses['top-1.5']} */ ;
/** @type {__VLS_StyleScopedClasses['right-1.5']} */ ;
/** @type {__VLS_StyleScopedClasses['size-2']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-sm']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-[#0960bd]']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
    viewBox: "0 0 24 24",
    ...{ class: "size-4 fill-none stroke-current stroke-2" },
});
/** @type {__VLS_StyleScopedClasses['size-4']} */ ;
/** @type {__VLS_StyleScopedClasses['fill-none']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-current']} */ ;
/** @type {__VLS_StyleScopedClasses['stroke-2']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
    'stroke-linecap': "round",
    'stroke-linejoin': "round",
    d: "M10.268 21a2 2 0 0 0 3.464 0M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326",
});
let __VLS_5;
/** @ts-ignore @type {typeof __VLS_components.aDropdown | typeof __VLS_components.ADropdown | typeof __VLS_components.aDropdown | typeof __VLS_components.ADropdown} */
aDropdown;
// @ts-ignore
const __VLS_6 = __VLS_asFunctionalComponent1(__VLS_5, new __VLS_5({
    placement: "bottomRight",
}));
const __VLS_7 = __VLS_6({
    placement: "bottomRight",
}, ...__VLS_functionalComponentArgsRest(__VLS_6));
const { default: __VLS_10 } = __VLS_8.slots;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "mr-2 cursor-pointer rounded-full p-1.5 hover:bg-slate-100 dark:hover:bg-[#2A2D33]" },
});
/** @type {__VLS_StyleScopedClasses['mr-2']} */ ;
/** @type {__VLS_StyleScopedClasses['cursor-pointer']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
/** @type {__VLS_StyleScopedClasses['p-1.5']} */ ;
/** @type {__VLS_StyleScopedClasses['hover:bg-slate-100']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:hover:bg-[#2A2D33]']} */ ;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "relative flex items-center h-8 w-8 shrink-0" },
});
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
/** @type {__VLS_StyleScopedClasses['flex']} */ ;
/** @type {__VLS_StyleScopedClasses['items-center']} */ ;
/** @type {__VLS_StyleScopedClasses['h-8']} */ ;
/** @type {__VLS_StyleScopedClasses['w-8']} */ ;
/** @type {__VLS_StyleScopedClasses['shrink-0']} */ ;
let __VLS_11;
/** @ts-ignore @type {typeof __VLS_components.aAvatar | typeof __VLS_components.AAvatar | typeof __VLS_components.aAvatar | typeof __VLS_components.AAvatar} */
aAvatar;
// @ts-ignore
const __VLS_12 = __VLS_asFunctionalComponent1(__VLS_11, new __VLS_11({
    size: "small",
    ...{ class: "!h-8 !w-8 bg-slate-200 text-slate-700 dark:bg-slate-700 dark:text-slate-100 rounded-full" },
}));
const __VLS_13 = __VLS_12({
    size: "small",
    ...{ class: "!h-8 !w-8 bg-slate-200 text-slate-700 dark:bg-slate-700 dark:text-slate-100 rounded-full" },
}, ...__VLS_functionalComponentArgsRest(__VLS_12));
/** @type {__VLS_StyleScopedClasses['!h-8']} */ ;
/** @type {__VLS_StyleScopedClasses['!w-8']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-slate-200']} */ ;
/** @type {__VLS_StyleScopedClasses['text-slate-700']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:bg-slate-700']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:text-slate-100']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
const { default: __VLS_16 } = __VLS_14.slots;
{
    const { icon: __VLS_17 } = __VLS_14.slots;
    let __VLS_18;
    /** @ts-ignore @type {typeof __VLS_components.UserOutlined} */
    UserOutlined;
    // @ts-ignore
    const __VLS_19 = __VLS_asFunctionalComponent1(__VLS_18, new __VLS_18({
        ...{ class: "text-[13px]" },
    }));
    const __VLS_20 = __VLS_19({
        ...{ class: "text-[13px]" },
    }, ...__VLS_functionalComponentArgsRest(__VLS_19));
    /** @type {__VLS_StyleScopedClasses['text-[13px]']} */ ;
    // @ts-ignore
    [];
}
// @ts-ignore
[];
var __VLS_14;
__VLS_asFunctionalElement1(__VLS_intrinsics.span, __VLS_intrinsics.span)({
    ...{ class: "absolute right-0 bottom-0 h-3 w-3 rounded-full border-2 border-white bg-green-500 dark:border-[#1C1E22]" },
});
/** @type {__VLS_StyleScopedClasses['absolute']} */ ;
/** @type {__VLS_StyleScopedClasses['right-0']} */ ;
/** @type {__VLS_StyleScopedClasses['bottom-0']} */ ;
/** @type {__VLS_StyleScopedClasses['h-3']} */ ;
/** @type {__VLS_StyleScopedClasses['w-3']} */ ;
/** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
/** @type {__VLS_StyleScopedClasses['border-2']} */ ;
/** @type {__VLS_StyleScopedClasses['border-white']} */ ;
/** @type {__VLS_StyleScopedClasses['bg-green-500']} */ ;
/** @type {__VLS_StyleScopedClasses['dark:border-[#1C1E22]']} */ ;
{
    const { overlay: __VLS_23 } = __VLS_8.slots;
    let __VLS_24;
    /** @ts-ignore @type {typeof __VLS_components.aMenu | typeof __VLS_components.AMenu | typeof __VLS_components.aMenu | typeof __VLS_components.AMenu} */
    aMenu;
    // @ts-ignore
    const __VLS_25 = __VLS_asFunctionalComponent1(__VLS_24, new __VLS_24({}));
    const __VLS_26 = __VLS_25({}, ...__VLS_functionalComponentArgsRest(__VLS_25));
    const { default: __VLS_29 } = __VLS_27.slots;
    let __VLS_30;
    /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
    aMenuItem;
    // @ts-ignore
    const __VLS_31 = __VLS_asFunctionalComponent1(__VLS_30, new __VLS_30({
        ...{ 'onClick': {} },
        key: "preferences",
    }));
    const __VLS_32 = __VLS_31({
        ...{ 'onClick': {} },
        key: "preferences",
    }, ...__VLS_functionalComponentArgsRest(__VLS_31));
    let __VLS_35;
    const __VLS_36 = ({ click: {} },
        { onClick: (...[$event]) => {
                __VLS_ctx.openAction('偏好设置');
                // @ts-ignore
                [openAction,];
            } });
    const { default: __VLS_37 } = __VLS_33.slots;
    {
        const { icon: __VLS_38 } = __VLS_33.slots;
        let __VLS_39;
        /** @ts-ignore @type {typeof __VLS_components.SettingOutlined} */
        SettingOutlined;
        // @ts-ignore
        const __VLS_40 = __VLS_asFunctionalComponent1(__VLS_39, new __VLS_39({}));
        const __VLS_41 = __VLS_40({}, ...__VLS_functionalComponentArgsRest(__VLS_40));
        // @ts-ignore
        [];
    }
    // @ts-ignore
    [];
    var __VLS_33;
    var __VLS_34;
    let __VLS_44;
    /** @ts-ignore @type {typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem | typeof __VLS_components.aMenuItem | typeof __VLS_components.AMenuItem} */
    aMenuItem;
    // @ts-ignore
    const __VLS_45 = __VLS_asFunctionalComponent1(__VLS_44, new __VLS_44({
        ...{ 'onClick': {} },
        key: "logout",
    }));
    const __VLS_46 = __VLS_45({
        ...{ 'onClick': {} },
        key: "logout",
    }, ...__VLS_functionalComponentArgsRest(__VLS_45));
    let __VLS_49;
    const __VLS_50 = ({ click: {} },
        { onClick: (...[$event]) => {
                __VLS_ctx.router.push('/login');
                // @ts-ignore
                [router,];
            } });
    const { default: __VLS_51 } = __VLS_47.slots;
    {
        const { icon: __VLS_52 } = __VLS_47.slots;
        let __VLS_53;
        /** @ts-ignore @type {typeof __VLS_components.LogoutOutlined} */
        LogoutOutlined;
        // @ts-ignore
        const __VLS_54 = __VLS_asFunctionalComponent1(__VLS_53, new __VLS_53({}));
        const __VLS_55 = __VLS_54({}, ...__VLS_functionalComponentArgsRest(__VLS_54));
        // @ts-ignore
        [];
    }
    // @ts-ignore
    [];
    var __VLS_47;
    var __VLS_48;
    // @ts-ignore
    [];
    var __VLS_27;
    // @ts-ignore
    [];
}
// @ts-ignore
[];
var __VLS_8;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({
    emits: {},
    __typeProps: {},
});
export default {};
