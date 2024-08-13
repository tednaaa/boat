<script setup lang="ts">
import { useRedisKeyInfo, useRedisKeys } from "../core";

const { redisKeys } = useRedisKeys();
const { fetchRedisKeyInfo } = useRedisKeyInfo();
</script>

<template>
	<div>
		<ul>
			<li v-for="key in redisKeys" :key="key">
				<button @click="fetchRedisKeyInfo(key)">
					{{ key }}
				</button>
			</li>
		</ul>
	</div>
</template>
