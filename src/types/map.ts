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
  min_hab: number
}
