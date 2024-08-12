<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const keys = ref([]);

const getKeys = async () => {
	keys.value = await invoke('fetch_redis_keys');
	console.log(keys.value);
};
</script>

<template>
	<div>
		<button @click="getKeys">Fetch Redis Keys</button>
		<ul>
			<li v-for="key in keys" :key="key">{{ key }}</li>
		</ul>
	</div>
</template>
