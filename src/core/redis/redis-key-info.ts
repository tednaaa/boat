import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

interface RedisKeyInfo {
	key: string;
	ttl: number;
	value: string;
}

function createUseRedisKeyInfo() {
	const current = ref<RedisKeyInfo | null>(null);

	return () => {
		const fetchRedisKeyInfo = async (key: string, onKeyNotFound: () => void) => {
			try {
				current.value = await invoke("fetch_redis_key_info", { key });
			} catch {
				onKeyNotFound();
			}
		};

		return { current, fetchRedisKeyInfo };
	};
}

export const useRedisKeyInfo = createUseRedisKeyInfo();
