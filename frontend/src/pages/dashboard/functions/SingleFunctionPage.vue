<script setup lang="ts">
import { $api } from '@/composables/useAPI'
import {
  CreateFunctionSchema,
  Language,
  Trigger,
  type CreateFunctionInput,
  type CreateFunctionOutput,
  type EnvVarInput,
  type EnvVarOutput,
} from '@/schemas/function'
import type { BreadcrumbItem, FormSubmitEvent } from '@nuxt/ui'
import { reactive, useTemplateRef } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const form = useTemplateRef<HTMLFormElement>('form')

const formState = reactive<CreateFunctionInput>({
  name: '',
  language: Language.RUST,
  trigger: Trigger.HTTP,
  memoryLimit: 32,
  timeout: 5000,
  envs: [],
  wasm: undefined as unknown as File,
})

const breadcrumbItems: BreadcrumbItem[] = [
  {
    label: 'Functions',
    to: '/functions',
  },
  {
    label: route.params.name as string,
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

const newEnvVariable = reactive<EnvVarInput>({
  key: '',
  value: '',
})

const addVariable = () => {
  if (!newEnvVariable.key.trim()) {
    return
  }
  formState.envs.push({
    key: newEnvVariable.key,
    value: newEnvVariable.value,
  })
  newEnvVariable.key = ''
  newEnvVariable.value = ''
}

const removeVariable = (index: number) => {
  formState.envs.splice(index, 1)
}

const clearFirstField = () => {
  newEnvVariable.key = ''
  newEnvVariable.value = ''
}

const stringifyEnvVariables = (envs: EnvVarOutput[]): string => {
  let stringifiedEnvs = ''
  for (const env of envs) {
    stringifiedEnvs += atob(env.key)
    stringifiedEnvs += ':'
    stringifiedEnvs += atob(env.value)
    stringifiedEnvs += ','
  }

  return stringifiedEnvs
}

const onSubmit = async (event: FormSubmitEvent<CreateFunctionOutput>) => {
  //Route without func uuid is a page from clicking on 'create function' btn
  const newForm = route.params.uuid === 'new'
  const method = newForm ? 'POST' : 'PATCH'
  const endpoint = newForm ? '/functions' : `/functions/${route.params.uuid}`

  const payload = new FormData()
  payload.append('name', event.data.name)
  payload.append('wasm', event.data.wasm!)
  payload.append('language', event.data.language)
  payload.append('memoryLimit', event.data.memoryLimit.toString())
  payload.append('timeout', event.data.timeout.toString())
  payload.append('trigger', event.data.trigger)
  payload.append('envsStringified', stringifyEnvVariables(event.data.envs))

  try {
    await $api(endpoint, {
      credentials: 'include',
      method,
      body: payload,
    })
  } catch (e) {
    console.error(e)
  }
}
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
            <UButton label="Save changes" @click="form?.submit()" />
          </div>
        </template>
      </UDashboardNavbar>
      <UDashboardPanel class="px-6 py-4 gap-y-4">
        <UForm
          ref="form"
          class="flex gap-x-4"
          :state="formState"
          @submit="onSubmit"
          :schema="CreateFunctionSchema"
        >
          <div class="flex flex-col w-2/3 gap-y-2">
            <UFormField label="Function name" class="w-full" required>
              <UInput class="w-full" v-model="formState.name" />
            </UFormField>
            <div class="flex gap-x-2">
              <UFormField label="Language" class="w-1/2" required>
                <USelect class="w-full" :items="languages" v-model="formState.language" />
              </UFormField>
              <UFormField label="Trigger" class="w-1/2" required>
                <USelect class="w-full" :items="triggers" v-model="formState.trigger" />
              </UFormField>
            </div>
            <div class="flex gap-x-2">
              <UFormField label="Memory limit (Mb)" class="w-1/2" required>
                <UInput
                  class="w-full"
                  type="number"
                  step="1"
                  min="1"
                  max="2048"
                  v-model="formState.memoryLimit"
                />
              </UFormField>
              <UFormField label="Timeout (ms)" class="w-1/2" required>
                <UInput class="w-full" type="number" step="1" min="0" v-model="formState.timeout" />
              </UFormField>
            </div>
            <div class="flex flex-col gap-2">
              <UFormField label="Environment variables">
                <div class="flex flex-col gap-y-2">
                  <div
                    class="flex items-center gap-x-2"
                    v-for="(variable, index) in formState.envs"
                    :key="index"
                  >
                    <UInput class="w-full" v-model="variable.key" />
                    <UInput class="w-full" v-model="variable.value" />
                    <UButton icon="i-lucide-x" variant="outline" @click="removeVariable(index)" />
                  </div>
                  <div class="flex items-center gap-x-2">
                    <UInput class="w-full" v-model="newEnvVariable.key" placeholder="Key" />
                    <UInput class="w-full" v-model="newEnvVariable.value" placeholder="Value" />
                    <UButton
                      icon="i-lucide-x"
                      variant="outline"
                      :disabled="
                        formState.envs.length !== 0 ||
                        (newEnvVariable.key === '' && newEnvVariable.value === '')
                      "
                      @click="clearFirstField"
                    />
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
            <UFormField label="Module (.wasm)" required>
              <UFileUpload
                accept="application/wasm"
                highlight
                label="Drag & drop your .wasm file"
                description="or click to browse"
                layout="list"
                v-model="formState.wasm"
              />
            </UFormField>
          </div>
        </UForm>
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
