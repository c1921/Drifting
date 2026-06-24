<script setup lang="ts">
import {
  IconBarbell,
  IconBone,
  IconBrain,
  IconCalendar,
  IconClover,
  IconEye,
  IconFlask,
  IconGenderFemale,
  IconGenderMale,
  IconHandThreeFingers,
  IconHeart,
  IconMoon,
  IconRun,
  IconShield,
  IconShoe,
  IconStar,
  IconUser,
  IconUsers,
} from "@tabler/icons-vue"
import { computed } from "vue"
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip"

// ── Types ──────────────────────────────────────────
interface CharacterAttribute {
  key: string
  label: string
  abbreviation: string
  value: number
  icon: any
}

interface CharacterTrait {
  name: string
  icon: any
  description: string
}

interface Character {
  name: string
  gender: string
  age: number
  attributes: CharacterAttribute[]
  traits: CharacterTrait[]
}

// ── Mock data ───────────────────────────────────────
const character: Character = {
  name: "Elara Shadowveil",
  gender: "Female",
  age: 27,
  attributes: [
    { key: "strength", label: "Strength", abbreviation: "STR", value: 16, icon: IconBarbell },
    { key: "dexterity", label: "Dexterity", abbreviation: "DEX", value: 14, icon: IconRun },
    { key: "constitution", label: "Constitution", abbreviation: "CON", value: 15, icon: IconHeart },
    { key: "intelligence", label: "Intelligence", abbreviation: "INT", value: 12, icon: IconBrain },
    { key: "charisma", label: "Charisma", abbreviation: "CHA", value: 17, icon: IconStar },
  ],
  traits: [
    { name: "Tenacity", icon: IconShield, description: "Unwavering willpower in the face of adversity" },
    { name: "Alertness", icon: IconEye, description: "Keen perception; difficult to catch off guard" },
    { name: "Leadership", icon: IconUsers, description: "Born leader who inspires allies" },
    { name: "Luck", icon: IconClover, description: "Fortune favors the bold — lucky breaks come often" },
    { name: "Iron Stomach", icon: IconFlask, description: "Extreme resistance to toxins and harmful substances" },
    { name: "Night Vision", icon: IconMoon, description: "Can see clearly even in total darkness" },
    { name: "Fleet-Footed", icon: IconShoe, description: "Moves significantly faster than an ordinary person" },
    { name: "Quick Hands", icon: IconHandThreeFingers, description: "Nimble fingers, skilled at delicate tasks" },
    { name: "Hardy", icon: IconBone, description: "Exceptional physique and remarkable recovery" },
  ],
}

// ── Computed ───────────────────────────────────────
const genderIcon = computed(() =>
  character.gender === "Male" ? IconGenderMale : IconGenderFemale,
)
</script>

<template>
  <div class="flex flex-1 flex-col gap-6">
    <!-- ── Basic Info ──────────────────────────── -->
    <div class="flex items-center gap-5">
      <!-- Avatar placeholder -->
      <div
        class="bg-muted/50 flex size-20 shrink-0 items-center justify-center rounded-full"
      >
        <IconUser class="size-8 text-muted-foreground/50" />
      </div>

      <!-- Name / Gender / Age -->
      <div class="flex flex-col gap-1.5">
        <h2 class="text-2xl font-semibold tracking-tight">
          {{ character.name }}
        </h2>
        <div class="flex items-center gap-4 text-sm text-muted-foreground">
          <span class="inline-flex items-center gap-1.5">
            <component :is="genderIcon" class="size-4" />
            {{ character.gender }}
          </span>
          <span class="inline-flex items-center gap-1.5">
            <IconCalendar class="size-4" />
            {{ character.age }} yrs old
          </span>
        </div>
      </div>
    </div>

    <!-- ── Five Attributes ─────────────────────── -->
    <section>
      <h3 class="mb-3 text-sm font-medium text-muted-foreground">Attributes</h3>
      <div class="flex flex-col gap-1">
        <div
          v-for="attr in character.attributes"
          :key="attr.key"
          class="flex items-center gap-2 px-4 py-2.5"
        >
          <component :is="attr.icon" class="" />
          <span class="text-base font-bold tabular-nums">{{ attr.value }}</span>
        </div>
      </div>
    </section>

    <!-- ── Character Traits ────────────────────── -->
    <section>
      <h3 class="mb-3 text-sm font-medium text-muted-foreground">Traits</h3>
      <TooltipProvider>
        <div
          class="grid gap-2 grid-cols-[repeat(auto-fill,minmax(80px,1fr))]"
        >
          <Tooltip
            v-for="trait in character.traits"
            :key="trait.name"
          >
            <TooltipTrigger as-child>
              <div
                class="bg-muted/50 flex aspect-square cursor-default flex-col items-center justify-center gap-2 rounded-xl transition-colors hover:bg-muted"
              >
                <component :is="trait.icon" class="size-5" />
                <span class="text-xs">{{ trait.name }}</span>
              </div>
            </TooltipTrigger>
            <TooltipContent side="top">
              <p>{{ trait.description }}</p>
            </TooltipContent>
          </Tooltip>
        </div>
      </TooltipProvider>
    </section>
  </div>
</template>
