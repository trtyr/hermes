import { defineStore } from 'pinia';

export interface TabView {
  path: string;
  name: string;
  title: string;
}

export const useAppStore = defineStore('app', {
  state: () => ({
    isDark: false,
    sidebarCollapsed: false,
    visitedViews: [] as TabView[],
  }),
  actions: {
    toggleTheme() {
      this.isDark = !this.isDark;
      if (this.isDark) {
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.classList.remove('dark');
      }
    },
    toggleCollapse() {
      this.sidebarCollapsed = !this.sidebarCollapsed;
    },
    addView(view: TabView) {
      if (this.visitedViews.some(v => v.path === view.path)) return;
      this.visitedViews.push(view);
    },
    removeView(path: string) {
      this.visitedViews = this.visitedViews.filter(v => v.path !== path);
    }
  }
});
