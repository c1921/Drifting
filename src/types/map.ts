export type LocationType = "large" | "medium" | "small"

export interface LocationData {
  id: string
  name: string
  x: number
  y: number
  type: LocationType
}

export interface RoadData {
  points: number[] // flattened [x1,y1,x2,y2,...]
}

export interface RegionData {
  id: string
  polygon: number[] // flattened [x0,y0,x1,y1,...] world-coord closed ring
}

export interface MapData {
  heightmap: number[]
  habitability: number[] // 512×512, 0..1 宜居度（海拔+坡度）
  flat_center: number[]   // 512×512, 0..1 平坦区域中心度
  width: number
  height: number
  boundary: number[] // flattened [x0,y0,x1,y1,...] world-coord closed ring
  locations: LocationData[]
  roads: RoadData[]
  regions: RegionData[]
  contours: ContourData[] // 后端预计算的等高线
  min_hab: number
}

/** 后端返回的单条等高线（与 contourExtractor.ts 的 ContourData 兼容） */
export interface ContourData {
  points: number[] // flattened [x, y, x, y, ...] 世界坐标
  closed: boolean  // 是否为闭合环
  level: number    // 该等高线对应的高度阈值 (0~1)
}
