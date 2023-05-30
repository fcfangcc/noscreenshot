import 'element-plus/dist/index.css'
import { createApp } from 'vue'
import App from './App.vue'
import { logger } from './command'
import router from './router'
import './styles.css'
import i18n from '@/locales/i18n'

const app = createApp(App)
// todo: get locale from tauri then change locale
i18n.global.locale.value = 'en'

app.use(router)
app.use(i18n)

app.mount('#app')

// eslint-disable-next-line @typescript-eslint/no-explicit-any
app.config.errorHandler = (err: any, _, info) => {
  logger.error(`err: ${err.stack}, info: ${info}`)
  throw err
}
