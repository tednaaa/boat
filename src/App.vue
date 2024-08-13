<script setup lang="ts">
import RedisKeyInfo from "./redis/components/RedisKeyInfo.vue";
import RedisKeys from "./redis/components/RedisKeys.vue";
</script>

<template>
	<div>
		<RedisKeys />
		<RedisKeyInfo />
	</div>
</template>
