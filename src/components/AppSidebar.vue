<script setup lang="ts">
import type { SidebarProps } from '@/components/ui/sidebar'

import {
  IconChartPie,
  IconFrame,
  IconMap,
} from '@tabler/icons-vue'
import NavProjects from '@/components/NavProjects.vue'

import {
  Sidebar,
  SidebarContent,
  SidebarRail,
} from '@/components/ui/sidebar'

const props = withDefaults(defineProps<SidebarProps>(), {
  collapsible: "icon",
})

// This is sample data.
const data = {
  projects: [
    {
      name: "Design Engineering",
      url: "#",
      icon: IconFrame,
    },
    {
      name: "Sales & Marketing",
      url: "#",
      icon: IconChartPie,
    },
    {
      name: "Travel",
      url: "#",
      icon: IconMap,
    },
  ],
}
</script>

<template>
  <Sidebar v-bind="props">
    <SidebarContent>
      <NavProjects :projects="data.projects" />
    </SidebarContent>
    <SidebarRail />
  </Sidebar>
</template>
