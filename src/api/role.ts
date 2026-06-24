import { invoke } from '@tauri-apps/api/core'
import type { RoleData } from '@/types/role'

/** 列出所有已生成的角色 */
export async function listRoles(): Promise<RoleData[]> {
  return invoke<RoleData[]>('list_roles')
}
