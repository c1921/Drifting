<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'

type Terrain = 'plains' | 'desert' | 'mountain' | 'forest' | 'hills' | 'swamp' | 'water'

interface Coordinate { x: number; y: number }
interface CityNode {
  name: string
  coord: Coordinate
  terrain: Terrain
  neighbors: string[]
}
interface Graph { nodes: Record<string, CityNode> }

const loading = ref(true)
const error = ref<string>('')
const graph = ref<Graph | null>(null)
const selected = ref<string | null>(null)
const showDistances = ref(false)

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

onMounted(load)

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
</script>

<template>
  <div class="map-panel">
    <div class="header">
      <strong>World Map</strong>
      <button class="reload" @click="load" :disabled="loading">{{ loading ? '加载中...' : '刷新' }}</button>
      <button @click="showDistances = !showDistances">{{ showDistances ? '隐藏距离' : '显示距离' }}</button>
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
            @click="selectNode(n.name)"
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
    </svg>

    <div v-if="selected" class="info">
      选中：<strong>{{ selected }}</strong>
    </div>
  </div>
  
</template>

<style scoped>
.map-panel { border: 1px solid #ddd; border-radius: 8px; padding: 12px; background: #fff; }
.header { display: flex; align-items: center; gap: 8px; margin-bottom: 8px; }
.reload { margin-left: auto; }
.error { color: #d33; margin-left: 8px; }
.map-canvas { width: 100%; height: 420px; background: #f7f7f7; border: 1px solid #eee; border-radius: 6px; }
.info { margin-top: 8px; color: #444; }
</style>

