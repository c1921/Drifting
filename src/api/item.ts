import { invoke } from '@tauri-apps/api/core'
import type { ItemData } from '@/types/item'

/** 列出所有物品 */
export async function listItems(): Promise<ItemData[]> {
  return invoke<ItemData[]>('list_items')
}
