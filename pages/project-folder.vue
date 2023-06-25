<template>
	<title-bar-container :title="title">
		<!-- <template v-if="state.state === 'loading'">
			weofijjoiefwoijiojfweiojfweiojiojefw loading h
		</template> -->
		<template #right>
			<div class="flex flex-row w-full h-full" data-tauri-drag-region>
				<div class="flex-grow" data-tauri-drag-region />
				<div v-if="state.state === 'success'" class="flex flex-col">
					<div class="flex-grow" data-tauri-drag-region />
					<select class="select select-bordered select-xs mr-2 focus:outline-none hover:border-primary cursor-default">
						<option v-for="version in state.project.versions">{{ version.name }}</option>
					</select>
					<div class="flex-grow" data-tauri-drag-region />
				</div>
				<div class="flex flex-col">
					<div class="flex-grow" data-tauri-drag-region />
					<select class="select select-bordered select-xs mr-2 focus:outline-none hover:border-primary cursor-default">
						<option selected>Build</option>
						<option>Develop</option>
					</select>
					<div class="flex-grow" data-tauri-drag-region />
				</div>
			</div>
		</template>
		<template v-if="state.state === 'loading'">
			loading...
		</template>
		<template v-else-if="state.state === 'success'">
		</template>
		<template v-else-if="state.state === 'error'">
			error: {{ state.e }}
		</template>
	</title-bar-container>
</template>

<script setup lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	import type { ProjectSupportedVersions } from "~/composables/project-meta"

	let path = await invoke_decode_hex_string(appWindow.label);

	type Loading = {
		state: "loading";
	};

	type Success = {
		state: "success";
		project: ProjectSupportedVersions;
	};

	type Error = {
		state: "error";
		e: any;
	};

	type State = Loading | Success | Error;

	let state = ref<State>({ state: "loading" });
	let title = computed(() => {
		if (state.value.state === "loading") return "Loading...";

		if (state.value.state === "success") {
			let names = state.value.project.names
			if (names.length === 1) return names[0];
			return `Workspace: ${names.join(", ")}`;
		}

		if (state.value.state === "error") return "Error in loading workspace";
		return `if you see this title please report it: ${state.value}`;
	});

	invoke_get_project_basic_meta(path)
		.then(project => {
			state.value = { state: "success", project };
			invoke_add_recent_project(path);
		})
		.catch(e => {
			state.value = { state: "error", e };
		});
</script>
