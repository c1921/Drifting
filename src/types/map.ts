/** 地点类型（匹配后端生成：large=城市, medium=小镇, small=村庄） */
export type LocationType = "large" | "medium" | "small"

/** 地点数据 */
export interface LocationData {
  id: string
  name: string
  x: number
  y: number
  type: LocationType
}

/** 道路折线 */
export interface RoadData {
  points: number[] // flattened [x1,y1,x2,y2,...]
}

/** 地图数据（匹配后端 MapData） */
export interface MapData {
  heightmap: number[]
  width: number
  height: number
  locations: LocationData[]
  roads: RoadData[]
}
