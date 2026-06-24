<script setup lang="ts">
import { onMounted, onUnmounted, useTemplateRef } from "vue"
import { createMap } from "./MapRenderer"
import type { LocationData } from "./data/mapData"

const canvasRef = useTemplateRef<HTMLDivElement>("canvasRef")
let destroyMap: (() => void) | null = null

function onLocationSelect(loc: LocationData) {
  // Placeholder for future interaction (e.g. tooltip, detail panel)
  console.log("Selected:", loc.name)
}

onMounted(async () => {
  if (!canvasRef.value) return
  const handle = await createMap(canvasRef.value, { onLocationSelect })
  destroyMap = handle.destroy
})

onUnmounted(() => {
  destroyMap?.()
})
</script>

<template>
  <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
    <div class="flex flex-1 items-stretch">
      <div
        ref="canvasRef"
        class="w-full rounded-xl border bg-card shadow-xs"
        style="min-height: 500px;"
      />
    </div>
  </div>
</template>
