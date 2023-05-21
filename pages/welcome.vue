<template>
	<div class="h-full flex flex-row">
		<div
			class="bg-opacity-100 bg-white py-10 px-14 flex flex-col h-full select-none"
			data-tauri-drag-region
		>
			<div class="cursor-default text-4xl px-2 font-permanent-marker" data-tauri-drag-region>wiwipaccer</div>
			<div class="cursor-default text-neutral-500 px-2 font-permanent-marker" data-tauri-drag-region>Pack Builder&trade;</div>

			<div class="flex-grow" data-tauri-drag-region />

			<wiwi-button disabled>
				New Project (Soon&trade;)
			</wiwi-button>
			<wiwi-button @click="invoke_open_project">
				Open Project
			</wiwi-button>
			<wiwi-button @click="invoke_open_docs">
				Documentation
			</wiwi-button>
			<wiwi-button disabled>
				Changelog
			</wiwi-button>
			<wiwi-button @click="invoke_open_about">
				About
			</wiwi-button>
		</div>

		<div class="my-10 border border-neutral-200" />

		<welcome-project-history :recents="recents" />
	</div>
</template>

<script setup lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	let recents = ref<Array<{ name: string, path: string }>>([]);

	function update_recents() {
		invoke_get_recent_projects()
			.then(v => v.map(([name, path]) => ({ name, path })))
			.then(v => {
				while (recents.value.pop());
				recents.value.push(...v);
			});
	}

	const unlisten = await appWindow.listen("refresh-recents", update_recents);

	onMounted(() => {
		update_recents();
	});

	onUnmounted(() => {
		unlisten();
	});
</script>
