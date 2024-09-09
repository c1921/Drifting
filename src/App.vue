<template>
  <div id="app">
    <Header :currentTime="gameStore.currentTime" :isRunning="gameStore.isRunning" @toggleTimer="toggleTimer" />
    <div class="layout">
      <CharacterPanel v-if="gameStore.selectedCharacter" :character="gameStore.selectedCharacter" :isPlayer="gameStore.isPlayer(gameStore.selectedCharacter)" />
      <Tabs
        :team="gameStore.team"
        :teamSpeed="gameStore.teamSpeed"
        :player="gameStore.player!"
        :items="gameStore.items"
        :travelDistance="gameStore.travelDistance"
        :isTraveling="gameStore.isTraveling"
        @memberSelected="gameStore.selectCharacter"
        @toggleTravelState="gameStore.toggleTravelState"
      />
    </div>
    <LogPanel :log="gameStore.log" />
    <Footer :currentTheme="currentTheme" @changeLanguage="changeLanguage" @toggleTheme="toggleTheme" />
  </div>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useGameStore } from './stores/gameStore';
import CharacterPanel from './components/CharacterPanel.vue';
import Tabs from './components/Tabs.vue';
import LogPanel from './components/LogPanel.vue';
import Header from './components/Header.vue';
import Footer from './components/Footer.vue';
import { TIMER_INTERVAL } from './constants';

const { t, locale } = useI18n();
const gameStore = useGameStore();

const currentTheme = ref<'light' | 'dark'>('light');

let timer: number | undefined;

const toggleTimer = () => {
  if (gameStore.isRunning) {
    clearInterval(timer);
  } else {
    timer = window.setInterval(gameStore.updateTime, TIMER_INTERVAL) as unknown as number;
  }
  gameStore.isRunning = !gameStore.isRunning;
};

const setTheme = (theme: 'light' | 'dark') => {
  document.documentElement.setAttribute('data-theme', theme);
  currentTheme.value = theme;
};

const toggleTheme = () => {
  const newTheme = currentTheme.value === 'light' ? 'dark' : 'light';
  setTheme(newTheme);
};

onMounted(() => {
  gameStore.initializeGame();
  timer = window.setInterval(gameStore.updateTime, TIMER_INTERVAL) as unknown as number;
  gameStore.isRunning = true;

  const prefersDarkScheme = window.matchMedia('(prefers-color-scheme: dark)');
  setTheme(prefersDarkScheme.matches ? 'dark' : 'light');

  prefersDarkScheme.addEventListener('change', (e) => {
    setTheme(e.matches ? 'dark' : 'light');
  });
});

onUnmounted(() => {
  clearInterval(timer);
});

const changeLanguage = (lang: string) => {
  locale.value = lang;
};
</script>

<style scoped>
.layout {
  display: flex;
}

.character-panel {
  width: 20%;
}

.tabs {
  width: 80%;
  margin-left: 1rem;
}

* {
  user-select: none;
}
</style>
