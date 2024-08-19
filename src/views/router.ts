import { createRouter, createWebHashHistory } from "vue-router";
import { routes } from "@/shared/routes";

export const router = createRouter({
	history: createWebHashHistory(),
	routes: [
		{
			path: "/",
			name: routes.ROOT,
			component: () => import("./root/Root.page.vue"),
		},
		{
			path: "/redis/:key",
			name: routes.REDIS,
			component: () => import("./redis/Redis.page.vue"),
		},
	],
});
