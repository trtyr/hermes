import type { App } from 'vue';
import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import { useConnectionStore } from '@/store/connection';
import { checkSession } from '@/api/connection';

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

// Track whether we've validated the stored session this app lifecycle.
let isSessionVerified = false;

// Navigation guard
router.beforeEach(async (to, _from, next) => {
  const connectionStore = useConnectionStore();

  // Login page: if there's an active profile, redirect away.
  if (to.path === '/login') {
    if (connectionStore.activeProfile) {
      next('/dashboard');
      return;
    }
    next();
    return;
  }

  // Protected routes: must have a profile
  if (!connectionStore.activeProfile) {
    next('/login');
    return;
  }

  // Verify session on first visit to a protected route this lifecycle.
  if (!isSessionVerified) {
    const valid = await checkSession(
      connectionStore.activeProfile.server_url,
      connectionStore.activeProfile.api_token
    );

    if (!valid) {
      connectionStore.logout();
      isSessionVerified = false;
      next('/login');
      return;
    }

    isSessionVerified = true;
  }

  next();
});

export function resetSessionVerification() {
  isSessionVerified = false;
}

export function setupRouter(app: App<Element>) {
  app.use(router);
}
