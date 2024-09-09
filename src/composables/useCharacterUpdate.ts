import { Character } from '../types'
import {
    SATIETY_DECREASE_RATE,
    HYDRATION_DECREASE_RATE,
    STAMINA_DECREASE_RATE,
    MOOD_DECREASE_RATE,
    STAMINA_INCREASE_RATE,
    MOOD_INCREASE_RATE,
    MAX_STAT_VALUE
} from '../constants'

export function useCharacterUpdate() {
    const updateCharacterWhileTraveling = (character: Character) => {
        character.satiety = parseFloat((Math.max(character.satiety - SATIETY_DECREASE_RATE, 0)).toFixed(2))
        character.hydration = parseFloat((Math.max(character.hydration - HYDRATION_DECREASE_RATE, 0)).toFixed(2))
        character.stamina = parseFloat((Math.max(character.stamina - STAMINA_DECREASE_RATE, 0)).toFixed(2))
        character.mood = parseFloat((Math.max(character.mood - MOOD_DECREASE_RATE, 0)).toFixed(2))
    }

    const updateCharacterWhileResting = (character: Character) => {
        character.stamina = parseFloat((Math.min(character.stamina + STAMINA_INCREASE_RATE, MAX_STAT_VALUE)).toFixed(2))
        character.mood = parseFloat((Math.min(character.mood + MOOD_INCREASE_RATE, MAX_STAT_VALUE)).toFixed(2))
    }

    return {
        updateCharacterWhileTraveling,
        updateCharacterWhileResting
    }
}