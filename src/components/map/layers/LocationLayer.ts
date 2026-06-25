import { Container, Graphics, Text } from "pixi.js"
import type { ThemeColors } from "../utils/themeUtils"
import type { LocationData } from "@/types/map"

export function createLocationLayer(
  locations: LocationData[],
  colors: ThemeColors,
  onSelect?: (loc: LocationData) => void,
): Container {
  const layer = new Container()

  for (const loc of locations) {
    const group = new Container()
    group.position.set(loc.x, loc.y)
    group.eventMode = "static"
    group.cursor = "pointer"

    const ringRadius = ringSize(loc.type)
    const dotRadius = dotSize(loc.type)

    // Outer ring
    const ring = new Graphics()
    ring.circle(0, 0, ringRadius)
    ring.fill({ color: colors.card })
    ring.stroke({ width: 2, color: colors.foreground, alpha: 0.8 })
    group.addChild(ring)

    // Type indicator dot
    const dot = new Graphics()
    dot.circle(0, 0, dotRadius)
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
    label.position.set(0, ringRadius + 6)
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

/** 根据地点类型返回外环半径 */
function ringSize(type: LocationData["type"]): number {
  switch (type) {
    case "large":  return 9   // 城市 — 大
    case "medium": return 7   // 小镇 — 中
    case "small":  return 5   // 村庄 — 小
  }
}

/** 根据地点类型返回内点半径 */
function dotSize(type: LocationData["type"]): number {
  switch (type) {
    case "large":  return 4.5 // 城市 — 大
    case "medium": return 3   // 小镇 — 中
    case "small":  return 2   // 村庄 — 小
  }
}

/** 根据地点类型返回标识颜色 */
function dotColor(type: LocationData["type"], colors: ThemeColors): number {
  switch (type) {
    case "large":  return 0x22c55e  // 城市 — 绿色
    case "medium": return 0xeab308  // 小镇 — 黄色
    case "small":  return colors.foreground // 村庄 — 默认色
  }
}
