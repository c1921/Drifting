import { ref } from 'vue'
import { Character } from '../types'
import { v4 as uuidv4 } from 'uuid'
import {
  MIN_AGE,
  MAX_AGE,
  MIN_ATTRIBUTE,
  MAX_ATTRIBUTE,
  WALKING_SPEED,
  MIN_RIDING_SPEED,
  MAX_RIDING_SPEED,
  MAX_STAT_VALUE
} from '../constants'

export function useCharacterGeneration() {
  const names = ref(['Alice', 'Bob', 'Charlie', 'Diana', 'Edward', 'Fiona', 'George', 'Hannah', 'Ian', 'Julia'])
  const genders = ref(['male', 'female'])

  function generateCharacter(): Character {
    return {
      id: uuidv4(),
      name: getRandomElement(names.value),
      gender: getRandomElement(genders.value),
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

  function getRandomElement<T>(arr: T[]): T {
    return arr[Math.floor(Math.random() * arr.length)]
  }

  function getRandomValue(min: number, max: number): number {
    return Math.floor(Math.random() * (max - min + 1)) + min
  }

  function addName(name: string): void {
    names.value.push(name)
  }

  function addGender(gender: string): void {
    genders.value.push(gender)
  }

  return {
    generateCharacter,
    addName,
    addGender,
    names,
    genders,
    getRandomValue  // 添加这一行
  }
}
