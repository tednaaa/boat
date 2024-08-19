<script setup lang="ts">
import Connection from "@/shared/ui/Connection.vue";

import redisIcon from "@/assets/icons/redis.svg";
import RedisKeys from "../sidebar/RedisKeys.vue";
</script>

<template>
	<Connection class="items-end" label="some redis connection" :icon="redisIcon" />

	<RedisKeys />
</template>
