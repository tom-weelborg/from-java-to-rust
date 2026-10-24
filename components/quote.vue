<template>
    <div class="flex flex-col justify-center gap-2">
        <a :href="sourceUrl" target="_blank" style="border-style: none">
            <h3 class="font-bold">
                “{{ quote }}”
            </h3>
            <span class="ml-4">
                – {{ quotee }}
            </span>
        </a>
    </div>
</template>

<script setup lang="ts">
defineProps<{
    quote: string;
    quotee: string;
    sourceUrl: string;
}>();
</script>
