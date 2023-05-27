import devtools from '@vue/devtools'
import {createApp} from 'vue'
import App from './App.vue'

import router from "@/router/index"

if (process.env.NODE_ENV === 'development') {
    devtools.connect('http://localhost', 8098)
}

const app = createApp(App)
app.use(router)
router.isReady().then(() => {
    app.mount('#app')
})
