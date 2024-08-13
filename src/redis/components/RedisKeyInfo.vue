<script setup lang="ts">
import { useRedisKeyInfo } from "../core";

const { current } = useRedisKeyInfo();
</script>

<template>
	<div v-if="current">
		<span>TTL: {{ current.ttl }} </span>
		<span>VALUE: {{ current.value }} </span>
	</div>
</template>
