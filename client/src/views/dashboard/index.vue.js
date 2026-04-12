/// <reference types="../../../node_modules/@vue/language-core/types/template-helpers.d.ts" />
/// <reference types="../../../node_modules/@vue/language-core/types/props-fallback.d.ts" />
import { ref, onMounted, watch } from 'vue';
import { message } from 'ant-design-vue';
// Subcomponents
import UnconnectedState from './components/UnconnectedState.vue';
import ConnectionBadge from './components/ConnectionBadge.vue';
import ConnectionModal from './components/ConnectionModal.vue';
import TopStatsGrid from './components/TopStatsGrid.vue';
import ServerInfoCard from './components/ServerInfoCard.vue';
import AgentsDistCard from './components/AgentsDistCard.vue';
import ListenersDistCard from './components/ListenersDistCard.vue';
// State and Networking
import { useConnectionStore } from '@/store/connection';
import { useEventStore } from '@/store/events';
import { fetchDashboardStats } from '@/api/dashboard';
const connectionStore = useConnectionStore();
const eventStore = useEventStore();
const showConnectionModal = ref(false);
const stats = ref(null);
const loading = ref(false);
const error = ref('');
async function loadStats() {
    if (!connectionStore.activeProfile)
        return;
    loading.value = true;
    error.value = '';
    try {
        stats.value = await fetchDashboardStats();
    }
    catch (err) {
        console.error('Failed to load dashboard stats:', err);
        error.value = err.message || '网络请求失败，请检查后端是否正常运行。';
        if (err.message && err.message.includes('Token')) {
            message.error(error.value);
            showConnectionModal.value = true;
        }
    }
    finally {
        loading.value = false;
    }
}
watch(() => connectionStore.activeProfileId, (newId) => {
    if (newId) {
        loadStats();
    }
    else {
        stats.value = null;
    }
});
onMounted(() => {
    if (connectionStore.activeProfile) {
        loadStats();
    }
});
const __VLS_ctx = {
    ...{},
    ...{},
};
let __VLS_components;
let __VLS_intrinsics;
let __VLS_directives;
__VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
    ...{ class: "h-full w-full p-4 relative" },
});
/** @type {__VLS_StyleScopedClasses['h-full']} */ ;
/** @type {__VLS_StyleScopedClasses['w-full']} */ ;
/** @type {__VLS_StyleScopedClasses['p-4']} */ ;
/** @type {__VLS_StyleScopedClasses['relative']} */ ;
if (!__VLS_ctx.connectionStore.activeProfile) {
    const __VLS_0 = UnconnectedState;
    // @ts-ignore
    const __VLS_1 = __VLS_asFunctionalComponent1(__VLS_0, new __VLS_0({
        ...{ 'onConnect': {} },
    }));
    const __VLS_2 = __VLS_1({
        ...{ 'onConnect': {} },
    }, ...__VLS_functionalComponentArgsRest(__VLS_1));
    let __VLS_5;
    const __VLS_6 = ({ connect: {} },
        { onConnect: (...[$event]) => {
                if (!(!__VLS_ctx.connectionStore.activeProfile))
                    return;
                __VLS_ctx.showConnectionModal = true;
                // @ts-ignore
                [connectionStore, showConnectionModal,];
            } });
    var __VLS_3;
    var __VLS_4;
}
else {
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "h-full flex flex-col" },
    });
    /** @type {__VLS_StyleScopedClasses['h-full']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
        ...{ class: "flex justify-between items-center mb-6" },
    });
    /** @type {__VLS_StyleScopedClasses['flex']} */ ;
    /** @type {__VLS_StyleScopedClasses['justify-between']} */ ;
    /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
    /** @type {__VLS_StyleScopedClasses['mb-6']} */ ;
    __VLS_asFunctionalElement1(__VLS_intrinsics.h2, __VLS_intrinsics.h2)({
        ...{ class: "text-2xl font-semibold text-slate-800 dark:text-slate-100" },
    });
    /** @type {__VLS_StyleScopedClasses['text-2xl']} */ ;
    /** @type {__VLS_StyleScopedClasses['font-semibold']} */ ;
    /** @type {__VLS_StyleScopedClasses['text-slate-800']} */ ;
    /** @type {__VLS_StyleScopedClasses['dark:text-slate-100']} */ ;
    const __VLS_7 = ConnectionBadge;
    // @ts-ignore
    const __VLS_8 = __VLS_asFunctionalComponent1(__VLS_7, new __VLS_7({
        ...{ 'onManage': {} },
    }));
    const __VLS_9 = __VLS_8({
        ...{ 'onManage': {} },
    }, ...__VLS_functionalComponentArgsRest(__VLS_8));
    let __VLS_12;
    const __VLS_13 = ({ manage: {} },
        { onManage: (...[$event]) => {
                if (!!(!__VLS_ctx.connectionStore.activeProfile))
                    return;
                __VLS_ctx.showConnectionModal = true;
                // @ts-ignore
                [showConnectionModal,];
            } });
    var __VLS_10;
    var __VLS_11;
    if (__VLS_ctx.loading) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "flex-1 flex flex-col items-center justify-center" },
        });
        /** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
        let __VLS_14;
        /** @ts-ignore @type {typeof __VLS_components.aSpin | typeof __VLS_components.ASpin} */
        aSpin;
        // @ts-ignore
        const __VLS_15 = __VLS_asFunctionalComponent1(__VLS_14, new __VLS_14({
            size: "large",
        }));
        const __VLS_16 = __VLS_15({
            size: "large",
        }, ...__VLS_functionalComponentArgsRest(__VLS_15));
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "mt-4 text-slate-400" },
        });
        /** @type {__VLS_StyleScopedClasses['mt-4']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-slate-400']} */ ;
    }
    else if (__VLS_ctx.error) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "flex-1 flex flex-col items-center justify-center text-center" },
        });
        /** @type {__VLS_StyleScopedClasses['flex-1']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex-col']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-center']} */ ;
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "w-16 h-16 bg-red-50 text-red-500 rounded-full flex items-center justify-center mb-4" },
        });
        /** @type {__VLS_StyleScopedClasses['w-16']} */ ;
        /** @type {__VLS_StyleScopedClasses['h-16']} */ ;
        /** @type {__VLS_StyleScopedClasses['bg-red-50']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-red-500']} */ ;
        /** @type {__VLS_StyleScopedClasses['rounded-full']} */ ;
        /** @type {__VLS_StyleScopedClasses['flex']} */ ;
        /** @type {__VLS_StyleScopedClasses['items-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['justify-center']} */ ;
        /** @type {__VLS_StyleScopedClasses['mb-4']} */ ;
        __VLS_asFunctionalElement1(__VLS_intrinsics.svg, __VLS_intrinsics.svg)({
            ...{ class: "w-8 h-8" },
            fill: "none",
            stroke: "currentColor",
            viewBox: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
        });
        /** @type {__VLS_StyleScopedClasses['w-8']} */ ;
        /** @type {__VLS_StyleScopedClasses['h-8']} */ ;
        __VLS_asFunctionalElement1(__VLS_intrinsics.path, __VLS_intrinsics.path)({
            'stroke-linecap': "round",
            'stroke-linejoin': "round",
            'stroke-width': "2",
            d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
        });
        __VLS_asFunctionalElement1(__VLS_intrinsics.h2, __VLS_intrinsics.h2)({
            ...{ class: "text-xl font-semibold text-slate-800 dark:text-slate-100 mb-2" },
        });
        /** @type {__VLS_StyleScopedClasses['text-xl']} */ ;
        /** @type {__VLS_StyleScopedClasses['font-semibold']} */ ;
        /** @type {__VLS_StyleScopedClasses['text-slate-800']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-slate-100']} */ ;
        /** @type {__VLS_StyleScopedClasses['mb-2']} */ ;
        __VLS_asFunctionalElement1(__VLS_intrinsics.p, __VLS_intrinsics.p)({
            ...{ class: "text-slate-500 dark:text-slate-400 mb-4 max-w-md" },
        });
        /** @type {__VLS_StyleScopedClasses['text-slate-500']} */ ;
        /** @type {__VLS_StyleScopedClasses['dark:text-slate-400']} */ ;
        /** @type {__VLS_StyleScopedClasses['mb-4']} */ ;
        /** @type {__VLS_StyleScopedClasses['max-w-md']} */ ;
        (__VLS_ctx.error);
        let __VLS_19;
        /** @ts-ignore @type {typeof __VLS_components.aButton | typeof __VLS_components.AButton | typeof __VLS_components.aButton | typeof __VLS_components.AButton} */
        aButton;
        // @ts-ignore
        const __VLS_20 = __VLS_asFunctionalComponent1(__VLS_19, new __VLS_19({
            ...{ 'onClick': {} },
        }));
        const __VLS_21 = __VLS_20({
            ...{ 'onClick': {} },
        }, ...__VLS_functionalComponentArgsRest(__VLS_20));
        let __VLS_24;
        const __VLS_25 = ({ click: {} },
            { onClick: (__VLS_ctx.loadStats) });
        const { default: __VLS_26 } = __VLS_22.slots;
        // @ts-ignore
        [loading, error, error, loadStats,];
        var __VLS_22;
        var __VLS_23;
    }
    else if (__VLS_ctx.stats) {
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "space-y-6" },
        });
        /** @type {__VLS_StyleScopedClasses['space-y-6']} */ ;
        const __VLS_27 = TopStatsGrid;
        // @ts-ignore
        const __VLS_28 = __VLS_asFunctionalComponent1(__VLS_27, new __VLS_27({
            stats: (__VLS_ctx.stats),
        }));
        const __VLS_29 = __VLS_28({
            stats: (__VLS_ctx.stats),
        }, ...__VLS_functionalComponentArgsRest(__VLS_28));
        __VLS_asFunctionalElement1(__VLS_intrinsics.div, __VLS_intrinsics.div)({
            ...{ class: "grid grid-cols-1 lg:grid-cols-3 gap-6" },
        });
        /** @type {__VLS_StyleScopedClasses['grid']} */ ;
        /** @type {__VLS_StyleScopedClasses['grid-cols-1']} */ ;
        /** @type {__VLS_StyleScopedClasses['lg:grid-cols-3']} */ ;
        /** @type {__VLS_StyleScopedClasses['gap-6']} */ ;
        const __VLS_32 = ServerInfoCard;
        // @ts-ignore
        const __VLS_33 = __VLS_asFunctionalComponent1(__VLS_32, new __VLS_32({
            stats: (__VLS_ctx.stats),
        }));
        const __VLS_34 = __VLS_33({
            stats: (__VLS_ctx.stats),
        }, ...__VLS_functionalComponentArgsRest(__VLS_33));
        const __VLS_37 = AgentsDistCard;
        // @ts-ignore
        const __VLS_38 = __VLS_asFunctionalComponent1(__VLS_37, new __VLS_37({
            stats: (__VLS_ctx.stats),
        }));
        const __VLS_39 = __VLS_38({
            stats: (__VLS_ctx.stats),
        }, ...__VLS_functionalComponentArgsRest(__VLS_38));
        const __VLS_42 = ListenersDistCard;
        // @ts-ignore
        const __VLS_43 = __VLS_asFunctionalComponent1(__VLS_42, new __VLS_42({
            stats: (__VLS_ctx.stats),
        }));
        const __VLS_44 = __VLS_43({
            stats: (__VLS_ctx.stats),
        }, ...__VLS_functionalComponentArgsRest(__VLS_43));
    }
}
const __VLS_47 = ConnectionModal;
// @ts-ignore
const __VLS_48 = __VLS_asFunctionalComponent1(__VLS_47, new __VLS_47({
    visible: (__VLS_ctx.showConnectionModal),
}));
const __VLS_49 = __VLS_48({
    visible: (__VLS_ctx.showConnectionModal),
}, ...__VLS_functionalComponentArgsRest(__VLS_48));
// @ts-ignore
[showConnectionModal, stats, stats, stats, stats, stats,];
const __VLS_export = (await import('vue')).defineComponent({});
export default {};
