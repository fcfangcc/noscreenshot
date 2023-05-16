import "element-plus/dist/index.css";
import { createApp } from "vue";
import App from "./App.vue";
import { logger } from "./command";
import router from "./router";
import "./styles.css";

const app = createApp(App);

app.use(router);

app.mount("#app");

// eslint-disable-next-line @typescript-eslint/no-explicit-any
app.config.errorHandler = (err: any, _, info) => {
  logger.error(`err: ${err.stack}, info: ${info}`);
  throw err;
};
