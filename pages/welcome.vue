<template>
	<div class="h-full flex flex-row">
		<div class="py-10 px-6 flex flex-col h-full select-none" data-tauri-drag-region>
			<div class="select-none cursor-default font-permanent-marker text-4xl" data-tauri-drag-region>wiwipaccer</div>
			<div class="select-none cursor-default font-permanent-marker opacity-60" data-tauri-drag-region>Pack Builder&trade;</div>

			<div class="flex-grow" data-tauri-drag-region />

			<button
				class="btn block btn-sm normal-case btn-outline mt-1 text-left"
				disabled
			>
				New Project (Soon&trade;)
			</button>
			<button
				class="btn block btn-sm normal-case btn-outline mt-1 text-left btn-primary"
				@click="invoke_open_project()"
			>
				Open Project
			</button>
			<button
				class="btn block btn-sm normal-case btn-outline mt-1 text-left"
				@click="invoke_open_docs"
			>
				Documentation
			</button>
			<button
				class="btn block btn-sm normal-case btn-outline mt-1 text-left"
				disabled
			>
				Changelog
			</button>
			<button
				class="btn block btn-sm normal-case btn-outline mt-1 text-left"
				@click="invoke_open_about"
			>
				About
			</button>
		</div>

		<!-- <div class="my-10 border border-base-200" /> -->

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
