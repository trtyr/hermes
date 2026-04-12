import { createApp } from 'vue';
import App from './App.vue';
import { setupRouter } from './router';
import { createPinia } from 'pinia';
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css'; // For Ant Design Vue v4+
import './style.css';

async function bootstrap() {
  const app = createApp(App);

  const pinia = createPinia();
  app.use(pinia);

  app.use(Antd);

  setupRouter(app);

  app.mount('#app');
}

bootstrap();