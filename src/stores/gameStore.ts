import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { Character, Item } from '../types'
import { itemFactory } from '../factory/itemFactory'
import { v4 as uuidv4 } from 'uuid'
import { Event, events } from '../types/event'
import { useCharacterUpdate } from '../composables/useCharacterUpdate'
import { useEventTrigger } from '../composables/useEventTrigger'
import {
  TIME_UPDATE_INTERVAL,
  SATIETY_DECREASE_RATE,
  HYDRATION_DECREASE_RATE,
  STAMINA_DECREASE_RATE,
  MOOD_DECREASE_RATE,
  STAMINA_INCREASE_RATE,
  MOOD_INCREASE_RATE,
  MAX_STAT_VALUE,
  EVENT_TRIGGER_PROBABILITY,
  APPLE_EVENT_PROBABILITY,
  TEAM_SIZE,
  MIN_AGE,
  MAX_AGE,
  MIN_ATTRIBUTE,
  MAX_ATTRIBUTE,
  WALKING_SPEED,
  MIN_RIDING_SPEED,
  MAX_RIDING_SPEED,
  MIN_INITIAL_APPLES,
  MAX_INITIAL_APPLES,
  MIN_INITIAL_SWORDS,
  MAX_INITIAL_SWORDS,
  MIN_INITIAL_SHIELDS,
  MAX_INITIAL_SHIELDS
} from '../constants'

export const useGameStore = defineStore('game', () => {
  const { updateCharacterWhileTraveling, updateCharacterWhileResting } = useCharacterUpdate()
  const { triggerEvents } = useEventTrigger()

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

  function generateCharacter(): Character {
    return {
      id: uuidv4(),
      name: getRandomElement(['Alice', 'Bob', 'Charlie', 'Diana', 'Edward']),
      gender: getRandomElement(['male', 'female']),
      age: getRandomValue(MIN_AGE, MAX_AGE),
      strength: getRandomValue(MIN_ATTRIBUTE, MAX_ATTRIBUTE),
      agility: getRandomValue(MIN_ATTRIBUTE, MAX_ATTRIBUTE),
      charisma: getRandomValue(MIN_ATTRIBUTE, MAX_ATTRIBUTE),
      intelligence: getRandomValue(MIN_ATTRIBUTE, MAX_ATTRIBUTE),
      walkingSpeed: WALKING_SPEED,
      ridingSpeed: getRandomValue(MIN_RIDING_SPEED, MAX_RIDING_SPEED),
      isRiding: false,
      satiety: MAX_STAT_VALUE,
      hydration: MAX_STAT_VALUE,
      stamina: MAX_STAT_VALUE,
      mood: MAX_STAT_VALUE
    }
  }

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

  function getRandomValue(min: number, max: number) {
    return Math.floor(Math.random() * (max - min + 1)) + min
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