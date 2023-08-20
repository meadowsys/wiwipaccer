<template>
	<title-bar-container :title="title">
		<template #right>
			<div class="flex flex-row w-full h-full" data-tauri-drag-region>
				<div class="flex-grow" data-tauri-drag-region />
				<div v-if="state.state === 'success'" class="flex flex-col mr-[2px]">
					<div class="h-[2px] flex-shrink-0" data-tauri-drag-region />
					<select-component-thing ref="version_selector" :options="state.project.versions.map(v => v.name)" width="100px" :default="state.project.versions[0].name" />
					<div class="flex-grow" data-tauri-drag-region />
				</div>
				<div class="flex flex-col mr-[2px]">
					<div class="h-[2px] flex-shrink-0" data-tauri-drag-region />
					<select-component-thing ref="mode_selector" :options="mode_options" width="100px" :default="mode_options[0]" />
					<div class="flex-grow" data-tauri-drag-region />
				</div>
			</div>
		</template>
		<template v-if="state.state === 'loading'">
			loading...
		</template>
		<template v-else-if="state.state === 'success'">
			<template v-if="selected_version !== '' && selected_mode !== ''">
				<workspace-build
					v-if="selected_mode === 'build'"
					:selected-version="selected_version"
				/>
				<workspace-dev
					v-else-if="selected_mode === 'develop'"
					:selected-version="selected_version"
				/>
				<template v-else>
					what
				</template>
			</template>
			<template v-else>
				please select a version / mode
			</template>
		</template>
		<template v-else-if="state.state === 'error'">
			error: {{ state.e }}
		</template>
	</title-bar-container>
</template>

<script setup lang="ts">
	import { getCurrent } from "@tauri-apps/plugin-window";
	import type { ProjectSupportedVersions } from "~/composables/project-meta";
	import type SelectComponentThing from "~/components/SelectComponentThing.vue";

	const appWindow = getCurrent();

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

	const mode_options = ["build", "develop"] as const;

	let version_selector = ref<InstanceType<typeof SelectComponentThing>>();
	let mode_selector = ref<InstanceType<typeof SelectComponentThing>>();

	let selected_version = computed(() => version_selector.value?.option || "");
	let selected_mode = computed(() => mode_selector.value?.option || "");

	invoke_get_project_basic_meta(path)
		.then(project => {
			state.value = { state: "success", project };
			invoke_add_recent_project(path);
		})
		.catch(e => {
			state.value = { state: "error", e };
		});
</script>
