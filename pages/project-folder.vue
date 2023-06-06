<template>
	<title-bar-container :title="title">
		<!-- <template v-if="state.state === 'loading'">
			weofijjoiefwoijiojfweiojfweiojiojefw loading h
		</template> -->
	</title-bar-container>
</template>

<script setup lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	let path = await invoke_decode_hex_string(appWindow.label);

	type Loading = {
		state: "loading";
	};

	type State = Loading;

	let title = ref("Loading...");
	let state = ref<State>({ state: "loading" });
</script>

<!-- <script setup lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	let path = await invoke_decode_hex_string(appWindow.label);

	type Loading = {
		state: "loading";
	};

	type Success = {
		state: "success";
		supported_versions: Awaited<ReturnType<typeof invoke_get_project_supported_versions>>;
	};

	type Error = {
		state: "error";
		e: any;
	};

	let things = ref<Loading | Success | Error>({ state: "loading" });
	let title = computed(() => {
		if (things.value.state === "loading") return "Loading...";

		if (things.value.state === "success") {
			let names = things.value.supported_versions.names
			if (names.length === 1) return names[0];
			return `Workspace: ${names.join(", ")}`;
		}

		if (things.value.state === "error") return "Error in loading workspace";
		throw "this should never happen lol";
	});
	invoke_get_project_supported_versions(path)
		.then(supported_versions => {
			things.value = { state: "success", supported_versions };
			invoke_add_recent_project(path);
		})
		.catch(e => {
			things.value = { state: "error", e };
		});
</script> -->
