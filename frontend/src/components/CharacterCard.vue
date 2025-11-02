<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Character } from '../types/game'

const props = defineProps<{
  character: Character
}>()

const expanded = ref(false)

const roleLabel = computed(() => {
  const roleMap: Record<string, string> = {
    traveler: '旅行者',
    guard: '守卫',
    merchant: '商人',
    scout: '侦察兵',
    healer: '治疗师',
  }
  return roleMap[props.character.role] || props.character.role
})

const controlTypeLabel = computed(() => {
  return props.character.control_type === 'player' ? '玩家' : 'AI'
})

const controlTypeColor = computed(() => {
  return props.character.control_type === 'player' ? '#4CAF50' : '#2196F3'
})

const healthPercentage = computed(() => props.character.health)
const moralePercentage = computed(() => props.character.morale)
const fatiguePercentage = computed(() => props.character.fatigue)

function getBarColor(value: number, reverse = false): string {
  if (reverse) {
    // For fatigue (higher is worse)
    if (value >= 70) return '#f44336'
    if (value >= 40) return '#ff9800'
    return '#4CAF50'
  } else {
    // For health and morale (higher is better)
    if (value >= 70) return '#4CAF50'
    if (value >= 40) return '#ff9800'
    return '#f44336'
  }
}

const traitLabels = computed(() => {
  const traitMap: Record<string, string> = {
    brave: '勇敢',
    cautious: '谨慎',
    greedy: '贪婪',
    loyal: '忠诚',
    stoic: '坚忍',
    curious: '好奇',
    coward: '懦弱',
    charismatic: '魅力',
    kind: '善良',
    selfish: '自私',
    lazy: '懒惰',
    diligent: '勤奋',
    schemer: '谋士',
    romantic: '浪漫',
  }
  return props.character.traits.map(t => traitMap[t] || t)
})
</script>

<template>
  <div class="character-card" :class="{ expanded }" @click="expanded = !expanded">
    <div class="card-header">
      <div class="char-icon">
        {{ character.control_type === 'player' ? '👤' : '🤖' }}
      </div>
      <div class="char-info">
        <div class="char-name">{{ character.name }}</div>
        <div class="char-meta">
          <span class="role">{{ roleLabel }}</span>
          <span class="control-type" :style="{ color: controlTypeColor }">
            {{ controlTypeLabel }}
          </span>
        </div>
      </div>
      <div class="expand-icon">{{ expanded ? '▼' : '▶' }}</div>
    </div>
    
    <div class="card-body">
      <div class="stat-bars">
        <div class="stat-bar">
          <div class="stat-label">
            <span>生命值</span>
            <span class="stat-value">{{ character.health }}</span>
          </div>
          <div class="bar-track">
            <div
              class="bar-fill"
              :style="{
                width: `${healthPercentage}%`,
                backgroundColor: getBarColor(healthPercentage)
              }"
            />
          </div>
        </div>
        
        <div class="stat-bar">
          <div class="stat-label">
            <span>士气</span>
            <span class="stat-value">{{ character.morale }}</span>
          </div>
          <div class="bar-track">
            <div
              class="bar-fill"
              :style="{
                width: `${moralePercentage}%`,
                backgroundColor: getBarColor(moralePercentage)
              }"
            />
          </div>
        </div>
        
        <div class="stat-bar">
          <div class="stat-label">
            <span>疲劳度</span>
            <span class="stat-value">{{ character.fatigue }}</span>
          </div>
          <div class="bar-track">
            <div
              class="bar-fill"
              :style="{
                width: `${fatiguePercentage}%`,
                backgroundColor: getBarColor(fatiguePercentage, true)
              }"
            />
          </div>
        </div>
      </div>
      
      <div v-if="expanded" class="expanded-content">
        <div class="details-section">
          <h4>基础信息</h4>
          <div class="detail-row">
            <span>年龄:</span>
            <span>{{ character.age }}岁</span>
          </div>
          <div class="detail-row">
            <span>性别:</span>
            <span>{{ character.gender }}</span>
          </div>
          <div class="detail-row">
            <span>出身:</span>
            <span>{{ character.origin || '未知' }}</span>
          </div>
          <div class="detail-row">
            <span>声誉:</span>
            <span>{{ character.reputation }}</span>
          </div>
        </div>
        
        <div class="details-section">
          <h4>属性</h4>
          <div class="attributes-grid">
            <div class="attr-item">
              <div class="attr-label">力量</div>
              <div class="attr-value">{{ character.attributes.strength }}</div>
            </div>
            <div class="attr-item">
              <div class="attr-label">智力</div>
              <div class="attr-value">{{ character.attributes.intelligence }}</div>
            </div>
            <div class="attr-item">
              <div class="attr-label">魅力</div>
              <div class="attr-value">{{ character.attributes.charisma }}</div>
            </div>
            <div class="attr-item">
              <div class="attr-label">耐力</div>
              <div class="attr-value">{{ character.attributes.endurance }}</div>
            </div>
            <div class="attr-item">
              <div class="attr-label">感知</div>
              <div class="attr-value">{{ character.attributes.perception }}</div>
            </div>
          </div>
        </div>
        
        <div class="details-section">
          <h4>技能</h4>
          <div class="skills-grid">
            <div class="skill-item">
              <span>交易</span>
              <span class="skill-value">{{ character.skills.trading }}</span>
            </div>
            <div class="skill-item">
              <span>战斗</span>
              <span class="skill-value">{{ character.skills.combat }}</span>
            </div>
            <div class="skill-item">
              <span>医疗</span>
              <span class="skill-value">{{ character.skills.medicine }}</span>
            </div>
            <div class="skill-item">
              <span>导航</span>
              <span class="skill-value">{{ character.skills.navigation }}</span>
            </div>
            <div class="skill-item">
              <span>谈判</span>
              <span class="skill-value">{{ character.skills.negotiation }}</span>
            </div>
          </div>
        </div>
        
        <div v-if="character.traits.length > 0" class="details-section">
          <h4>性格特质</h4>
          <div class="traits">
            <span v-for="trait in traitLabels" :key="trait" class="trait-tag">
              {{ trait }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.character-card {
  background: white;
  border: 2px solid #e0e0e0;
  border-radius: 12px;
  padding: 16px;
  margin-bottom: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.character-card:hover {
  border-color: #667eea;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.character-card.expanded {
  border-color: #667eea;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.char-icon {
  font-size: 2rem;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 50%;
}

.char-info {
  flex: 1;
}

.char-name {
  font-size: 1.1rem;
  font-weight: 600;
  color: #333;
}

.char-meta {
  display: flex;
  gap: 12px;
  font-size: 0.85rem;
  margin-top: 2px;
}

.role {
  color: #666;
}

.control-type {
  font-weight: 600;
}

.expand-icon {
  color: #999;
  font-size: 0.8rem;
}

.card-body {
  border-top: 1px solid #f0f0f0;
  padding-top: 12px;
}

.stat-bars {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.stat-bar {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-label {
  display: flex;
  justify-content: space-between;
  font-size: 0.85rem;
  color: #666;
}

.stat-value {
  font-weight: 600;
  color: #333;
}

.bar-track {
  height: 6px;
  background: #f0f0f0;
  border-radius: 3px;
  overflow: hidden;
}

.bar-fill {
  height: 100%;
  transition: width 0.3s, background-color 0.3s;
  border-radius: 3px;
}

.expanded-content {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #f0f0f0;
  animation: expandIn 0.3s ease-out;
}

@keyframes expandIn {
  from {
    opacity: 0;
    max-height: 0;
  }
  to {
    opacity: 1;
    max-height: 1000px;
  }
}

.details-section {
  margin-bottom: 16px;
}

.details-section:last-child {
  margin-bottom: 0;
}

.details-section h4 {
  margin: 0 0 8px;
  font-size: 0.9rem;
  color: #667eea;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.detail-row {
  display: flex;
  justify-content: space-between;
  padding: 4px 0;
  font-size: 0.9rem;
}

.detail-row span:first-child {
  color: #666;
}

.detail-row span:last-child {
  color: #333;
  font-weight: 500;
}

.attributes-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 8px;
}

.attr-item {
  text-align: center;
  padding: 8px;
  background: #f8f8f8;
  border-radius: 8px;
}

.attr-label {
  font-size: 0.75rem;
  color: #666;
  margin-bottom: 4px;
}

.attr-value {
  font-size: 1.2rem;
  font-weight: 600;
  color: #667eea;
}

.skills-grid {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.skill-item {
  display: flex;
  justify-content: space-between;
  padding: 6px 12px;
  background: #f8f8f8;
  border-radius: 6px;
  font-size: 0.9rem;
}

.skill-value {
  font-weight: 600;
  color: #667eea;
}

.traits {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.trait-tag {
  display: inline-block;
  padding: 4px 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 16px;
  font-size: 0.8rem;
  font-weight: 500;
}
</style>

