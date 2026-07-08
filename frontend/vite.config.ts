import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import tailwindcss from '@tailwindcss/vite'
import ui from '@nuxt/ui/vite'

// https://vite.dev/config/
export default defineConfig({
  envDir: '../',
  plugins: [
    vue(),
    vueDevTools(),
    tailwindcss(),
    ui({
      ui: {
        colors: {
          primary: 'rust',
          neutral: 'slate',
        },
        button: {
          slots: { base: 'font-semibold rounded-lg transition-colors' },
          defaultVariants: { color: 'primary', variant: 'solid', size: 'md' },
        },
        card: {
          slots: {
            root: 'rounded-lg  ring ring-default',
            header: 'border-b border-default px-5 py-4',
            body: 'p-5 bg-elevated/50',
          },
        },
        input: {
          slots: { base: 'rounded-lg bg-muted border-default text-default' },
        },
        select: {
          slots: {
            base: 'rounded-lg bg-muted border-default text-default',
            content: 'rounded-lg bg-elevated border-muted shadow-lg',
          },
        },

        selectMenu: {
          slots: {
            base: 'rounded-lg bg-muted  border-default text-default',
            content: 'rounded-lg bg-elevated border-muted shadow-lg',
            item: 'rounded-md data-highlighted:bg-accented',
          },
        },

        fileUpload: {
          slots: {
            base: 'rounded-xl border-2 border-dashed border-default bg-muted/50 hover:bg-accented/40 transition-colors',
            file: 'rounded-lg bg-elevated border-muted',
          },
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
