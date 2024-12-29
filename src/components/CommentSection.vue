<template>
  <div>
    <h2>Comments</h2>
    <ul>
      <li v-for="(comment, index) in comments" :key="index">
        {{ comment }}
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref } from 'vue';
import { initComments } from '../modules/commentSection';

export default defineComponent({
  name: 'CommentSection',
  setup() {
    const comments = ref<string[]>([]);

    onMounted(() => {
      comments.value = initComments();
    });

    return { comments };
  },
});
</script>
