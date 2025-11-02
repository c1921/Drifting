<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getTeam, getTeamStats, teamRest } from '../services/api'
import type { Team, TeamStats } from '../types/game'
import CharacterCard from './CharacterCard.vue'

const emit = defineEmits<{
  teamUpdated: []
}>()

const team = ref<Team | null>(null)
const stats = ref<TeamStats | null>(null)
const loading = ref(false)
const error = ref('')
const collapsed = ref(false)
const restDays = ref(1)
const resting = ref(false)

async function loadTeam() {
  loading.value = true
  error.value = ''
  
  try {
    const [teamData, statsData] = await Promise.all([
      getTeam(),
      getTeamStats(),
    ])
    
    team.value = teamData
    stats.value = statsData
  } catch (e: any) {
    error.value = e?.message ?? '加载队伍信息失败'
  } finally {
    loading.value = false
  }
}

async function handleRest() {
  if (!team.value || resting.value) return
  
  resting.value = true
  error.value = ''
  
  try {
    await teamRest(restDays.value)
    await loadTeam()
    emit('teamUpdated')
  } catch (e: any) {
    error.value = e?.message ?? '休息失败'
  } finally {
    resting.value = false
  }
}

const moraleColor = computed(() => {
  if (!team.value) return '#999'
  const morale = team.value.morale
  if (morale >= 70) return '#4CAF50'
  if (morale >= 40) return '#ff9800'
  return '#f44336'
})

onMounted(() => {
  loadTeam()
})

defineExpose({ loadTeam })
</script>

<template>
  <div class="team-panel" :class="{ collapsed }">
    <div class="panel-header" @click="collapsed = !collapsed">
      <h2>{{ team?.name || '队伍信息' }}</h2>
      <button class="collapse-btn">{{ collapsed ? '展开' : '收起' }}</button>
    </div>
    
    <div v-if="!collapsed" class="panel-content">
      <div v-if="loading" class="loading">加载中...</div>
      
      <div v-else-if="error" class="error">{{ error }}</div>
      
      <div v-else-if="team" class="team-info">
        <div class="info-section">
          <h3>队伍状态</h3>
          <div class="info-grid">
            <div class="info-item">
              <div class="info-label">队长</div>
              <div class="info-value">{{ team.leader }}</div>
            </div>
            <div class="info-item">
              <div class="info-label">位置</div>
              <div class="info-value">{{ team.location }}</div>
            </div>
            <div class="info-item">
              <div class="info-label">成员</div>
              <div class="info-value">{{ team.member_count }}人</div>
            </div>
            <div v-if="team.traveling" class="info-item">
              <div class="info-label">目的地</div>
              <div class="info-value">{{ team.destination }}</div>
            </div>
          </div>
        </div>
        
        <div class="info-section">
          <h3>资源</h3>
          <div class="resource-grid">
            <div class="resource-item">
              <div class="resource-icon">💰</div>
              <div class="resource-info">
                <div class="resource-label">金币</div>
                <div class="resource-value">{{ team.gold }}</div>
              </div>
            </div>
            <div class="resource-item">
              <div class="resource-icon">🍞</div>
              <div class="resource-info">
                <div class="resource-label">食物</div>
                <div class="resource-value">{{ team.food }}</div>
              </div>
            </div>
            <div class="resource-item">
              <div class="resource-icon">❤️</div>
              <div class="resource-info">
                <div class="resource-label">士气</div>
                <div class="resource-value" :style="{ color: moraleColor }">
                  {{ Math.round(team.morale) }}
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <div v-if="stats" class="info-section">
          <h3>统计</h3>
          <div class="stats-grid">
            <div class="stat-item">
              <div class="stat-label">平均生命</div>
              <div class="stat-value">{{ Math.round(stats.average_health) }}</div>
            </div>
            <div class="stat-item">
              <div class="stat-label">平均士气</div>
              <div class="stat-value">{{ Math.round(stats.average_morale) }}</div>
            </div>
            <div class="stat-item">
              <div class="stat-label">平均疲劳</div>
              <div class="stat-value">{{ Math.round(stats.average_fatigue) }}</div>
            </div>
            <div class="stat-item">
              <div class="stat-label">存活人数</div>
              <div class="stat-value">{{ stats.alive_count }}</div>
            </div>
          </div>
        </div>
        
        <div class="info-section">
          <h3>休息</h3>
          <div class="rest-controls">
            <label for="rest-days">休息天数:</label>
            <input
              id="rest-days"
              v-model.number="restDays"
              type="number"
              min="1"
              max="7"
              :disabled="resting"
            />
            <button
              class="rest-btn"
              :disabled="resting || team.traveling"
              @click="handleRest"
            >
              {{ resting ? '休息中...' : '开始休息' }}
            </button>
          </div>
          <div v-if="team.traveling" class="rest-note">
            正在旅行中，无法休息
          </div>
        </div>
        
        <div class="info-section">
          <div class="section-header">
            <h3>队员列表</h3>
            <button class="refresh-btn" @click="loadTeam" :disabled="loading">
              🔄 刷新
            </button>
          </div>
          <div class="members-list">
            <CharacterCard
              v-for="member in team.members"
              :key="member.id"
              :character="member"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.team-panel {
  background: white;
  border: 2px solid #ddd;
  border-radius: 12px;
  overflow: hidden;
  transition: all 0.3s;
}

.team-panel.collapsed .panel-content {
  display: none;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  cursor: pointer;
  user-select: none;
}

.panel-header h2 {
  margin: 0;
  font-size: 1.3rem;
}

.collapse-btn {
  background: rgba(255, 255, 255, 0.2);
  border: 1px solid rgba(255, 255, 255, 0.3);
  color: white;
  padding: 6px 12px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.2s;
}

.collapse-btn:hover {
  background: rgba(255, 255, 255, 0.3);
}

.panel-content {
  padding: 20px;
  max-height: calc(100vh - 200px);
  overflow-y: auto;
}

.loading,
.error {
  padding: 20px;
  text-align: center;
}

.error {
  color: #f44336;
}

.team-info {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.info-section {
  background: #f8f8f8;
  padding: 16px;
  border-radius: 10px;
}

.info-section h3 {
  margin: 0 0 12px;
  font-size: 1rem;
  color: #667eea;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.section-header h3 {
  margin: 0;
}

.refresh-btn {
  background: #667eea;
  border: none;
  color: white;
  padding: 6px 12px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.2s;
}

.refresh-btn:hover:not(:disabled) {
  background: #5568d3;
}

.refresh-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-label {
  font-size: 0.8rem;
  color: #666;
}

.info-value {
  font-size: 1rem;
  font-weight: 600;
  color: #333;
}

.resource-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.resource-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px;
  background: white;
  border-radius: 8px;
}

.resource-icon {
  font-size: 2rem;
}

.resource-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.resource-label {
  font-size: 0.75rem;
  color: #666;
}

.resource-value {
  font-size: 1.2rem;
  font-weight: 600;
  color: #333;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 12px;
  background: white;
  border-radius: 6px;
}

.stat-label {
  font-size: 0.85rem;
  color: #666;
}

.stat-value {
  font-size: 0.95rem;
  font-weight: 600;
  color: #667eea;
}

.rest-controls {
  display: flex;
  align-items: center;
  gap: 10px;
}

.rest-controls label {
  font-size: 0.9rem;
  color: #666;
}

.rest-controls input {
  width: 60px;
  padding: 6px 8px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 0.9rem;
}

.rest-btn {
  flex: 1;
  padding: 8px 16px;
  background: #4CAF50;
  border: none;
  color: white;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 500;
  transition: all 0.2s;
}

.rest-btn:hover:not(:disabled) {
  background: #45a049;
}

.rest-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.rest-note {
  margin-top: 8px;
  font-size: 0.8rem;
  color: #ff9800;
}

.members-list {
  display: flex;
  flex-direction: column;
}
</style>

