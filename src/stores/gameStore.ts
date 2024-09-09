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
  PASSERBY_APPEAR_CHANCE,
  MIN_PASSERBY_DURATION,
  MAX_PASSERBY_DURATION,
  MAX_PASSERBY_COUNT
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

  const passersby = ref<{ character: Character; expirationTime: number }[]>([])

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
    const now = Date.now()
    currentTime.value = new Date(now)
    if (isTraveling.value) {
      travelDistance.value += teamSpeed.value
      team.value.forEach(updateCharacterWhileTraveling)
      triggerEvents(items.value, log.value)
      
      handlePassersby(now)
    } else {
      team.value.forEach(updateCharacterWhileResting)
    }
  }

  function handlePassersby(now: number) {
    // 移除已经过期的路人
    passersby.value = passersby.value.filter(p => now < p.expirationTime)

    // 如果路人数量少于最大值,有机会生成新的路人
    if (passersby.value.length < MAX_PASSERBY_COUNT && Math.random() < PASSERBY_APPEAR_CHANCE) {
      const newPasserby = generateCharacter()
      const duration = getRandomValue(MIN_PASSERBY_DURATION, MAX_PASSERBY_DURATION)
      const expirationTime = now + duration

      passersby.value.push({ character: newPasserby, expirationTime })
    }
  }

  function toggleTravelState() {
    isTraveling.value = !isTraveling.value
    // 如果停止旅行，暂停所有路人的计时器
    if (!isTraveling.value) {
      const now = Date.now()
      passersby.value.forEach(p => {
        p.expirationTime += (TIME_UPDATE_INTERVAL - (now % TIME_UPDATE_INTERVAL))
      })
    }
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
    toggleTravelState,
    passersby
  }
})