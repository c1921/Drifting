import { Container, Graphics } from "pixi.js"
import { contours } from "../data/mapData"
import type { ThemeColors } from "../utils/themeUtils"

export function createContourLayer(colors: ThemeColors): Container {
  const layer = new Container()

  for (let i = 0; i < contours.length; i++) {
    const c = contours[i]
    const pts = c.points
    if (pts.length < 4) continue

    const g = new Graphics()

    // Smooth curve using quadratic bezier
    g.moveTo(pts[0], pts[1])
    for (let j = 2; j < pts.length - 1; j += 2) {
      const xc = (pts[j] + pts[j + 2]) / 2
      const yc = (pts[j + 1] + pts[j + 3]) / 2
      g.quadraticCurveTo(pts[j], pts[j + 1], xc, yc)
    }
    // Close the last segment back to start
    const last = pts.length - 2
    g.quadraticCurveTo(pts[last], pts[last + 1], pts[0], pts[1])
    g.closePath()

    // Faded fill + thin stroke
    const alpha = 0.04 + (i / contours.length) * 0.06
    g.fill({ color: colors.mutedForeground, alpha })
    g.stroke({
      width: 0.5 + (i / contours.length) * 0.8,
      color: colors.mutedForeground,
      alpha: 0.15 + (i / contours.length) * 0.15,
    })

    layer.addChild(g)
  }

  return layer
}
