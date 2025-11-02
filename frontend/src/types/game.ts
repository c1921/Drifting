// ===========================================
// File: types/game.ts
// Description: TypeScript type definitions for game entities
// ===========================================

export interface Character {
  id: string
  name: string
  gender: string
  age: number
  control_type: 'player' | 'AI'
  role: string
  origin: string | null
  attributes: {
    strength: number
    intelligence: number
    charisma: number
    endurance: number
    perception: number
  }
  skills: {
    trading: number
    combat: number
    medicine: number
    navigation: number
    negotiation: number
  }
  traits: string[]
  mood: {
    happiness: number
    fear: number
    anger: number
  }
  health: number
  fatigue: number
  morale: number
  loyalty: number
  is_alive: boolean
  reputation: number
}

export interface Team {
  name: string
  leader: string
  leader_id: string
  is_player_controlled: boolean
  members: Character[]
  member_count: number
  gold: number
  food: number
  capacity: number
  morale: number
  speed: number
  location: string
  destination: string | null
  distance_to_destination: number
  traveling: boolean
}

export interface TeamStats {
  total_health: number
  average_health: number
  total_morale: number
  average_morale: number
  total_fatigue: number
  average_fatigue: number
  alive_count: number
  dead_count: number
}

export interface GameState {
  is_initialized: boolean
  current_location: string
  elapsed_hours: number
  current_day: number
  current_hour: number
  player_team: {
    name: string
    members: string[]
    gold: number
    food: number
    morale: number
    location: string
    traveling: boolean
  } | null
}

export interface InitGameRequest {
  player_name?: string
  team_name?: string
  starting_location?: string
}

export interface InitGameResponse {
  is_initialized: boolean
  current_location: string
  elapsed_hours: number
  current_day: number
  current_hour: number
  player_team: {
    name: string
    members: string[]
    gold: number
    food: number
    morale: number
    location: string
    traveling: boolean
  }
}

export interface TeamRestResponse {
  status: string
  message: string
  current_day: number
  current_hour: number
  team_morale: number
}

