// ===========================================
// File: services/api.ts
// Description: API service layer for backend communication
// ===========================================

import type {
  GameState,
  InitGameRequest,
  InitGameResponse,
  Team,
  Character,
  TeamStats,
  TeamRestResponse,
} from '../types/game'

const API_BASE = '/api'

/**
 * 通用API请求封装
 */
async function apiRequest<T>(
  endpoint: string,
  options?: RequestInit
): Promise<T> {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
    ...options,
  })

  if (!response.ok) {
    const errorText = await response.text()
    throw new Error(errorText || `HTTP ${response.status}`)
  }

  return response.json()
}

/**
 * 初始化新游戏
 */
export async function initGame(
  request: InitGameRequest = {}
): Promise<InitGameResponse> {
  return apiRequest<InitGameResponse>('/game/init', {
    method: 'POST',
    body: JSON.stringify(request),
  })
}

/**
 * 获取游戏状态
 */
export async function getGameState(): Promise<GameState> {
  return apiRequest<GameState>('/game/state')
}

/**
 * 重置游戏
 */
export async function resetGame(): Promise<{ status: string; message: string }> {
  return apiRequest('/game/reset', {
    method: 'POST',
  })
}

/**
 * 获取队伍信息
 */
export async function getTeam(): Promise<Team> {
  return apiRequest<Team>('/team')
}

/**
 * 获取队伍成员列表
 */
export async function getTeamMembers(): Promise<{ characters: Character[]; total: number }> {
  return apiRequest<{ characters: Character[]; total: number }>('/team/members')
}

/**
 * 获取指定队员信息
 */
export async function getTeamMember(memberId: string): Promise<Character> {
  return apiRequest<Character>(`/team/members/${memberId}`)
}

/**
 * 获取队伍统计信息
 */
export async function getTeamStats(): Promise<TeamStats> {
  return apiRequest<TeamStats>('/team/stats')
}

/**
 * 队伍休息
 */
export async function teamRest(days: number = 1): Promise<TeamRestResponse> {
  return apiRequest<TeamRestResponse>(`/team/rest?days=${days}`, {
    method: 'POST',
  })
}

