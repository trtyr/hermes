import { defineStore } from 'pinia';
export const useAppStore = defineStore('app', {
    state: () => ({
        isDark: false,
        sidebarCollapsed: false,
        visitedViews: [],
    }),
    actions: {
        toggleTheme() {
            this.isDark = !this.isDark;
            if (this.isDark) {
                document.documentElement.classList.add('dark');
            }
            else {
                document.documentElement.classList.remove('dark');
            }
        },
        toggleCollapse() {
            this.sidebarCollapsed = !this.sidebarCollapsed;
        },
        addView(view) {
            if (this.visitedViews.some(v => v.path === view.path))
                return;
            this.visitedViews.push(view);
        },
        removeView(path) {
            this.visitedViews = this.visitedViews.filter(v => v.path !== path);
        }
    }
});
