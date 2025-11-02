<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getGameState, resetGame } from './services/api'
import GameInitModal from './components/GameInitModal.vue'
import TeamPanel from './components/TeamPanel.vue'
import MapView from './components/MapView.vue'
import type { InitGameResponse } from './types/game'

const gameInitialized = ref(false)
const checkingState = ref(true)
const showResetConfirm = ref(false)
const teamPanelRef = ref<InstanceType<typeof TeamPanel> | null>(null)
const mapViewRef = ref<InstanceType<typeof MapView> | null>(null)

async function checkGameState() {
  checkingState.value = true
  
  try {
    const state = await getGameState()
    gameInitialized.value = state.is_initialized
  } catch (e) {
    // 游戏未初始化或出错，显示初始化界面
    gameInitialized.value = false
  } finally {
    checkingState.value = false
  }
}

function handleGameInitialized(response: InitGameResponse) {
  console.log('Game initialized:', response)
  gameInitialized.value = true
  
  // 刷新地图和队伍信息
  if (mapViewRef.value) {
    mapViewRef.value.loadPlayer?.()
  }
  if (teamPanelRef.value) {
    teamPanelRef.value.loadTeam?.()
  }
}

async function handleResetGame() {
  try {
    await resetGame()
    gameInitialized.value = false
    showResetConfirm.value = false
  } catch (e: any) {
    console.error('Reset failed:', e)
    alert('重置失败: ' + (e?.message ?? '未知错误'))
  }
}

function handleTeamUpdated() {
  // 队伍更新后刷新地图（时间可能改变）
  if (mapViewRef.value) {
    mapViewRef.value.loadTime?.()
  }
}

onMounted(() => {
  checkGameState()
})
</script>

<template>
  <div id="app">
    <div v-if="checkingState" class="checking-state">
      <div class="spinner"></div>
      <p>检查游戏状态...</p>
    </div>
    
    <GameInitModal
      v-else-if="!gameInitialized"
      @initialized="handleGameInitialized"
    />
    
    <div v-else class="game-layout">
      <header class="game-header">
        <h1>🌊 Drifting</h1>
        <div class="header-actions">
          <button
            class="reset-btn"
            @click="showResetConfirm = true"
          >
            重置游戏
          </button>
        </div>
      </header>
      
      <div class="game-content">
        <aside class="sidebar">
          <TeamPanel
            ref="teamPanelRef"
            @team-updated="handleTeamUpdated"
          />
        </aside>
        
        <main class="main-area">
          <MapView ref="mapViewRef" />
        </main>
      </div>
    </div>
    
    <!-- Reset Confirmation Modal -->
    <div v-if="showResetConfirm" class="modal-overlay" @click.self="showResetConfirm = false">
      <div class="confirm-modal">
        <h2>确认重置游戏</h2>
        <p>重置游戏将清除所有进度，确定要继续吗？</p>
        <div class="modal-actions">
          <button class="cancel-btn" @click="showResetConfirm = false">
            取消
          </button>
          <button class="confirm-btn" @click="handleResetGame">
            确认重置
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
* {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  background: #f5f5f5;
}

#app {
  width: 100%;
  min-height: 100vh;
}

.checking-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  color: #666;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.game-layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.game-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 16px 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.game-header h1 {
  margin: 0;
  font-size: 1.8rem;
  font-weight: 600;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.reset-btn {
  background: rgba(255, 255, 255, 0.2);
  border: 1px solid rgba(255, 255, 255, 0.3);
  color: white;
  padding: 8px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s;
}

.reset-btn:hover {
  background: rgba(255, 255, 255, 0.3);
}

.game-content {
  display: flex;
  flex: 1;
  gap: 20px;
  padding: 20px;
  overflow: hidden;
}

.sidebar {
  width: 400px;
  flex-shrink: 0;
  overflow-y: auto;
}

.main-area {
  flex: 1;
  overflow-y: auto;
}

.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  padding: 20px;
}

.confirm-modal {
  background: white;
  border-radius: 12px;
  padding: 24px;
  max-width: 400px;
  width: 100%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.confirm-modal h2 {
  margin: 0 0 12px;
  font-size: 1.5rem;
  color: #333;
}

.confirm-modal p {
  margin: 0 0 24px;
  color: #666;
  line-height: 1.6;
}

.modal-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.cancel-btn,
.confirm-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-size: 0.95rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.cancel-btn {
  background: #e0e0e0;
  color: #333;
}

.cancel-btn:hover {
  background: #d0d0d0;
}

.confirm-btn {
  background: #f44336;
  color: white;
}

.confirm-btn:hover {
  background: #d32f2f;
}

/* Responsive Design */
@media (max-width: 1024px) {
  .game-content {
    flex-direction: column;
  }
  
  .sidebar {
    width: 100%;
  }
}

@media (max-width: 640px) {
  .game-header {
    flex-direction: column;
    gap: 12px;
    text-align: center;
  }
  
  .game-header h1 {
    font-size: 1.5rem;
  }
  
  .game-content {
    padding: 12px;
    gap: 12px;
  }
}
</style>
