<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{ payload: string; details: { duration: number } }>()

const duration = computed(() => {
  const ms = props.details.duration
  const hours = ((ms / 3600000) % 24).toFixed()
  const minutes = ((ms / 60000) % 60).toFixed()
  const seconds = ((ms / 1000) % 60).toFixed()
  const milliseconds = ((ms % 1000) / 1000).toFixed()

  if (ms > 3600000) {
    return `${hours}h ${minutes}m ${seconds}s`
  } else if (ms > 60000) {
    return `${minutes}m ${seconds}s`
  } else if (ms > 1000) {
    return `${seconds}s ${milliseconds}ms`
  }

  return `${ms}ms`
})
</script>

<template>
  <div class="w-1/3 max-w-1/3 flex flex-col h-full px-6">
    <div class="text-highlighted font-bold py-4 shrink-0">Request payload</div>
    <UCard
      class="flex-1 flex min-h-0 min-w-0 overflow-scroll"
      :ui="{ body: 'flex-1 min-h-0 min-w-0 overflow-auto' }"
    >
      <highlightjs language="json" :code="payload" />
    </UCard>
    <div class="w-full py-2 shrink-0">
      <div class="flex flex-row justify-between">
        <div class="text-muted text-sm">Duration</div>
        <div class="text-highlighted font-bold">{{ duration }}</div>
      </div>
      <!-- <div class="flex flex-row justify-between">
        <div class="text-muted text-sm">Memory used</div>
        <div class="text-highlighted font-bold">6.4 / 128Mb</div>
      </div>
      <div class="flex flex-row justify-between">
        <div class="text-muted text-sm">Cold start</div>
        <div class="text-highlighted font-bold">No</div>
      </div> -->
    </div>
  </div>
</template>
