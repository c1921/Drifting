<script setup lang="ts">
import {
  IconBell,
  IconCoin,
  IconCompass,
  IconFlag,
  IconSwords,
} from "@tabler/icons-vue"
import { Badge } from "@/components/ui/badge"

// ── Types ──────────────────────────────────────────
type EventType = "quest" | "combat" | "discovery" | "trade" | "system"

interface EventLogEntry {
  time: string
  type: EventType
  title: string
  content: string
}

interface TypeMeta {
  label: string
  badgeVariant: "default" | "secondary" | "destructive" | "outline"
  icon: any
}

const typeMeta: Record<EventType, TypeMeta> = {
  quest: {
    label: "Quest",
    badgeVariant: "default",
    icon: IconFlag,
  },
  combat: {
    label: "Combat",
    badgeVariant: "destructive",
    icon: IconSwords,
  },
  discovery: {
    label: "Discovery",
    badgeVariant: "secondary",
    icon: IconCompass,
  },
  trade: {
    label: "Trade",
    badgeVariant: "secondary",
    icon: IconCoin,
  },
  system: {
    label: "System",
    badgeVariant: "outline",
    icon: IconBell,
  },
}

// ── Mock data ───────────────────────────────────────
const events: EventLogEntry[] = [
  { time: "2026-06-15 09:32", type: "quest", title: "A New Beginning", content: "The hero embarks on a journey to retrieve the lost artifact from the ancient ruins." },
  { time: "2026-06-15 10:15", type: "combat", title: "Goblin Ambush", content: "A group of goblins attacked the party on the road. 3 goblins defeated, no casualties." },
  { time: "2026-06-15 11:00", type: "discovery", title: "Hidden Passage", content: "Found a concealed entrance behind the waterfall leading to the underground caverns." },
  { time: "2026-06-15 12:30", type: "trade", title: "Merchant Encounter", content: "Traded 5 iron ore for 2 health potions with a traveling merchant." },
  { time: "2026-06-15 14:00", type: "quest", title: "The Riddle of the Sphinx", content: "Solved the ancient riddle at the temple gate, gaining access to the inner sanctum." },
  { time: "2026-06-15 15:45", type: "system", title: "Level Up", content: "Elara Shadowveil reached level 5. New skill unlocked: Shadow Strike." },
  { time: "2026-06-15 16:20", type: "combat", title: "Wraith Encounter", content: "Encountered a spectral wraith in the crypt. Used silver weapons to banish it." },
  { time: "2026-06-15 18:00", type: "discovery", title: "Ancient Map Fragment", content: "Found a fragment of an ancient map inside the chest. It points toward the Crystal Peaks." },
]
</script>

<template>
  <div class="flex flex-col gap-3">
    <div
      v-for="event in events"
      :key="event.time + event.title"
      class="flex flex-col gap-1.5 rounded-lg border bg-card p-4 text-card-foreground shadow-xs"
    >
      <!-- Header: time + type badge -->
      <div class="flex items-center gap-3">
        <span class="text-xs text-muted-foreground tabular-nums">{{ event.time }}</span>
        <Badge :variant="typeMeta[event.type].badgeVariant" class="gap-1">
          <component :is="typeMeta[event.type].icon" class="size-3" />
          {{ typeMeta[event.type].label }}
        </Badge>
      </div>

      <!-- Title -->
      <h4 class="text-sm font-semibold leading-snug">{{ event.title }}</h4>

      <!-- Content -->
      <p class="text-sm text-muted-foreground leading-relaxed">{{ event.content }}</p>
    </div>
  </div>
</template>
