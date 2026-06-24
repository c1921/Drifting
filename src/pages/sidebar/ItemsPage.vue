<script setup lang="ts">
import { ref } from 'vue'
import {
  IconList,
  IconPlus,
  IconCategory,
  IconPackage,
} from '@tabler/icons-vue'

const activeTab = ref('browse')

const tabs = [
  { key: 'browse', label: 'Browse Items', icon: IconList },
  { key: 'create', label: 'Add Item', icon: IconPlus },
  { key: 'categories', label: 'Categories', icon: IconCategory },
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

    <!-- Browse Items -->
    <div v-if="activeTab === 'browse'" class="flex flex-1 flex-col gap-4">
      <div class="grid auto-rows-min gap-4 md:grid-cols-4">
        <div
          v-for="n in 8"
          :key="n"
          class="bg-muted/50 flex items-center justify-center rounded-xl p-4 aspect-square"
        >
          <div class="text-center">
            <IconPackage class="mx-auto h-8 w-8 text-muted-foreground/50" />
            <p class="mt-2 text-sm text-muted-foreground">Item #{{ n }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Add Item -->
    <div v-else-if="activeTab === 'create'" class="flex flex-1 flex-col gap-4">
      <div class="bg-muted/50 flex flex-1 items-center justify-center rounded-xl p-6">
        <div class="text-center">
          <IconPlus class="mx-auto h-12 w-12 text-muted-foreground/50" />
          <p class="mt-4 text-lg font-medium text-muted-foreground">Add New Item</p>
          <p class="mt-1 text-sm text-muted-foreground/70">Item creation form will be displayed here</p>
        </div>
      </div>
    </div>

    <!-- Categories -->
    <div v-else-if="activeTab === 'categories'" class="flex flex-1 flex-col gap-4">
      <div class="grid auto-rows-min gap-4 md:grid-cols-3">
        <div
          v-for="cat in ['Weapons', 'Armor', 'Potions', 'Materials', 'Scrolls', 'Treasure']"
          :key="cat"
          class="bg-muted/50 flex items-center justify-center rounded-xl p-6 aspect-video"
        >
          <div class="text-center">
            <IconCategory class="mx-auto h-8 w-8 text-muted-foreground/50" />
            <p class="mt-2 text-sm text-muted-foreground">{{ cat }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
