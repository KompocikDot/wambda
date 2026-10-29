<script setup lang="ts">
export type InvocationStat = {
  name: string
  count: number
}

defineProps<{
  invocations: InvocationStat[]
  loading: boolean
}>()
</script>

<template>
  <UCard class="flex flex-col" :ui="{ body: 'bg-elevated/50 h-full' }">
    <div class="text-highlighted font-bold pb-2">Top functions by invocations</div>
    <USkeleton v-if="loading" class="w-full h-80" />
    <div
      v-else
      v-for="stat in invocations"
      class="flex justify-between not-last:border-b border-default"
      :key="stat.name"
    >
      <div>{{ stat.name }}</div>
      <div class="text-highlighted font-bold">{{ stat.count }}</div>
    </div>
    <div v-if="!loading && invocations.length === 0">No invocations found</div>
  </UCard>
</template>
