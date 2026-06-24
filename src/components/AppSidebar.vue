<script setup lang="ts">
import type { SidebarProps } from '@/components/ui/sidebar'

import {
  IconCalendarEvent,
  IconMap,
  IconPackage,
  IconShoppingCart,
  IconUserShield,
} from '@tabler/icons-vue'
import NavMain from '@/components/NavMain.vue'
import ModeToggle from '@/components/ModeToggle.vue'

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarRail,
} from '@/components/ui/sidebar'

const props = withDefaults(defineProps<SidebarProps>(), {
  collapsible: "icon",
})

// This is sample data.
const data = {
  navMain: [
    {
      title: "Role",
      url: "/role",
      icon: IconUserShield,
    },
    {
      title: "Items",
      url: "/items",
      icon: IconPackage,
    },
    {
      title: "Events",
      url: "/events",
      icon: IconCalendarEvent,
    },
    {
      title: "Trading",
      url: "/trading",
      icon: IconShoppingCart,
    },
    {
      title: "Map",
      url: "/map",
      icon: IconMap,
    },
  ],
}
</script>

<template>
  <Sidebar v-bind="props">
    <SidebarContent>
      <NavMain :items="data.navMain" />
    </SidebarContent>
    <SidebarFooter class="border-t p-2">
      <div class="flex justify-center">
        <ModeToggle />
      </div>
    </SidebarFooter>
    <SidebarRail />
  </Sidebar>
</template>
