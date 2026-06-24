<script setup lang="ts">
import { ref, onMounted } from 'vue'
import {
  IconList,
  IconInfoCircle,
  IconUserShield,
  IconLoader2,
} from '@tabler/icons-vue'
import CharacterDetail from '@/components/CharacterDetail.vue'
import type { RoleData } from '@/types/role'
import { listRoles } from '@/api/role'

const activeTab = ref('list')
const roles = ref<RoleData[]>([])
const selectedRole = ref<RoleData | null>(null)
const loading = ref(false)
const error = ref('')

const tabs = [
  { key: 'list', label: 'Role List', icon: IconList },
  { key: 'details', label: 'Role Details', icon: IconInfoCircle },
]

// ── 加载角色列表 ──────────────────────────────
async function loadRoles() {
  loading.value = true
  error.value = ''
  try {
    roles.value = await listRoles()
  } catch (e) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}

// ── 选择角色查看详情 ──────────────────────────
function selectRole(role: RoleData) {
  selectedRole.value = role
  activeTab.value = 'details'
}

onMounted(loadRoles)
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

    <!-- 错误提示 -->
    <div
      v-if="error"
      class="rounded-lg border border-destructive/50 bg-destructive/10 px-4 py-3 text-sm text-destructive"
    >
      {{ error }}
    </div>

    <!-- Role List -->
    <div v-if="activeTab === 'list'" class="flex flex-1 flex-col gap-4">
      <!-- 角色计数 -->
      <div class="text-sm text-muted-foreground">{{ roles.length }} roles found</div>

      <!-- 加载中 -->
      <div v-if="loading" class="flex flex-1 items-center justify-center">
        <IconLoader2 class="size-8 animate-spin text-muted-foreground/50" />
      </div>

      <!-- 角色网格 -->
      <div v-else-if="roles.length > 0" class="grid auto-rows-min gap-4 md:grid-cols-3">
        <div
          v-for="role in roles"
          :key="role.id"
          class="bg-muted/50 flex cursor-pointer flex-col items-center justify-center rounded-xl p-6 aspect-video transition-colors hover:bg-muted"
          @click="selectRole(role)"
        >
          <IconUserShield class="mx-auto h-8 w-8 text-muted-foreground/50" />
          <p class="mt-2 text-sm font-medium text-foreground">{{ role.name }}</p>
          <p class="text-xs text-muted-foreground">{{ role.gender }}, {{ role.age }} yrs</p>
        </div>
      </div>

      <!-- 空状态 -->
      <div v-else class="flex flex-1 items-center justify-center">
        <div class="text-center">
          <IconUserShield class="mx-auto h-12 w-12 text-muted-foreground/30" />
          <p class="mt-4 text-lg font-medium text-muted-foreground">No roles yet</p>
        </div>
      </div>
    </div>

    <!-- Role Details -->
    <div v-else-if="activeTab === 'details'" class="flex flex-1 flex-col">
      <CharacterDetail :role="selectedRole" />
    </div>
  </div>
</template>
