<script setup lang="ts">
import type { BreadcrumbItem } from '@nuxt/ui'
import { useTemplateRef } from 'vue'
import { useRoute } from 'vue-router'
import FunctionForm from '@/components/dashboard/functions/FunctionForm.vue'

const route = useRoute()
const formRef = useTemplateRef<InstanceType<typeof FunctionForm>>('form')

const breadcrumbItems: BreadcrumbItem[] = [
  {
    label: 'Functions',
    to: '/functions',
  },
  {
    label: route.params.name as string,
  },
]
</script>

<template>
  <UDashboardGroup>
    <SidebarPanel />
    <div class="flex flex-col w-full h-full">
      <UDashboardNavbar class="w-full">
        <template #leading>
          <div class="flex flex-col">
            <UBreadcrumb :items="breadcrumbItems">
              <template #separator>
                <span class="text-muted">/</span>
              </template>
            </UBreadcrumb>
            <div class="text-highlighted font-bold text-xl">{{ $route.params.name }}</div>
          </div>
        </template>
        <template #right>
          <div class="flex gap-x-2">
            <UButton label="Cancel" variant="outline" :to="{ name: 'functions' }" />
            <UButton label="Save changes" @click="formRef?.submit()" />
          </div>
        </template>
      </UDashboardNavbar>
      <UDashboardPanel class="px-6 py-4 gap-y-4">
        <FunctionForm ref="form" />
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
