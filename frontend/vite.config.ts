import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import tailwindcss from '@tailwindcss/vite'
import ui from '@nuxt/ui/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
    tailwindcss(),
    ui({
      ui: {
        colors: {
          primary: 'rust', // custom name OK for primary
          neutral: 'slate', // must be a built-in name; we redefined slate's values
        },
        button: {
          slots: { base: 'font-semibold rounded-lg transition-colors' },
          defaultVariants: { color: 'primary', variant: 'solid', size: 'md' },
        },
        card: {
          slots: {
            root: 'rounded-lg bg-elevated ring ring-default',
            header: 'border-b border-default px-5 py-4',
            body: 'p-5',
          },
        },
        input: {
          slots: { base: 'rounded-lg bg-muted border-default text-default' },
        },
      },
    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
})
