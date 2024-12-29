<template>
  <div>
    <h2>Bear Data</h2>
    <div v-if="bearData">
      <p><strong>Name:</strong> {{ bearData.name }}</p>
      <p><strong>Type:</strong> {{ bearData.type }}</p>
    </div>
    <div v-else>
      <p>Loading bear data...</p>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from 'vue';
import { getBearData } from '../modules/bearData';

export default defineComponent({
  name: 'BearData',
  setup() {
    const bearData = ref(null);

    onMounted(async () => {
      bearData.value = await getBearData();
    });

    return { bearData };
  },
});
</script>
