<script setup lang="ts">
import { useRedisKeyInfo } from "@/core/redis";
import { watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { routes } from "@/shared/routes";

const route = useRoute();
const router = useRouter();

const { current } = useRedisKeyInfo();
const { fetchRedisKeyInfo } = useRedisKeyInfo();

watch(
	() => route.params.key,
	() => {
		fetchRedisKeyInfo(route.params.key as string, () => router.push({ name: routes.ROOT }));
	},
);
</script>

<template>
	<div v-if="current">
		<span>TTL: {{ current.ttl }} </span>
		<span>VALUE: {{ current.value }} </span>
	</div>
</template>
