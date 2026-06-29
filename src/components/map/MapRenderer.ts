import { Application, Container, Point } from "pixi.js"
import { createContourLayer } from "./layers/ContourLayer"
import { createRoadLayer } from "./layers/RoadLayer"
import { createLocationLayer } from "./layers/LocationLayer"
import { createBoundaryLayer } from "./layers/BoundaryLayer"
import { createHabitabilityLayer } from "./layers/HabitabilityLayer"
import { getThemeColors, onThemeChange } from "./utils/themeUtils"
import { extractContours } from "@/map/contourExtractor"
import { getMap, regenerateMap } from "@/api/map"
import type { LocationData } from "@/types/map"

export interface MapHandle {
  destroy: () => void
  setHabitabilityVisible: (visible: boolean) => void
  regenerate: () => Promise<void>
}

export async function createMap(
  container: HTMLDivElement,
  options?: { onLocationSelect?: (loc: LocationData) => void },
): Promise<MapHandle> {
  const app = new Application()

  await app.init({
    resizeTo: container,
    backgroundAlpha: 0,
    antialias: true,
    resolution: Math.min(window.devicePixelRatio || 1, 2),
    autoDensity: true,
  })

  container.appendChild(app.canvas as HTMLCanvasElement)

  // ── 从后端获取地图数据 ──────────────────────────
  let mapData: import("@/types/map").MapData | null = null
  try {
    mapData = await getMap()
  } catch (e) {
    console.error("Failed to fetch map data:", e)
  }

  // ── Build layers ────────────────────────────────
  let habLayer: Container | null = null

  function buildWorld(map: import("@/types/map").MapData | null): Container {
    const colors = getThemeColors()
    const world = new Container()

    if (map) {
      // 层序：宜居度(底) → 等高线 → 省界 → 道路 → 地点(顶)
      const habLayer_ = createHabitabilityLayer(map.habitability, map.width, map.height)
      habLayer = habLayer_
      world.addChild(habLayer_)

      // 从高度网格提取等高线
      const contours = extractContours(map.heightmap, map.width, map.height)
      world.addChild(createContourLayer(contours, colors))

      const boundaryLayer = createBoundaryLayer(map.boundary, colors)
      if (boundaryLayer) world.addChild(boundaryLayer)

      world.addChild(createRoadLayer(map.roads, colors))
      world.addChild(createLocationLayer(map.locations, colors, options?.onLocationSelect))
    }

    return world
  }

  let world = buildWorld(mapData)
  app.stage.addChild(world)

  // ── Initial layout ──────────────────────────────
  const MIN_SCALE = 0.3
  const MAX_SCALE = 3.0

  /** Fit the map into the viewport */
  function resetView() {
    const { width, height } = app.screen
    const s = Math.min(width / 1600, height / 1600) * 0.85
    world.position.set(width / 2, height / 2)
    world.scale.set(s)
  }
  resetView()
  app.renderer.on("resize", resetView)

  // ── Pan (drag) ──────────────────────────────────
  let dragging = false
  let dragStart = new Point()
  let worldStartPos = new Point()

  app.stage.eventMode = "static"
  app.stage.hitArea = app.screen

  app.stage.on("pointerdown", (e) => {
    dragging = true
    dragStart.copyFrom(e.global)
    worldStartPos.copyFrom(world.position)
  })

  app.stage.on("pointermove", (e) => {
    if (!dragging) return
    world.position.set(
      worldStartPos.x + (e.global.x - dragStart.x),
      worldStartPos.y + (e.global.y - dragStart.y),
    )
  })

  app.stage.on("pointerup", () => {
    dragging = false
  })
  app.stage.on("pointerupoutside", () => {
    dragging = false
  })

  // ── Zoom (wheel) ────────────────────────────────
  const canvas = app.canvas as HTMLCanvasElement
  canvas.addEventListener("wheel", (e) => {
    e.preventDefault()

    const oldScale = world.scale.x
    const factor = e.deltaY > 0 ? 0.9 : 1.1
    const newScale = Math.min(MAX_SCALE, Math.max(MIN_SCALE, oldScale * factor))

    // Zoom towards cursor position
    const mouse = new Point(
      (e.clientX - world.position.x) / oldScale,
      (e.clientY - world.position.y) / oldScale,
    )

    world.scale.set(newScale)
    world.position.set(
      e.clientX - mouse.x * newScale,
      e.clientY - mouse.y * newScale,
    )
  }, { passive: false })

  // ── Rebuild on theme change ─────────────────────
  const stopThemeWatch = onThemeChange(() => {
    app.stage.removeChild(world)
    world.destroy({ children: true })
    world = buildWorld(mapData)
    app.stage.addChild(world)
    resetView()
  })

  return {
    destroy() {
      stopThemeWatch()
      app.destroy(true)
    },
    setHabitabilityVisible(visible: boolean) {
      if (habLayer) habLayer.visible = visible
    },
    async regenerate() {
      try {
        const newData = await regenerateMap()
        mapData = newData
      } catch (e) {
        console.error("Failed to regenerate map:", e)
        return
      }
      app.stage.removeChild(world)
      world.destroy({ children: true })
      world = buildWorld(mapData)
      app.stage.addChild(world)
      resetView()
    },
  }
}
