<script setup lang="ts">
import { ref } from 'vue'
import {
  IconList,
  IconUserPlus,
  IconInfoCircle,
  IconUserShield,
} from '@tabler/icons-vue'

const activeTab = ref('list')

const tabs = [
  { key: 'list', label: 'Role List', icon: IconList },
  { key: 'create', label: 'Create Role', icon: IconUserPlus },
  { key: 'details', label: 'Role Details', icon: IconInfoCircle },
]
</script>

<template>
  <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
    <!-- Tab 切换栏 -->
    <div class="flex gap-1 rounded-lg bg-muted p-1">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        class="inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium whitespace-nowrap transition-all"
        :class="activeTab === tab.key
          ? 'bg-background text-foreground shadow-xs'
          : 'text-muted-foreground hover:text-foreground'"
        @click="activeTab = tab.key"
      >
        <component :is="tab.icon" class="h-4 w-4" />
        {{ tab.label }}
      </button>
    </div>

    <!-- Role List -->
    <div v-if="activeTab === 'list'" class="flex flex-1 flex-col gap-4">
      <div class="grid auto-rows-min gap-4 md:grid-cols-3">
        <div
          v-for="n in 6"
          :key="n"
          class="bg-muted/50 flex items-center justify-center rounded-xl p-6 aspect-video"
        >
          <div class="text-center">
            <IconUserShield class="mx-auto h-8 w-8 text-muted-foreground/50" />
            <p class="mt-2 text-sm text-muted-foreground">Role {{ n }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Role -->
    <div v-else-if="activeTab === 'create'" class="flex flex-1 flex-col gap-4">
      <div class="bg-muted/50 flex flex-1 items-center justify-center rounded-xl p-6">
        <div class="text-center">
          <IconUserPlus class="mx-auto h-12 w-12 text-muted-foreground/50" />
          <p class="mt-4 text-lg font-medium text-muted-foreground">Create New Role</p>
          <p class="mt-1 text-sm text-muted-foreground/70">Role creation form will be displayed here</p>
        </div>
      </div>
    </div>

    <!-- Role Details -->
    <div v-else-if="activeTab === 'details'" class="flex flex-1 flex-col gap-4">
      <div class="bg-muted/50 flex flex-1 items-center justify-center rounded-xl p-6">
        <div class="text-center">
          <IconInfoCircle class="mx-auto h-12 w-12 text-muted-foreground/50" />
          <p class="mt-4 text-lg font-medium text-muted-foreground">Role Details</p>
          <p class="mt-1 text-sm text-muted-foreground/70">Selected role details will be displayed here</p>
        </div>
      </div>
    </div>
  </div>
</template>
