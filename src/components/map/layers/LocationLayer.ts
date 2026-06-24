import { Container, Graphics, Text } from "pixi.js"
import { locations, type LocationData } from "../data/mapData"
import type { ThemeColors } from "../utils/themeUtils"

export function createLocationLayer(
  colors: ThemeColors,
  onSelect?: (loc: LocationData) => void,
): Container {
  const layer = new Container()

  for (const loc of locations) {
    const group = new Container()
    group.position.set(loc.x, loc.y)
    group.eventMode = "static"
    group.cursor = "pointer"

    // Outer ring
    const ring = new Graphics()
    ring.circle(0, 0, 7)
    ring.fill({ color: colors.card })
    ring.stroke({ width: 2, color: colors.foreground, alpha: 0.8 })
    group.addChild(ring)

    // Type indicator dot
    const dot = new Graphics()
    dot.circle(0, 0, 3)
    dot.fill({ color: dotColor(loc.type, colors) })
    group.addChild(dot)

    // Label
    const label = new Text({
      text: loc.name,
      style: {
        fontSize: 11,
        fill: colors.mutedForeground,
        fontFamily: "Inter, system-ui, sans-serif",
        fontWeight: "500",
      },
    })
    label.anchor.set(0.5, 0)
    label.position.set(0, 12)
    group.addChild(label)

    // Hover effects
    group.on("pointerover", () => {
      ring.tint = 0xcccccc
      label.style.fill = colors.foreground
    })
    group.on("pointerout", () => {
      ring.tint = 0xffffff
      label.style.fill = colors.mutedForeground
    })
    group.on("pointertap", () => {
      onSelect?.(loc)
    })

    layer.addChild(group)
  }

  return layer
}

function dotColor(type: LocationData["type"], colors: ThemeColors): number {
  switch (type) {
    case "town":       return colors.foreground
    case "dungeon":    return colors.destructive
    case "wilderness": return 0x5a9e6f  // green-ish
    case "landmark":   return colors.primary
  }
}
