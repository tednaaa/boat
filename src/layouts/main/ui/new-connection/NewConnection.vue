<script setup lang="ts">
import Button from "primevue/button";

import { Add, AddCircle, AddCircleOutline, SettingsOutline, TimeOutline } from "@vicons/ionicons5";
</script>

<template>
	<div class="flex gap-2 mb-6">
		<Button class="flex h-14 items-center w-full border justify-center py-4 border-neutral-500 rounded-lg gap-3">
			<div class="p-0.5 size-6 rounded-full bg-neutral-700">
				<Add class="text-white" />
			</div>
			<span class="text-nowrap">New Connection</span>
		</Button>
		<div class="flex gap-3">
			<Button class="flex items-center justify-center bg-neutral-700 border border-neutral-500 size-14 rounded-lg">
				<SettingsOutline width="24" />
			</Button>
			<Button class="flex items-center justify-center bg-neutral-700 border border-neutral-500 size-14 rounded-lg">
				<TimeOutline width="24" />
			</Button>
		</div>
	</div>
</template>
