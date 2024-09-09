<template>
  <div>
    <p>{{ $t('travelDistance') }}: {{ travelDistance.toFixed(2) }} {{ $t('distanceUnit') }}</p>
    <button @click="toggleTravelState">{{ isTraveling ? $t('rest') : $t('travel') }}</button>
    
    <div v-if="passersby.length > 0">
      <h3>{{ $t('passersby') }}</h3>
      <ul>
        <li v-for="passerby in passersby" :key="passerby.character.id">
          {{ generateCharacterDescription(passerby.character) }}
          <button @click="talkTo(passerby.character)">{{ $t('talk') }}</button>
          <button @click="attack(passerby.character)">{{ $t('attack') }}</button>
          <button @click="invite(passerby.character)">{{ $t('invite') }}</button>
        </li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue';
import { useGameStore } from '../stores/gameStore';
import { storeToRefs } from 'pinia';
import { useCharacterGeneration } from '../composables/useCharacterGeneration';
import { Character } from '../types'; // 添加这行

export default defineComponent({
  name: 'TravelTab',
  props: {
    travelDistance: {
      type: Number,
      required: true
    },
    isTraveling: {
      type: Boolean,
      required: true
    }
  },
  emits: ['toggleTravelState'],
  setup(props, { emit }) {
    const gameStore = useGameStore();
    const { passersby } = storeToRefs(gameStore);
    const { generateCharacterDescription } = useCharacterGeneration();

    const toggleTravelState = () => {
      emit('toggleTravelState');
    };

    const talkTo = (character: Character) => {
      gameStore.interactWithPasserby('talk', character);
    };

    const attack = (character: Character) => {
      gameStore.interactWithPasserby('attack', character);
    };

    const invite = (character: Character) => {
      gameStore.interactWithPasserby('invite', character);
    };

    return { 
      toggleTravelState, 
      passersby, 
      generateCharacterDescription,
      talkTo,
      attack,
      invite
    };
  }
});
</script>
