<script setup lang="ts">
import Sidebar from "./ui/sidebar/Sidebar.vue";
</script>

<template>
	<div class="fixed h-full w-full flex gap-4 p-4">
		<Sidebar class="min-w-96 p-4" />
		<div class="p-4 w-full rounded-lg bg-neutral-800">
			<slot></slot>
		</div>
	</div>
</template>
