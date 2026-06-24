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
  IconStar as IconTrait,
} from "@tabler/icons-vue"
import { computed } from "vue"
import type { RoleData } from "@/types/role"
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip"

// ── Props ──────────────────────────────────────────
const props = defineProps<{
  role: RoleData | null
}>()

// ── Computed ───────────────────────────────────────
const genderIcon = computed(() =>
  props.role?.gender === "Male" ? IconGenderMale : IconGenderFemale,
)

// 属性展示配置
const attributeConfig = [
  { key: "strength" as const, label: "Strength", abbreviation: "STR", icon: IconBarbell },
  { key: "dexterity" as const, label: "Dexterity", abbreviation: "DEX", icon: IconRun },
  { key: "constitution" as const, label: "Constitution", abbreviation: "CON", icon: IconHeart },
  { key: "intelligence" as const, label: "Intelligence", abbreviation: "INT", icon: IconBrain },
  { key: "charisma" as const, label: "Charisma", abbreviation: "CHA", icon: IconStar },
]
</script>

<template>
  <div v-if="role" class="flex flex-1 flex-col gap-6">
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
          {{ role.name }}
        </h2>
        <div class="flex items-center gap-4 text-sm text-muted-foreground">
          <span class="inline-flex items-center gap-1.5">
            <component :is="genderIcon" class="size-4" />
            {{ role.gender }}
          </span>
          <span class="inline-flex items-center gap-1.5">
            <IconCalendar class="size-4" />
            {{ role.age }} yrs old
          </span>
        </div>
      </div>
    </div>

    <!-- ── Five Attributes ─────────────────────── -->
    <section>
      <h3 class="mb-3 text-sm font-medium text-muted-foreground">Attributes</h3>
      <div class="flex flex-col gap-1">
        <div
          v-for="attr in attributeConfig"
          :key="attr.key"
          class="flex items-center gap-2 px-4 py-2.5"
        >
          <component :is="attr.icon" class="" />
          <span class="text-base font-bold tabular-nums">{{ role.attributes[attr.key] }}</span>
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
            v-for="trait in role.traits"
            :key="trait.name"
          >
            <TooltipTrigger as-child>
              <div
                class="bg-muted/50 flex aspect-square cursor-default flex-col items-center justify-center gap-2 rounded-xl transition-colors hover:bg-muted"
              >
                <IconTrait class="size-5" />
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

  <!-- 无角色时显示空状态 -->
  <div v-else class="flex flex-1 flex-col items-center justify-center gap-4 p-12">
    <IconUser class="size-16 text-muted-foreground/30" />
    <p class="text-lg font-medium text-muted-foreground">No role selected</p>
    <p class="text-sm text-muted-foreground/70 text-center">
      Generate a role in the "Create Role" tab or select one from the list.
    </p>
  </div>
</template>
