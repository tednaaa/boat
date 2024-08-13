import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

export function useRedisKeys() {
	const redisKeys = ref<string[]>([]);

	const getKeys = async () => {
		redisKeys.value = await invoke("fetch_redis_keys");
	};

	setInterval(getKeys, 3000);

	return { redisKeys };
}
