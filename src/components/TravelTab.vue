<template>
  <div>
    <p>{{ $t('travelDistance') }}: {{ travelDistance.toFixed(2) }} {{ $t('distanceUnit') }}</p>
    <button @click="toggleTravelState">{{ isTraveling ? $t('rest') : $t('travel') }}</button>
    
    <div v-if="passersby.length > 0">
      <h3>{{ $t('passersby') }}</h3>
      <ul>
        <li v-for="passerby in passersby" :key="passerby.character.id">
          {{ generateCharacterDescription(passerby.character) }}
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

    return { toggleTravelState, passersby, generateCharacterDescription };
  }
});
</script>
