<script setup lang="ts">
import { useRedisKeys } from "@/core/redis";
import { routes } from "@/shared/routes";
import { SearchOutline } from "@vicons/ionicons5";
import InputText from "primevue/inputtext";
import { computed, ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const { redisKeys } = useRedisKeys();

const searchValue = ref("");

const filteredKeys = computed(() => {
	return redisKeys.value.filter((key) => key.toLowerCase().includes(searchValue.value.toLowerCase()));
});
</script>

<template>
	<div>
		<div class="bg-neutral-700 relative mt-4 px-4 pr-10 py-2 rounded-lg">
			<InputText class="w-full bg-transparent text-white outline-none" type="text" v-model="searchValue" placeholder="Search keys..." />
			<SearchOutline class="absolute top-1/2 -translate-y-1/2 right-2" width="24" />
		</div>

		<ul class="mt-4">
			<li v-for="key in filteredKeys" :key="key">
				<button @click="router.push({ name: routes.REDIS, params: { key } })">
					{{ key }}
				</button>
			</li>
		</ul>
	</div>
</template>
