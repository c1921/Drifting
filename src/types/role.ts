/** 角色属性 */
export interface Attributes {
  strength: number
  dexterity: number
  constitution: number
  intelligence: number
  charisma: number
}

/** 特质信息 */
export interface TraitInfo {
  name: string
  description: string
}

/** 角色数据（匹配后端 RoleData） */
export interface RoleData {
  id: number
  name: string
  gender: string
  age: number
  attributes: Attributes
  traits: TraitInfo[]
}
