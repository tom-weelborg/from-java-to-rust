<template>
    <a :href="sourceUrl" target="_blank">
        <img :src="imageUrl" :alt="imageAlt" :class="imageClass" />
    </a>
</template>

<script setup lang="ts">
defineProps<{
    imageUrl: string;
    imageAlt: string;
    imageClass: string;
    sourceUrl: string;
}>();
</script>
