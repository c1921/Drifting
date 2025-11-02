<script setup lang="ts">
import { ref } from 'vue'
import { initGame } from '../services/api'
import type { InitGameResponse } from '../types/game'

const emit = defineEmits<{
  initialized: [response: InitGameResponse]
}>()

const playerName = ref('旅行者')
const teamName = ref('漂流者')
const startingLocation = ref('Capital')
const loading = ref(false)
const error = ref('')
const showDetails = ref(false)
const initResponse = ref<InitGameResponse | null>(null)

async function handleInit() {
  if (!playerName.value.trim()) {
    error.value = '请输入玩家名称'
    return
  }
  
  if (!teamName.value.trim()) {
    error.value = '请输入队伍名称'
    return
  }

  loading.value = true
  error.value = ''
  
  try {
    const response = await initGame({
      player_name: playerName.value,
      team_name: teamName.value,
      starting_location: startingLocation.value,
    })
    
    initResponse.value = response
    showDetails.value = true
  } catch (e: any) {
    error.value = e?.message ?? '初始化失败'
  } finally {
    loading.value = false
  }
}

function startGame() {
  if (initResponse.value) {
    emit('initialized', initResponse.value)
  }
}
</script>

<template>
  <div class="modal-overlay">
    <div class="modal-content">
      <div v-if="!showDetails" class="init-form">
        <h1 class="title">🌊 Drifting</h1>
        <p class="subtitle">欢迎来到漂流世界</p>
        
        <div class="form-group">
          <label for="player-name">玩家名称</label>
          <input
            id="player-name"
            v-model="playerName"
            type="text"
            placeholder="输入你的名字"
            :disabled="loading"
            @keyup.enter="handleInit"
          />
        </div>
        
        <div class="form-group">
          <label for="team-name">队伍名称</label>
          <input
            id="team-name"
            v-model="teamName"
            type="text"
            placeholder="为你的队伍命名"
            :disabled="loading"
            @keyup.enter="handleInit"
          />
        </div>
        
        <div class="form-group">
          <label for="starting-location">起始地点</label>
          <select
            id="starting-location"
            v-model="startingLocation"
            :disabled="loading"
          >
            <option value="Capital">Capital (首都)</option>
            <option value="Harbor">Harbor (港口)</option>
            <option value="Farmland">Farmland (农田)</option>
          </select>
        </div>
        
        <div v-if="error" class="error-message">
          {{ error }}
        </div>
        
        <button
          class="start-button"
          :disabled="loading"
          @click="handleInit"
        >
          {{ loading ? '初始化中...' : '开始游戏' }}
        </button>
        
        <div class="info-text">
          游戏将生成1个玩家角色和2个NPC队友
        </div>
      </div>
      
      <div v-else class="init-success">
        <h2 class="success-title">✨ 队伍组建成功！</h2>
        
        <div class="team-summary">
          <div class="summary-item">
            <span class="label">队伍名称:</span>
            <span class="value">{{ initResponse?.player_team.name }}</span>
          </div>
          <div class="summary-item">
            <span class="label">起始地点:</span>
            <span class="value">{{ initResponse?.current_location }}</span>
          </div>
          <div class="summary-item">
            <span class="label">队伍成员:</span>
            <span class="value">{{ initResponse?.player_team.members.join(', ') }}</span>
          </div>
          <div class="summary-item">
            <span class="label">初始金币:</span>
            <span class="value">{{ initResponse?.player_team.gold }} 💰</span>
          </div>
          <div class="summary-item">
            <span class="label">初始食物:</span>
            <span class="value">{{ initResponse?.player_team.food }} 🍞</span>
          </div>
        </div>
        
        <button class="enter-button" @click="startGame">
          进入游戏
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.modal-content {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 16px;
  padding: 40px;
  max-width: 500px;
  width: 100%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  animation: slideIn 0.3s ease-out;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.init-form {
  color: white;
}

.title {
  font-size: 3rem;
  margin: 0 0 0.5rem;
  text-align: center;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
}

.subtitle {
  text-align: center;
  font-size: 1.2rem;
  margin: 0 0 2rem;
  opacity: 0.95;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  font-size: 0.95rem;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 12px 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 8px;
  font-size: 1rem;
  background: rgba(255, 255, 255, 0.9);
  color: #333;
  transition: all 0.2s;
  box-sizing: border-box;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: rgba(255, 255, 255, 0.8);
  background: white;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.form-group input:disabled,
.form-group select:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.error-message {
  background: rgba(255, 59, 48, 0.9);
  color: white;
  padding: 12px;
  border-radius: 8px;
  margin-bottom: 1rem;
  font-size: 0.9rem;
}

.start-button {
  width: 100%;
  padding: 14px;
  background: white;
  color: #667eea;
  border: none;
  border-radius: 8px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.start-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
}

.start-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.info-text {
  text-align: center;
  margin-top: 1rem;
  font-size: 0.85rem;
  opacity: 0.8;
}

.init-success {
  color: white;
}

.success-title {
  font-size: 2rem;
  margin: 0 0 2rem;
  text-align: center;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
}

.team-summary {
  background: rgba(255, 255, 255, 0.15);
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 2rem;
  backdrop-filter: blur(10px);
}

.summary-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.2);
}

.summary-item:last-child {
  border-bottom: none;
}

.summary-item .label {
  font-weight: 500;
  opacity: 0.9;
}

.summary-item .value {
  font-weight: 600;
}

.enter-button {
  width: 100%;
  padding: 14px;
  background: white;
  color: #667eea;
  border: none;
  border-radius: 8px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.enter-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
}
</style>

