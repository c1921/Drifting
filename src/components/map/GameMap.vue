<script setup lang="ts">
import { onMounted, onUnmounted, ref, useTemplateRef } from "vue"
import { createMap } from "./MapRenderer"
import { Switch } from "@/components/ui/switch"
import { Button } from "@/components/ui/button"
import type { LocationData } from "@/types/map"
import type { MapHandle } from "./MapRenderer"

const canvasRef = useTemplateRef<HTMLDivElement>("canvasRef")
const mapHandle = ref<MapHandle | null>(null)
const showHabitability = ref(true)
const regenerating = ref(false)

function onLocationSelect(loc: LocationData) {
  // Placeholder for future interaction (e.g. tooltip, detail panel)
  console.log("Selected:", loc.name)
}

async function regenerate() {
  if (!mapHandle.value || regenerating.value) return
  regenerating.value = true
  try {
    await mapHandle.value.regenerate()
  } finally {
    regenerating.value = false
  }
}

onMounted(async () => {
  if (!canvasRef.value) return
  const handle = await createMap(canvasRef.value, { onLocationSelect })
  mapHandle.value = handle
})

onUnmounted(() => {
  mapHandle.value?.destroy()
})

function toggleHabitability(value: boolean) {
  showHabitability.value = value
  mapHandle.value?.setHabitabilityVisible(value)
}
</script>

<template>
  <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
    <div class="flex items-center gap-2">
      <Switch
        :id="'habitability-toggle'"
        :model-value="showHabitability"
        @update:model-value="toggleHabitability"
      />
      <label
        :for="'habitability-toggle'"
        class="text-sm leading-none font-medium select-none"
      >
        宜居度
      </label>
      <Button
        variant="outline"
        size="sm"
        :disabled="regenerating"
        @click="regenerate"
      >
        {{ regenerating ? '生成中…' : '重新生成' }}
      </Button>
    </div>
    <div class="flex flex-1 items-stretch">
      <div
        ref="canvasRef"
        class="w-full rounded-xl border bg-card shadow-xs"
        style="min-height: 500px"
      />
    </div>
  </div>
</template>
