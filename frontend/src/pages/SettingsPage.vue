<script setup lang="ts">
import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'

const updateAccountDetails = async () => {}
const changePassword = async () => {}
const deleteAccount = async () => {}
</script>

<template>
  <UDashboardGroup>
    <SidebarPanel />
    <div class="flex flex-col w-full h-full">
      <UDashboardNavbar title="Settings" icon="i-lucide-settings" />
      <UDashboardPanel class="px-6 py-4">
        <div class="flex flex-col gap-y-4">
          <UCard class="w-2/3" variant="subtle">
            <UForm class="flex flex-col gap-y-2" @submit="updateAccountDetails">
              <div class="text-highlighted font-bold">Account details</div>
              <div class="flex flex-row gap-x-2">
                <UFormField label="Full name">
                  <UInput />
                </UFormField>
                <UFormField label="Email">
                  <UInput />
                </UFormField>
              </div>
              <UButton type="submit" label="Save changes" class="w-fit" />
            </UForm>
          </UCard>
          <UCard class="w-2/3" variant="subtle">
            <UForm class="flex flex-col gap-y-2" @submit="changePassword">
              <div class="text-highlighted font-bold">Change password</div>
              <div class="flex flex-row gap-x-2">
                <UFormField label="Current password">
                  <UInput type="password" />
                </UFormField>
                <UFormField label="New password">
                  <UInput type="password" />
                </UFormField>
              </div>
              <UButton type="submit" label="Update password" variant="outline" class="w-fit" />
            </UForm>
          </UCard>
          <UCard class="w-2/3" variant="subtle">
            <UForm class="flex flex-col gap-y-2" @submit="deleteAccount">
              <div class="text-highlighted font-bold">Delete account</div>
              <div class="text-muted text-sm">
                Permanently deletes your account, all deployed functions and run history. This
                cannot be undone.
              </div>
              <UButton type="submit" label="Delete account" color="error" class="w-fit" />
            </UForm>
          </UCard>
        </div>
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
