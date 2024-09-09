import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import i18n from './i18n/index'
import './assets/styles.css'

const app = createApp(App);
const pinia = createPinia()

app.use(i18n);
app.use(pinia);

app.mount('#app');