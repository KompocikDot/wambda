<script setup lang="ts">
import type { BreadcrumbItem } from '@nuxt/ui'
import { onUnmounted, ref, useTemplateRef } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import FunctionForm from '@/components/dashboard/functions/FunctionForm.vue'
import { $api } from '@/composables/useAPI'

const route = useRoute()
const router = useRouter()
const formRef = useTemplateRef<InstanceType<typeof FunctionForm>>('form')
const isFormLoading = ref(false)

const abortController = new AbortController()

const breadcrumbItems: BreadcrumbItem[] = [
  {
    label: 'Functions',
    to: '/functions',
  },
  {
    label: route.params.name as string,
  },
]

const deleteFunction = async (uuid: string) => {
  try {
    await $api(`/functions/${uuid}`, {
      method: 'DELETE',
      signal: abortController.signal,
      credentials: 'include',
    })
  } catch (e) {
    console.log(e)
  }
  router.replace({ name: 'functions' })
}

onUnmounted(() => {
  abortController.abort()
})
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
            <UButton label="Save changes" @click="formRef?.submit()" :disabled="isFormLoading" />

            <UButton
              v-if="$route.params.name !== 'new'"
              label="Delete function"
              :disabled="isFormLoading"
              color="error"
              @click="deleteFunction($route.params.uuid as string)"
            />
          </div>
        </template>
      </UDashboardNavbar>
      <UDashboardPanel class="px-6 py-4 gap-y-4">
        <FunctionForm ref="form" v-model:loading="isFormLoading" />
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
