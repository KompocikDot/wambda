<script setup lang="ts">
import { useColorMode } from '@vueuse/core'
const colorMode = useColorMode()
</script>

<template>
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" width="24" height="24">
    <g :fill="colorMode === 'dark' ? '#ffffff' : '#111111'">
      <rect x="10" y="10" width="40" height="14" rx="4"></rect>
      <rect x="10" y="10" width="14" height="40" rx="4"></rect>
      <rect x="70" y="10" width="40" height="14" rx="4"></rect>
      <rect x="96" y="10" width="14" height="40" rx="4"></rect>
      <rect x="10" y="96" width="40" height="14" rx="4"></rect>
      <rect x="10" y="70" width="14" height="40" rx="4"></rect>
      <rect x="70" y="96" width="40" height="14" rx="4"></rect>
      <rect x="96" y="70" width="14" height="40" rx="4"></rect>
    </g>
  </svg>
</template>
