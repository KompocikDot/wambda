<script setup lang="ts">
import SidebarPanel from '@/components/dashboard/SidebarPanel.vue'
import DeleteAccountForm from '@/components/dashboard/settings/DeleteAccountForm.vue'
import ChangePasswordForm from '@/components/dashboard/settings/ChangePasswordForm.vue'
import UpdateAccountDetailsForm from '@/components/dashboard/settings/UpdateAccountDetailsForm.vue'
</script>

<template>
  <UDashboardGroup>
    <SidebarPanel />
    <div class="flex flex-col w-full h-full">
      <UDashboardNavbar title="Settings" icon="i-lucide-settings" />
      <UDashboardPanel class="px-6 py-4">
        <div class="flex flex-col gap-y-4">
          <UpdateAccountDetailsForm />
          <ChangePasswordForm />
          <DeleteAccountForm />
        </div>
      </UDashboardPanel>
    </div>
  </UDashboardGroup>
</template>
