<script setup lang="ts">
import { useRedisKeys } from "./core/redis";

const { redisKeys } = useRedisKeys();
</script>

<template>
	<div>
		<ul>
			<li v-for="key in redisKeys" :key="key">{{ key }}</li>
		</ul>
	</div>
</template>
