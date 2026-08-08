<script setup lang="ts">
import { computed, onMounted, onUnmounted, reactive, ref } from 'vue'
import { useRoute } from 'vue-router'
import type { BreadcrumbItem, TabsItem } from '@nuxt/ui'

import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'
import PayloadViewer from '@/components/dashboard/runs/ResponseViewer.vue'
import LogsViewer from '@/components/dashboard/runs/LogsViewer.vue'
import RequestPayload from '@/components/dashboard/runs/RequestPayload.vue'
import { $api } from '@/composables/useAPI'

const route = useRoute()

const breadcrumbItems: BreadcrumbItem[] = [
  { label: 'Functions', to: '/runs' },
  { label: route.params.name as string },
  { label: 'Runs', to: '/runs' },
  { label: route.params.uuid as string },
]
const tabs: TabsItem[] = [
  { label: 'Stdout', slot: 'stdout' as const },
  { label: 'Stderr', slot: 'stderr' as const },
  { label: 'Response', slot: 'response' as const },
]

const abortController = new AbortController()

const loadingDetails = ref(true)
const details = reactive({ duration: 0 })
const stderr = ref<string>('')
const stdout = ref<string>('')
const payload = ref<string>('')
const response = ref<string>('')

const formattedPayload = computed(() => JSON.stringify(payload.value, null, 2))
const formattedResponse = computed(() => JSON.stringify(response.value, null, 2))

const fetchRunDetails = async () => {
  loadingDetails.value = true

  try {
    const [detailsResp, stdoutResp, stderrResp, payloadResp, responseResp] = await Promise.all([
      $api(`/runs/${route.params.uuid}/details`, { credentials: 'include' }),
      $api<string>(`/runs/${route.params.uuid}/files/stdout`, { credentials: 'include' }),
      $api<string>(`/runs/${route.params.uuid}/files/stderr`, { credentials: 'include' }),
      $api<string>(`/runs/${route.params.uuid}/files/payload`, { credentials: 'include' }),
      $api<string>(`/runs/${route.params.uuid}/files/response`, { credentials: 'include' }),
    ])

    details.duration = detailsResp.duration
    stdout.value = stdoutResp
    stderr.value = stderrResp
    payload.value = payloadResp
    response.value = responseResp
  } catch (e) {
    console.error(e)
  }

  loadingDetails.value = false
}

onMounted(() => {
  fetchRunDetails()
})
onUnmounted(() => {
  abortController.abort()
})
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
        <RequestPayload :details="details" :payload="formattedPayload" />

        <div class="flex-1 h-full flex flex-col min-w-0">
          <UTabs
            variant="link"
            :items="tabs"
            class="flex-1 flex flex-col min-w-0 min-h-0"
            :ui="{ content: 'flex-1 flex flex-col min-h-0 min-w-0' }"
          >
            <template #stdout>
              <LogsViewer :rawIO="stdout" />
            </template>

            <template #response>
              <PayloadViewer :resp="formattedResponse" />
            </template>

            <template #stderr>
              <LogsViewer :rawIO="stderr" />
            </template>
          </UTabs>
        </div>
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
