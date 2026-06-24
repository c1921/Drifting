import { Container, Graphics } from "pixi.js"
import { roads } from "../data/mapData"
import type { ThemeColors } from "../utils/themeUtils"

export function createRoadLayer(colors: ThemeColors): Container {
  const layer = new Container()

  for (const road of roads) {
    const pts = road.points
    if (pts.length < 4) continue

    const g = new Graphics()
    g.moveTo(pts[0], pts[1])

    for (let j = 2; j < pts.length - 1; j += 2) {
      const xc = (pts[j] + pts[j + 2]) / 2
      const yc = (pts[j + 1] + pts[j + 3]) / 2
      g.quadraticCurveTo(pts[j], pts[j + 1], xc, yc)
    }
    const last = pts.length - 2
    g.lineTo(pts[last], pts[last + 1])

    // Dashed line effect: light stroke
    g.stroke({
      width: 1.5,
      color: colors.border,
      alpha: 0.4,
    })

    layer.addChild(g)
  }

  return layer
}
