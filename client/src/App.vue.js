/// <reference types="../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { computed } from 'vue';
import { theme } from 'ant-design-vue';
import { useAppStore } from '@/store/app';
import { useEventStore } from '@/store/events';
const appStore = useAppStore();
// Initialize event store to start watching connection profile changes
const eventStore = useEventStore();
const themeConfig = computed(() => ({
    algorithm: appStore.isDark ? theme.darkAlgorithm : theme.defaultAlgorithm,
    token: {
        colorPrimary: '#0960bd',
    },
}));
const __VLS_ctx = {
    ...{},
    ...{},
};
let __VLS_components;
let __VLS_intrinsics;
let __VLS_directives;
let __VLS_0;
/** @ts-ignore @type {typeof __VLS_components.aConfigProvider | typeof __VLS_components.AConfigProvider | typeof __VLS_components.aConfigProvider | typeof __VLS_components.AConfigProvider} */
aConfigProvider;
// @ts-ignore
const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
    theme: (__VLS_ctx.themeConfig),
}));
const __VLS_2 = __VLS_1({
    theme: (__VLS_ctx.themeConfig),
}, ...__VLS_functionalComponentArgsRest(__VLS_1));
var __VLS_5 = {};
const { default: __VLS_6 } = __VLS_3.slots;
let __VLS_7;
/** @ts-ignore @type {typeof __VLS_components.routerView | typeof __VLS_components.RouterView} */
routerView;
// @ts-ignore
const __VLS_8 = __VLS_asFunctionalComponent1(__VLS_7, new __VLS_7({}));
const __VLS_9 = __VLS_8({}, ...__VLS_functionalComponentArgsRest(__VLS_8));
// @ts-ignore
[themeConfig,];
var __VLS_3;
// @ts-ignore
[];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
