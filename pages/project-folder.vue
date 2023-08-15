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
					<select
						ref="version_selector"
						@change="update_mode_and_version"
						class="select select-bordered select-xs mr-2 focus:outline-none hover:border-primary cursor-default"
					>
						<option v-for="version in state.project.versions" :key="version.name">{{ version.name }}</option>
					</select>
					<div class="flex-grow" data-tauri-drag-region />
				</div>
				<div class="flex flex-col">
					<div class="flex-grow" data-tauri-drag-region />
					<select
						ref="mode_selector"
						@change="update_mode_and_version"
						class="select select-bordered select-xs mr-2 focus:outline-none hover:border-primary cursor-default"
					>
						<option v-for="opt in mode_options">{{ opt }}</option>
					</select>
					<div class="flex-grow" data-tauri-drag-region />
				</div>
			</div>
		</template>
		<template v-if="state.state === 'loading'">
			loading...
		</template>
		<template v-else-if="state.state === 'success'">
			<template v-if="selected_version !== ''">
				<workspace-build
					v-if="selected_mode === 'Build'"
					:selected-version="selected_version"
				/>
				<workspace-dev
					v-else-if="selected_mode === 'Develop'"
					:selected-version="selected_version"
				/>
				<template v-else>
					what
				</template>
			</template>
			<template v-else>
				please select a version
			</template>
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

	const mode_options = ["Build", "Develop"] as const;
	let selected_mode = ref<typeof mode_options[number]>("Build");
	let mode_selector = ref<HTMLSelectElement>();
	let selected_version = ref("");
	let version_selector = ref<HTMLSelectElement>();
	function update_mode_and_version() {
		if (state.value.state !== "success" || mode_selector.value === undefined || version_selector.value === undefined) {
			selected_version.value = "";
			return;
		} else {
			let i = version_selector.value.selectedIndex;
			let name = version_selector.value.children[i].innerHTML;
			selected_version.value = name;

			i = mode_selector.value.selectedIndex;
			let mode = mode_selector.value.children[i].innerHTML;
			selected_mode.value = mode as typeof mode_options[number];
		}
	}
	watch([state, mode_selector, version_selector], update_mode_and_version);

	invoke_get_project_basic_meta(path)
		.then(project => {
			state.value = { state: "success", project };
			invoke_add_recent_project(path);
		})
		.catch(e => {
			state.value = { state: "error", e };
		});
</script>
