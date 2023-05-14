import '@mdi/font/css/materialdesignicons.css'
import { App } from 'vue'
import { createVuetify } from 'vuetify'
import { aliases, mdi } from 'vuetify/iconsets/mdi'

const vuetify = createVuetify({
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi
    }
  }
})

export default function setVuetify(app: App) {
  app.use(vuetify)
}
