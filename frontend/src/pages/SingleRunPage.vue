<script setup lang="ts">
import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'
import type { BreadcrumbItem, TabsItem } from '@nuxt/ui'
import { useRoute } from 'vue-router'
import { AnsiUp } from 'ansi_up'
import { computed, ref } from 'vue'
const route = useRoute()
const ansiUP = new AnsiUp()

const rawOutput =
  ref(`\x1b[36m10:42:01.032\x1b[0m \x1b[34mINFO\x1b[0m cold sandbox initialized in 1.2ms
\x1b[36m10:42:01.034\x1b[0m \x1b[34mINFO\x1b[0m fetching source_url s3://uploads/photo.jpg
\x1b[36m10:42:01.041\x1b[0m \x1b[34mINFO\x1b[0m decoded JPEG 3024×4032
\x1b[36m10:42:01.048\x1b[0m \x1b[33mWARN\x1b[0m source larger than 3000px, downscaling before resize
\x1b[36m10:42:01.052\x1b[0m \x1b[34mINFO\x1b[0m resized to 800×600
\x1b[36m10:42:01.059\x1b[0m \x1b[34mINFO\x1b[0m encoded output as webp, 214 KB
\x1b[36m10:42:01.061\x1b[0m \x1b[32mDONE\x1b[0m execution finished · 14ms total`)

const interpretedOutput = computed(() => ansiUP.ansi_to_html(rawOutput.value))

const breadcrumbItems: BreadcrumbItem[] = [
  {
    label: 'Functions',
    to: '/functions',
  },
  {
    label: route.params.id as string,
  },
]

const tabs: TabsItem[] = [
  {
    label: 'Logs',
    slot: 'logs' as const,
  },
  { label: 'Response', slot: 'response' as const },
  { label: 'Errors', slot: 'errors' as const },
]

const payload = JSON.stringify(
  {
    token: 'eyJhbGciOiJIUzI1NiJ9.eyJhbGciOiJIUzI1NiJ9',
    audience: 'api.ferrite.dev',
  },
  null,
  2,
)
</script>

<template>
  <UDashboardGroup
    ><SidebarPanel />
    <div class="flex flex-col w-full h-full bg-inkslate-950">
      <UDashboardNavbar class="w-full">
        <template #leading>
          <div class="flex flex-col">
            <UBreadcrumb :items="breadcrumbItems">
              <template #separator>
                <span class="text-muted">/</span>
              </template>
            </UBreadcrumb>
            <div class="text-highlighted font-bold text-xl">{{ $route.params.id }}</div>
          </div>
        </template>
        <template #right>
          <div class="flex gap-x-2">
            <UButton label="Run function" icon="i-lucide-play" />
          </div>
        </template>
      </UDashboardNavbar>
      <UDashboardPanel class="divide-x divide-default flex flex-row divide-y">
        <div class="w-1/3 max-w-1/3 flex flex-col px-6">
          <div class="text-highlighted font-bold py-4">Request payload</div>
          <UCard class="overflow-scroll grow" variant="outline">
            <highlightjs language="json" :code="payload" />
          </UCard>
          <div class="size-38 w-full py-2">
            <div class="flex flex-row justify-between">
              <div class="text-muted text-sm">Duration</div>
              <div class="text-highlighted font-bold">14ms</div>
            </div>
            <div class="flex flex-row justify-between">
              <div class="text-muted text-sm">Memory used</div>
              <div class="text-highlighted font-bold">6.4 / 128Mb</div>
            </div>
            <div class="flex flex-row justify-between">
              <div class="text-muted text-sm">Cold start</div>
              <div class="text-highlighted font-bold">No</div>
            </div>
          </div>
        </div>
        <div class="w-2/3 max-w-2/3">
          <UTabs variant="link" :items="tabs" class="flex pt-1.5">
            <template #logs>
              <UCard class="m-4" variant="outline">
                <pre class="grow overflow-scroll" v-html="interpretedOutput" />
              </UCard>
            </template>
            <template #response>
              <div class="flex gap-x-2 mx-4 mt-2 justify-between">
                <div class="flex items-center gap-x-1">
                  <UBadge color="success" variant="soft">200 OK</UBadge>
                  <div class="text-muted text-sm">Content-Type: image/webp</div>
                  <div class="text-muted text-sm">·</div>
                  <div class="text-muted text-sm">214Kb</div>
                  <div class="text-muted text-sm">·</div>
                  <div class="text-muted text-sm">14ms</div>
                </div>

                <div class="flex self-end gap-x-2">
                  <UButton icon="i-lucide-copy" variant="outline" />
                  <UButton icon="i-lucide-download" label="Download" variant="outline" />
                </div>
              </div>
              <UCard class="m-4">
                <highlightjs language="json" :code="payload" />
              </UCard>
            </template>
            <template #errors></template>
          </UTabs>
        </div>
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
