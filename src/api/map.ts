import { invoke } from '@tauri-apps/api/core'
import type { MapData } from '@/types/map'

/** 获取地图数据（首次调用时后端自动生成） */
export async function getMap(): Promise<MapData> {
  return invoke<MapData>('get_map')
}

/** 强制重新生成地图（新随机种子） */
export async function regenerateMap(): Promise<MapData> {
  return invoke<MapData>('regenerate_map')
}
