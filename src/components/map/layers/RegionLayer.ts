import { Container, Graphics } from "pixi.js"
import type { RegionData } from "@/types/map"

/**
 * 区域范围图层：绘制各地区的多边形轮廓 + 半透明填充
 * 每个区域使用不同颜色，便于区分。
 */
export function createRegionLayer(regions: RegionData[]): Container | null {
  if (regions.length === 0) return null

  const layer = new Container()
  const palette = [
    0x4A90D9, // 蓝
    0xE67E22, // 橙
    0x2ECC71, // 绿
    0xE74C3C, // 红
    0x9B59B6, // 紫
    0x1ABC9C, // 青
    0xF39C12, // 黄
    0x3498DB, // 亮蓝
  ]

  for (let ri = 0; ri < regions.length; ri++) {
    const region = regions[ri]
    const poly = region.polygon
    if (poly.length < 6) continue // 至少 3 个点

    const color = palette[ri % palette.length]

    const g = new Graphics()

    // 半透明填充
    g.moveTo(poly[0], poly[1])
    for (let i = 2; i < poly.length - 1; i += 2) {
      g.lineTo(poly[i], poly[i + 1])
    }
    g.closePath()

    g.fill({ color, alpha: 0.15 })

    // 描边
    g.stroke({ width: 2.5, color, alpha: 0.7 })

    layer.addChild(g)
  }

  return layer
}
