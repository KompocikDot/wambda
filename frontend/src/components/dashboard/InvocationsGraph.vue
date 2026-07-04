<script setup lang="ts">
import { VisXYContainer, VisArea, VisLine, VisAxis } from '@unovis/vue'
import { Scale } from '@unovis/ts'

export type InvocationGraphPoint = {
  date: Date
  totalInvocations: number
}

defineProps<{ usageData: InvocationGraphPoint[] }>()

const x = (d: InvocationGraphPoint) => +d.date
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
  <div class="pt-2">
    <VisXYContainer :data="usageData" :x-scale="xtimeScale">
      <VisArea :x="x" :y="y" :opacity="0.3" />
      <VisLine :x="x" :y="y" :line-width="5" />

      <VisAxis type="x" :grid-line="false" :tick-format="xTickFormat" />
      <VisAxis type="y" :grid-line="false" :tick-format="yTickFormat" />
    </VisXYContainer>
  </div>
</template>
