import './assets/main.css'

import { createApp } from 'vue'

import App from './App.vue'
import { setupRouter } from './router'
import { setupStore } from './stores/counter'
import setVuetify from './plugins/vuetify'

async function bootstrap() {
  const app = createApp(App)

  setVuetify(app)
  setupStore(app)
  await setupRouter(app)

  app.mount('#app')
}

bootstrap()
