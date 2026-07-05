<script setup lang="ts">
defineProps<{ payload: string }>()
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
</template>
