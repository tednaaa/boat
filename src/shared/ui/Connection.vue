<script setup lang="ts">
const props = defineProps<{
	icon: string;
	label: string;
}>();
</script>

<template>
	<div class="flex gap-2">
		<img class="size-5" :src="props.icon" alt="connection" />
		<span>{{ props.label }}</span>
	</div>
</template>
