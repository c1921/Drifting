import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { Character, Item } from '../types'
import { itemFactory } from '../factory/itemFactory'
import { useCharacterUpdate } from '../composables/useCharacterUpdate'
import { useEventTrigger } from '../composables/useEventTrigger'
import { useCharacterGeneration } from '../composables/useCharacterGeneration'
import {
  TIME_UPDATE_INTERVAL,
  TEAM_SIZE,
  MIN_INITIAL_APPLES,
  MAX_INITIAL_APPLES,
  MIN_INITIAL_SWORDS,
  MAX_INITIAL_SWORDS,
  MIN_INITIAL_SHIELDS,
  MAX_INITIAL_SHIELDS,
  // ... 其他导入的常量
} from '../constants'

export const useGameStore = defineStore('game', () => {
  const { updateCharacterWhileTraveling, updateCharacterWhileResting } = useCharacterUpdate()
  const { triggerEvents } = useEventTrigger()
  const { generateCharacter, getRandomValue } = useCharacterGeneration()

  const player = ref<Character | null>(null)
  const team = ref<Character[]>([])
  const selectedCharacter = ref<Character | null>(null)
  const travelDistance = ref(0)
  const isTraveling = ref(true)
  const items = ref<Item[]>([])
  const log = ref<string[]>([])
  const currentTime = ref(new Date())
  const isRunning = ref(false)

  const teamSpeed = computed(() => {
    return Math.min(...team.value.map(character => character.isRiding ? character.ridingSpeed : character.walkingSpeed))
  })

  function initializeGame() {
    player.value = generateCharacter()
    team.value = [player.value]
    for (let i = 0; i < TEAM_SIZE - 1; i++) {
      team.value.push(generateCharacter())
    }
    selectedCharacter.value = player.value

    items.value = [
      itemFactory('Apple', getRandomValue(MIN_INITIAL_APPLES, MAX_INITIAL_APPLES)),
      itemFactory('Sword', getRandomValue(MIN_INITIAL_SWORDS, MAX_INITIAL_SWORDS)),
      itemFactory('Wooden Shield', getRandomValue(MIN_INITIAL_SHIELDS, MAX_INITIAL_SHIELDS))
    ]
  }

  function selectCharacter(character: Character) {
    selectedCharacter.value = character
  }

  function isPlayer(character: Character) {
    return character.id === player.value?.id
  }

  function updateTime() {
    currentTime.value = new Date(currentTime.value.getTime() + TIME_UPDATE_INTERVAL)
    if (isTraveling.value) {
      travelDistance.value += teamSpeed.value
      team.value.forEach(updateCharacterWhileTraveling)
      triggerEvents(items.value, log.value)
    } else {
      team.value.forEach(updateCharacterWhileResting)
    }
  }

  function toggleTravelState() {
    isTraveling.value = !isTraveling.value
  }

  // 辅助函数
  function getRandomElement(arr: any[]) {
    return arr[Math.floor(Math.random() * arr.length)]
  }

  return {
    player,
    team,
    selectedCharacter,
    travelDistance,
    isTraveling,
    items,
    log,
    currentTime,
    isRunning,
    teamSpeed,
    initializeGame,
    selectCharacter,
    isPlayer,
    updateTime,
    toggleTravelState
  }
})