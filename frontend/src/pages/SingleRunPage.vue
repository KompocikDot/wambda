<script setup lang="ts">
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import type { BreadcrumbItem, TabsItem } from '@nuxt/ui'

import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'
import PayloadViewer from '@/components/dashboard/runs/ResponseViewer.vue'
import LogsViewer from '@/components/dashboard/runs/LogsViewer.vue'
import RequestPayload from '@/components/dashboard/runs/RequestPayload.vue'
import payload from '@/assets/blob.json'
import rawLogPayload from '@/assets/ansiLogsMock.log?raw'

const route = useRoute()

const rawOutput = ref(rawLogPayload)

const breadcrumbItems: BreadcrumbItem[] = [
  { label: 'Runs', to: '/runs' },
  { label: route.params.id as string },
]

const tabs: TabsItem[] = [
  { label: 'Logs', slot: 'logs' as const },
  { label: 'Response', slot: 'response' as const },
  { label: 'Errors', slot: 'errors' as const },
]

const formattedPayload = JSON.stringify(payload, null, 2)
</script>

<template>
  <UDashboardGroup>
    <SidebarPanel />

    <div class="flex flex-col w-full h-full bg-inkslate-950 min-w-0">
      <UDashboardNavbar class="w-full shrink-0">
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

      <UDashboardPanel class="flex-1 min-h-0 min-w-0 w-full divide-x divide-default flex flex-row">
        <RequestPayload :payload="formattedPayload" />

        <div class="flex-1 h-full flex flex-col min-w-0">
          <UTabs
            variant="link"
            :items="tabs"
            class="flex-1 flex flex-col min-w-0 min-h-0"
            :ui="{ content: 'flex-1 flex flex-col min-h-0 min-w-0' }"
          >
            <template #logs>
              <LogsViewer :rawIO="rawOutput" />
            </template>

            <template #response>
              <PayloadViewer :logs="formattedPayload" />
            </template>

            <template #errors> </template>
          </UTabs>
        </div>
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
