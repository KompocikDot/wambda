<script setup lang="ts">
import { VisXYContainer, VisArea, VisLine, VisAxis } from '@unovis/vue'
import { Scale } from '@unovis/ts'

export type InvocationGraphPoint = {
  timestamp: number
  totalInvocations: number
}

defineProps<{ usageData: InvocationGraphPoint[]; loading: boolean }>()

const x = (d: InvocationGraphPoint) => d.timestamp

const y = (d: InvocationGraphPoint) => d.totalInvocations

const xtimeScale = Scale.scaleTime()

const xTickFormat = (tick: number) => {
  return new Date(tick).toLocaleDateString(undefined, {
    month: 'short',
    day: 'numeric',
  })
}
const yTickFormat = (tick: number) => tick.toLocaleString()
</script>

<template>
  <div class="pt-2 h-full">
    <USkeleton v-if="loading" class="w-full h-80" />
    <VisXYContainer v-else class="h-80" :data="usageData" :x-scale="xtimeScale">
      <VisArea :x="x" :y="y" :opacity="0.3" />
      <VisLine :x="x" :y="y" :line-width="5" />

      <VisAxis type="x" :grid-line="false" :tick-format="xTickFormat" />
      <VisAxis type="y" :grid-line="false" :tick-format="yTickFormat" />
    </VisXYContainer>
  </div>
</template>
