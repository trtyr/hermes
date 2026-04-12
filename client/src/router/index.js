import { createRouter, createWebHistory } from 'vue-router';
const routes = [
    {
        path: '/login',
        name: 'Login',
        component: () => import('../views/sys/login/Login.vue'),
        meta: {
            title: 'Login - Hermes C2',
        },
    },
    {
        path: '/',
        name: 'Layout',
        component: () => import('../layouts/default/index.vue'),
        redirect: '/dashboard',
        children: [
            {
                path: 'dashboard',
                name: 'Dashboard',
                component: () => import('../views/dashboard/index.vue'),
                meta: { title: '总览 - Hermes C2' },
            },
            {
                path: 'agent',
                name: 'Agent',
                component: () => import('../views/agent/index.vue'),
                meta: { title: '节点管理 - Hermes C2' },
            },
            {
                path: 'listener',
                name: 'Listener',
                component: () => import('../views/listener/index.vue'),
                meta: { title: '监听器管理 - Hermes C2' },
            },
            {
                path: 'payload',
                name: 'Payload',
                component: () => import('../views/payload/index.vue'),
                meta: { title: '载荷生成 - Hermes C2' },
            },
            {
                path: 'log',
                name: 'Log',
                component: () => import('../views/log/index.vue'),
                meta: { title: '操作日志 - Hermes C2' },
            },
            {
                path: 'agent/terminal/:id',
                name: 'AgentTerminal',
                component: () => import('../views/agent/terminal.vue'),
                meta: { title: '终端控制面板' },
            },
        ]
    }
];
export const router = createRouter({
    history: createWebHistory(),
    routes,
});
export function setupRouter(app) {
    app.use(router);
}
