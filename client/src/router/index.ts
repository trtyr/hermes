import type { App } from 'vue';
import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import { useConnectionStore } from '@/store/connection';

const routes: Array<RouteRecordRaw> = [
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

// Navigation guard: redirect to login if not authenticated
router.beforeEach((to, _from, next) => {
  const connectionStore = useConnectionStore();
  const isAuthenticated = connectionStore.activeProfile !== null;

  if (to.path !== '/login' && !isAuthenticated) {
    next('/login');
  } else if (to.path === '/login' && isAuthenticated) {
    next('/dashboard');
  } else {
    next();
  }
});

export function setupRouter(app: App<Element>) {
  app.use(router);
}
