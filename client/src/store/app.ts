import { defineStore } from 'pinia';

export interface TabView {
  path: string;
  name: string;
  title: string;
}

export const useAppStore = defineStore('app', {
  state: () => ({
    sidebarCollapsed: false,
    visitedViews: [] as TabView[],
  }),
  actions: {
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
