import { Application, Container, Point } from "pixi.js"
import { createContourLayer } from "./layers/ContourLayer"
import { createRoadLayer } from "./layers/RoadLayer"
import { createLocationLayer } from "./layers/LocationLayer"
import { getThemeColors, onThemeChange } from "./utils/themeUtils"
import type { LocationData } from "./data/mapData"

export interface MapHandle {
  destroy: () => void
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

  // ── Build layers ────────────────────────────────
  function buildWorld(): Container {
    const colors = getThemeColors()
    const world = new Container()
    world.addChild(createContourLayer(colors))
    world.addChild(createRoadLayer(colors))
    world.addChild(createLocationLayer(colors, options?.onLocationSelect))
    return world
  }

  let world = buildWorld()
  app.stage.addChild(world)

  // ── Initial layout ──────────────────────────────
  const MIN_SCALE = 0.3
  const MAX_SCALE = 3.0

  /** Fit the map into the viewport */
  function resetView() {
    const { width, height } = app.screen
    const s = Math.min(width / 800, height / 600) * 0.85
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
    world = buildWorld()
    app.stage.addChild(world)
    resetView()
  })

  return {
    destroy() {
      stopThemeWatch()
      app.destroy(true)
    },
  }
}
