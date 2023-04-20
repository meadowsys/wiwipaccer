<template>
	<div class="h-full flex flex-row">
		<div
			class="bg-opacity-100 bg-white py-10 px-14 flex flex-col h-full select-none"
			data-tauri-drag-region
		>
			<div class="cursor-default text-4xl px-2 font-permanent-marker" data-tauri-drag-region>wiwipaccer</div>
			<div class="cursor-default text-neutral-500 px-2 font-permanent-marker" data-tauri-drag-region>Pack Builder&trade;</div>

			<div class="flex-grow" data-tauri-drag-region />

			<welcome-header-button disabled>
				New Project (Soon&trade;)
			</welcome-header-button>
			<welcome-header-button @click="invoke_open_project">
				Open Project
			</welcome-header-button>
			<welcome-header-button @click="invoke_open_docs">
				Documentation
			</welcome-header-button>
			<welcome-header-button disabled>
				Changelog
			</welcome-header-button>
			<welcome-header-button @click="invoke_open_about">
				About
			</welcome-header-button>
		</div>

		<welcome-project-history :recents="recents" />

		<!-- <div class="py-8 px-12">
			<table class="w-full">
				<welcome-recents-entry-button v-for="item in recent_stuff" :item="item" />
			</table>
		</div> -->
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
