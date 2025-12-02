import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { setupGlobalNotifications } from "./composables/useNotifications";
import { initializeConsoleMessages, setupGlobalConsoleLogger } from "./composables/useConsoleMessages";
import { setupI18n } from "./locales";

// 全局错误捕获已移至 useConsoleMessages.ts 中统一处理

const app = createApp(App);

app.use(router);

// 设置全局通知系统
setupGlobalNotifications();

// 设置全局日志系统
setupGlobalConsoleLogger();

// 初始化全局控制台消息管理器
initializeConsoleMessages();

// 设置国际化
setupI18n();

app.mount("#app");
