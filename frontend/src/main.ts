import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import './style.css'
import ui from '@nuxt/ui/vue-plugin'
import hljs from 'highlight.js/lib/core'
import hljsVuePlugin from '@highlightjs/vue-plugin'

import json from 'highlight.js/lib/languages/json'
import http from 'highlight.js/lib/languages/http'

const app = createApp(App)

hljs.registerLanguage('json', json)
hljs.registerLanguage('http', http)

app.use(router)
app.use(ui)
app.use(hljsVuePlugin)

app.mount('#app')
