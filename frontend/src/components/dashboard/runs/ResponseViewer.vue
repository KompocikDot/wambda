<script setup lang="ts">
defineProps<{ logs: string }>()
</script>

<template>
  <div class="flex flex-col w-full h-full min-h-0 min-w-0">
    <div class="flex shrink-0 gap-x-2 mx-4 mt-2 justify-between">
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

    <div class="flex-1 flex flex-col min-h-0 min-w-0 p-4 pt-2">
      <UCard
        class="flex-1 flex flex-col min-h-0 min-w-0"
        variant="outline"
        :ui="{ body: 'flex-1 overflow-auto' }"
      >
        <highlightjs language="json" :code="logs" />
      </UCard>
    </div>
  </div>
</template>
