<script setup lang="ts">
import type { BreadcrumbItem, FormSubmitEvent } from '@nuxt/ui'
import { reactive } from 'vue'
import { useRoute } from 'vue-router'

type EnvVariable = {
  key: string
  value: string
}

type FunctionForm = {
  name: string
  language: string
  trigger: string
  memoryLimitMb: number
  timeoutMs: number
  envVariables: EnvVariable[]
  binary?: File
}

const route = useRoute()

const formState = reactive<FunctionForm>({
  name: '',
  language: '',
  trigger: '',
  memoryLimitMb: 32,
  timeoutMs: 5000,
  envVariables: [],
  binary: undefined,
})

const breadcrumbItems: BreadcrumbItem[] = [
  {
    label: 'Functions',
    to: '/functions',
  },
  {
    label: route.params.id as string,
  },
]

const languages = [
  { label: 'C', value: 'c' },
  { label: 'Go', value: 'go' },
  { label: 'Javascript', value: 'javascript' },
  { label: 'Python', value: 'python' },
  { label: 'Rust', value: 'rust' },
]

const triggers = [
  { label: 'HTTP endpoint', value: 'http' },
  { label: 'Cron', value: 'cron' },
]

const newEnvVariable = reactive<EnvVariable>({
  key: '',
  value: '',
})

const addVariable = () => {
  if (!newEnvVariable.key.trim()) {
    return
  }
  formState.envVariables.push({
    key: newEnvVariable.key,
    value: newEnvVariable.value,
  })
  newEnvVariable.key = ''
  newEnvVariable.value = ''
}

const removeVariable = (index: number) => {
  formState.envVariables.splice(index, 1)
}

const onSubmit = async (event: FormSubmitEvent<FunctionForm>) => {}
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
            <div class="text-highlighted font-bold text-xl">{{ $route.params.id }}</div>
          </div>
        </template>
        <template #right>
          <div class="flex gap-x-2">
            <UButton label="Cancel" variant="outline" />
            <UButton label="Save changes" />
          </div>
        </template>
      </UDashboardNavbar>
      <UDashboardPanel class="px-6 py-4 gap-y-4">
        <UForm class="flex gap-x-4" :state="formState" @submit="onSubmit">
          <div class="flex flex-col w-2/3 gap-y-2">
            <UFormField label="Function name" class="w-full">
              <UInput class="w-full" v-model="formState.name" />
            </UFormField>
            <div class="flex gap-x-2">
              <UFormField label="Language" class="w-1/2">
                <USelect class="w-full" :items="languages" v-model="formState.language" />
              </UFormField>
              <UFormField label="Trigger" class="w-1/2">
                <USelect class="w-full" :items="triggers" v-model="formState.trigger" />
              </UFormField>
            </div>
            <div class="flex gap-x-2">
              <UFormField label="Memory limit (Mb)" class="w-1/2">
                <UInput
                  class="w-full"
                  type="number"
                  step="1"
                  min="1"
                  max="2048"
                  v-model="formState.memoryLimitMb"
                />
              </UFormField>
              <UFormField label="Timeout (ms)" class="w-1/2">
                <UInput
                  class="w-full"
                  type="number"
                  step="1"
                  min="0"
                  v-model="formState.timeoutMs"
                />
              </UFormField>
            </div>
            <div class="flex flex-col gap-2">
              <UFormField label="Environment variables">
                <div class="flex flex-col gap-y-2">
                  <div
                    class="flex items-center gap-x-2"
                    v-for="(variable, index) in formState.envVariables"
                    :key="variable.key"
                  >
                    <UInput class="w-full" v-model="variable.key" />
                    <UInput class="w-full" v-model="variable.value" />
                    <UButton icon="i-lucide-x" variant="outline" @click="removeVariable(index)" />
                  </div>
                  <div class="flex items-center gap-x-2">
                    <UInput class="w-full" v-model="newEnvVariable.key" placeholder="Key" />
                    <UInput class="w-full" v-model="newEnvVariable.value" placeholder="Value" />
                    <UButton icon="i-lucide-x" variant="outline" disabled />
                  </div>
                </div>
              </UFormField>
              <UButton
                label="Add variable"
                class="w-fit"
                @click="addVariable"
                :disabled="newEnvVariable.key.trim() === ''"
                icon="i-lucide-plus"
                variant="outline"
              />
            </div>
          </div>
          <div class="w-1/3">
            <UFormField label="Module (.wasm)">
              <UFileUpload
                accept="application/wasm"
                highlight
                label="Drag & drop your .wasm file"
                description="or click to browse"
                layout="list"
                v-model="formState.binary"
              />
            </UFormField>
          </div>
        </UForm>
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
