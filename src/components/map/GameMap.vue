<script setup lang="ts">
import { onMounted, onUnmounted, ref, useTemplateRef, watch } from "vue"
import { createMap } from "./MapRenderer"
import { Button } from "@/components/ui/button"
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import type { LocationData } from "@/types/map"
import type { MapHandle } from "./MapRenderer"

type OverlayMode = 'habitability' | 'heightmap' | 'contours'

const canvasRef = useTemplateRef<HTMLDivElement>("canvasRef")
const mapHandle = ref<MapHandle | null>(null)
const overlayMode = ref<OverlayMode>('contours')
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

watch(overlayMode, (mode) => {
  mapHandle.value?.setOverlayMode(mode)
})

onMounted(async () => {
  if (!canvasRef.value) return
  const handle = await createMap(canvasRef.value, { onLocationSelect })
  mapHandle.value = handle
  // 初始同步默认模式
  handle.setOverlayMode(overlayMode.value)
})

onUnmounted(() => {
  mapHandle.value?.destroy()
})
</script>

<template>
  <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
    <div class="flex items-center gap-4">
      <RadioGroup
        v-model="overlayMode"
        class="flex flex-row gap-3"
      >
        <div class="flex items-center gap-1.5">
          <RadioGroupItem value="contours" :id="'overlay-contours'" />
          <label
            :for="'overlay-contours'"
            class="text-sm leading-none font-medium select-none"
          >
            等高线
          </label>
        </div>
        <div class="flex items-center gap-1.5">
          <RadioGroupItem value="heightmap" :id="'overlay-heightmap'" />
          <label
            :for="'overlay-heightmap'"
            class="text-sm leading-none font-medium select-none"
          >
            高度图
          </label>
        </div>
        <div class="flex items-center gap-1.5">
          <RadioGroupItem value="habitability" :id="'overlay-habitability'" />
          <label
            :for="'overlay-habitability'"
            class="text-sm leading-none font-medium select-none"
          >
            宜居度
          </label>
        </div>
      </RadioGroup>
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
