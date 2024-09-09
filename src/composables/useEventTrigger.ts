import { Item } from '../types'
import { events } from '../types/event'
import {
    EVENT_TRIGGER_PROBABILITY,
    APPLE_EVENT_PROBABILITY
} from '../constants'

export function useEventTrigger() {
    const triggerEvents = (items: Item[], log: string[]) => {
        const random = Math.random()
        if (random < APPLE_EVENT_PROBABILITY) {
            events[0].effect(items, log)
        } else if (random < EVENT_TRIGGER_PROBABILITY) {
            events[1].effect(items, log)
        }
    }

    return {
        triggerEvents
    }
}