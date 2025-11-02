<script setup lang="ts">
import { computed, onMounted, onBeforeUnmount, ref } from 'vue'

type Terrain = 'plains' | 'desert' | 'mountain' | 'forest' | 'hills' | 'swamp' | 'water'

interface Coordinate { x: number; y: number }
interface CityNode {
  name: string
  coord: Coordinate
  terrain: Terrain
  neighbors: string[]
}
interface Graph { nodes: Record<string, CityNode> }
interface TimeStatus {
  day: number
  hour: number
  total_hours: number
}

const loading = ref(true)
const error = ref<string>('')
const graph = ref<Graph | null>(null)
const selected = ref<string | null>(null)
const showDistances = ref(false)
const moveMode = ref(false)
const player = ref<string | null>(null)
const timeState = ref<TimeStatus | null>(null)
const lastTravelHours = ref<number | null>(null)
const displayTime = ref<TimeStatus | null>(null)
const isSimulating = ref(false)
const simulatedPosition = ref<Coordinate | null>(null)

const HOURS_PER_DAY = 24
const REALTIME_PER_GAME_HOUR = 1000

let simulationFrame: number | null = null
let simulationStartTotal = 0
let simulationTargetTotal = 0
let simulationDurationMs = 0
let simulationStartMs = 0
let simulationFinalTime: TimeStatus | null = null
let simulationStartCoord: Coordinate | null = null
let simulationTargetCoord: Coordinate | null = null

async function load() {
  loading.value = true
  error.value = ''
  try {
    const res = await fetch('/api/map')
    if (!res.ok) throw new Error(`HTTP ${res.status}`)
    graph.value = (await res.json()) as Graph
  } catch (e: any) {
    error.value = e?.message ?? '加载失败'
  } finally {
    loading.value = false
  }
}

async function loadPlayer() {
  try {
    const res = await fetch('/api/player')
    if (!res.ok) throw new Error(`HTTP ${res.status}`)
    const data = await res.json()
    player.value = data.location as string
    if (data.time) {
      timeState.value = data.time as TimeStatus
      syncDisplayTime(timeState.value)
    }
  } catch (e: any) {
    // 不阻塞主流程
  }
}

async function loadTime() {
  try {
    const res = await fetch('/api/time')
    if (!res.ok) throw new Error(`HTTP ${res.status}`)
    const data = await res.json()
    timeState.value = data as TimeStatus
    syncDisplayTime(timeState.value)
  } catch (e: any) {
    // ignore standalone time errors
  }
}

function cloneTime(t: TimeStatus): TimeStatus {
  return { day: t.day, hour: t.hour, total_hours: t.total_hours }
}

function cloneCoord(c: Coordinate): Coordinate {
  return { x: c.x, y: c.y }
}

function deriveTimeStatus(totalHours: number): TimeStatus {
  const safeTotal = totalHours < 0 ? 0 : totalHours
  const dayIndex = Math.floor(safeTotal / HOURS_PER_DAY)
  const hour = safeTotal - dayIndex * HOURS_PER_DAY
  return { day: dayIndex + 1, hour, total_hours: safeTotal }
}

function syncDisplayTime(source: TimeStatus | null) {
  if (isSimulating.value) return
  displayTime.value = source ? cloneTime(source) : null
}

function cancelSimulation() {
  if (simulationFrame != null) {
    cancelAnimationFrame(simulationFrame)
    simulationFrame = null
  }
  isSimulating.value = false
  simulationFinalTime = null
  simulationStartCoord = null
  simulationTargetCoord = null
  simulatedPosition.value = null
}

function finishSimulation() {
  if (simulationFinalTime) {
    displayTime.value = cloneTime(simulationFinalTime)
  } else if (timeState.value) {
    displayTime.value = cloneTime(timeState.value)
  }
  cancelSimulation()
  simulationFinalTime = null
  error.value = ''
  void loadPlayer()
}

function startSimulation(start: TimeStatus, end: TimeStatus, durationHours: number, startCoord?: Coordinate | null, endCoord?: Coordinate | null) {
  cancelSimulation()
  if (durationHours <= 0) {
    displayTime.value = cloneTime(end)
    return
  }

  isSimulating.value = true
  simulationStartTotal = start.total_hours
  simulationTargetTotal = end.total_hours
  simulationDurationMs = Math.max(durationHours * REALTIME_PER_GAME_HOUR, 0)
  simulationStartMs = performance.now()
  simulationFinalTime = cloneTime(end)
  simulationStartCoord = startCoord ? cloneCoord(startCoord) : null
  simulationTargetCoord = endCoord ? cloneCoord(endCoord) : null
  simulatedPosition.value = simulationStartCoord ? cloneCoord(simulationStartCoord) : null
  displayTime.value = cloneTime(start)

  const step = (now: number) => {
    if (!isSimulating.value) return
    const elapsed = now - simulationStartMs
    const ratio = simulationDurationMs <= 0 ? 1 : Math.min(elapsed / simulationDurationMs, 1)
    const current = simulationStartTotal + (simulationTargetTotal - simulationStartTotal) * ratio
    displayTime.value = deriveTimeStatus(current)
    if (simulationStartCoord && simulationTargetCoord) {
      const dx = simulationTargetCoord.x - simulationStartCoord.x
      const dy = simulationTargetCoord.y - simulationStartCoord.y
      simulatedPosition.value = {
        x: simulationStartCoord.x + dx * ratio,
        y: simulationStartCoord.y + dy * ratio,
      }
    }
    if (ratio >= 1) {
      finishSimulation()
    } else {
      simulationFrame = requestAnimationFrame(step)
    }
  }

  simulationFrame = requestAnimationFrame(step)
}

onMounted(async () => {
  await load()
  await loadPlayer()
  if (!timeState.value) {
    await loadTime()
  }
})

onBeforeUnmount(() => {
  cancelSimulation()
})

const nodes = computed(() => {
  if (!graph.value) return [] as CityNode[]
  return Object.values(graph.value.nodes)
})

const edges = computed(() => {
  // 去重的无向边集合
  const s = new Set<string>()
  const list: Array<{ a: CityNode; b: CityNode }> = []
  if (!graph.value) return list
  for (const a of Object.values(graph.value.nodes)) {
    for (const nb of a.neighbors) {
      const b = graph.value.nodes[nb]
      if (!b) continue
      const key = [a.name, b.name].sort().join('::')
      if (!s.has(key)) {
        s.add(key)
        list.push({ a, b })
      }
    }
  }
  return list
})

// 视图窗口与坐标投影
const viewW = 640
const viewH = 400

const projector = computed(() => {
  if (nodes.value.length === 0) {
    return () => ({ x: 0, y: 0 })
  }
  const xs = nodes.value.map(n => n.coord.x)
  const ys = nodes.value.map(n => n.coord.y)
  const minX = Math.min(...xs)
  const maxX = Math.max(...xs)
  const minY = Math.min(...ys)
  const maxY = Math.max(...ys)
  const pad = 20
  const w = Math.max(1, maxX - minX)
  const h = Math.max(1, maxY - minY)
  const scale = Math.min((viewW - 2 * pad) / w, (viewH - 2 * pad) / h)
  return (p: Coordinate) => ({
    x: pad + (p.x - minX) * scale,
    y: viewH - (pad + (p.y - minY) * scale), // Y 轴向下
  })
})

function colorOf(terrain: Terrain): string {
  switch (terrain) {
    case 'plains': return '#a3d977'
    case 'desert': return '#e7c66a'
    case 'mountain': return '#9aa3ad'
    case 'forest': return '#6bbf59'
    case 'hills': return '#b2cc7a'
    case 'swamp': return '#6aa39a'
    case 'water': return '#68a8e3'
    default: return '#cccccc'
  }
}

const timeLabel = computed(() => {
  const t = displayTime.value ?? timeState.value
  if (!t) return '时间未知'
  const hourRounded = Math.round(t.hour * 10) / 10
  const hourText = Number.isInteger(hourRounded) ? hourRounded.toFixed(0) : hourRounded.toFixed(1)
  const label = `第${t.day} 天 · ${hourText} 小时`
  return isSimulating.value ? `${label}（行进中）` : label
})

const lastTravelLabel = computed(() => {
  if (lastTravelHours.value == null) return ''
  const hours = lastTravelHours.value
  return `上次行程耗时 ${hours.toFixed(2)} 小时`
})

function selectNode(name: string) {
  selected.value = name
}

function isNeighbor(n: CityNode): boolean {
  if (!selected.value || !graph.value) return false
  const cur = graph.value.nodes[selected.value]
  return cur?.neighbors.includes(n.name) ?? false
}

function distance(a: CityNode, b: CityNode): number {
  const dx = a.coord.x - b.coord.x
  const dy = a.coord.y - b.coord.y
  return Math.hypot(dx, dy)
}

const playerCity = computed(() => {
  if (!graph.value || !player.value) return null as CityNode | null
  return graph.value.nodes[player.value] ?? null
})

const playerMarkerPoint = computed(() => {
  const coord = simulatedPosition.value ?? playerCity.value?.coord ?? null
  if (!coord) return null
  return projector.value(coord)
})

function canMoveTo(n: CityNode): boolean {
  const p = playerCity.value
  if (!p) return false
  return p.neighbors.includes(n.name)
}

async function moveTo(name: string) {
  if (isSimulating.value) {
    error.value = '正在模拟行程，请稍候完成后再尝试'
    return
  }
  try {
    error.value = ''
    const originLocation = player.value
    const originCoord = originLocation && graph.value
      ? graph.value.nodes[originLocation]?.coord ?? null
      : null
    const before = timeState.value ? cloneTime(timeState.value) : null
    const res = await fetch('/api/player/move', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ to: name }),
    })
    if (!res.ok) {
      const msg = await res.text()
      throw new Error(msg || `HTTP ${res.status}`)
    }
    const data = await res.json()
    player.value = data.location as string
    selected.value = data.location as string
    const destinationCoord = graph.value
      ? graph.value.nodes[data.location]?.coord ?? null
      : null
    const rawDuration = typeof data.move_duration_hours === 'number' && Number.isFinite(data.move_duration_hours)
      ? data.move_duration_hours
      : null
    if (data.time) {
      timeState.value = data.time as TimeStatus
    }
    if (rawDuration != null && rawDuration > 0 && before && timeState.value) {
      lastTravelHours.value = rawDuration
      const startCoordForSimulation = originCoord && destinationCoord ? originCoord : null
      const endCoordForSimulation = originCoord && destinationCoord ? destinationCoord : null
      startSimulation(
        before,
        cloneTime(timeState.value),
        rawDuration,
        startCoordForSimulation,
        endCoordForSimulation,
      )
    } else {
      lastTravelHours.value = rawDuration != null ? Math.max(rawDuration, 0) : null
      syncDisplayTime(timeState.value)
      if (rawDuration != null && rawDuration > 0) {
        void loadPlayer()
      }
    }
  } catch (e: any) {
    error.value = e?.message ?? '移动失败'
    cancelSimulation()
  }
}
</script>

<template>
  <div class="map-panel">
    <div class="header">
      <strong>World Map</strong>
      <button class="reload" @click="load" :disabled="loading">{{ loading ? '加载中...' : '刷新' }}</button>
      <button @click="showDistances = !showDistances">{{ showDistances ? '隐藏距离' : '显示距离' }}</button>
      <button @click="moveMode = !moveMode">{{ moveMode ? '关闭移动模式' : '开启移动模式' }}</button>
      <span class="time">{{ timeLabel }}</span>
      <span v-if="lastTravelLabel" class="last-travel">{{ lastTravelLabel }}</span>
      <span v-if="error" class="error">{{ error }}</span>
    </div>

    <svg :viewBox="`0 0 ${viewW} ${viewH}`" class="map-canvas">
      <!-- 边 -->
      <g class="edges">
        <template v-for="e in edges" :key="e.a.name + '::' + e.b.name">
          <line
            :x1="projector(e.a.coord).x"
            :y1="projector(e.a.coord).y"
            :x2="projector(e.b.coord).x"
            :y2="projector(e.b.coord).y"
            stroke="#888"
            stroke-width="2"
            stroke-linecap="round"
            opacity="0.8"
          />
        </template>
      </g>

      <!-- 边距离标注 -->
      <g v-if="showDistances" class="edge-labels">
        <template v-for="e in edges" :key="'label::' + e.a.name + '::' + e.b.name">
          <text
            :x="(projector(e.a.coord).x + projector(e.b.coord).x) / 2"
            :y="(projector(e.a.coord).y + projector(e.b.coord).y) / 2 - 4"
            font-size="12"
            fill="#111"
            stroke="#fff"
            stroke-width="3"
            stroke-linejoin="round"
            paint-order="stroke"
          >{{ distance(e.a, e.b).toFixed(2) }}</text>
        </template>
      </g>

      <!-- 节点 -->
      <g class="nodes">
        <template v-for="n in nodes" :key="n.name">
          <g
            class="node"
            :transform="`translate(${projector(n.coord).x}, ${projector(n.coord).y})`"
            @click="(moveMode && canMoveTo(n)) ? moveTo(n.name) : selectNode(n.name)"
            style="cursor: pointer;"
          >
            <circle
              :r="selected === n.name ? 8 : (isNeighbor(n) ? 7 : 6)"
              :fill="selected === n.name ? '#ff6b6b' : colorOf(n.terrain)"
              stroke="#333"
              stroke-width="1.5"
            />
            <text
              x="10"
              y="4"
              font-size="12"
              fill="#222"
            >{{ n.name }}</text>
          </g>
        </template>
      </g>

      <!-- 玩家位置标记 -->
      <g v-if="playerMarkerPoint" class="player">
        <circle
          :cx="playerMarkerPoint.x"
          :cy="playerMarkerPoint.y"
          r="10"
          fill="none"
          stroke="#ff3b30"
          stroke-width="3"
        />
        <circle
          :cx="playerMarkerPoint.x"
          :cy="playerMarkerPoint.y"
          r="3"
          fill="#ff3b30"
        />
      </g>
    </svg>

    <div v-if="selected" class="info">
      选中：<strong>{{ selected }}</strong>
      <span v-if="lastTravelLabel" class="info-travel">{{ lastTravelLabel }}</span>
    </div>
  </div>
  
</template>

<style scoped>
.map-panel { border: 1px solid #ddd; border-radius: 8px; padding: 12px; background: #fff; }
.header { display: flex; align-items: center; gap: 8px; margin-bottom: 8px; }
.reload { margin-left: auto; }
.time { margin-left: 8px; color: #333; font-size: 0.95rem; white-space: nowrap; }
.last-travel { color: #555; font-size: 0.85rem; white-space: nowrap; }
.error { color: #d33; margin-left: 8px; }
.map-canvas { width: 100%; height: 420px; background: #f7f7f7; border: 1px solid #eee; border-radius: 6px; }
.info { margin-top: 8px; color: #444; }
.info-travel { margin-left: 12px; font-size: 0.9rem; color: #666; }
</style>

