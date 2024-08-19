<script setup lang="ts">
import NewConnection from "../new-connection/NewConnection.vue";
import RedisConnection from "../redis-connection/RedisConnection.vue";
</script>

<template>
	<div class="bg-neutral-800 rounded-lg p-3">
		<NewConnection />
		<RedisConnection />
	</div>
</template>
