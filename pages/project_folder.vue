<template>
	<title-bar-container :title="`Project: ${path}`">
		wheeeeeeeeeeeee path: {{ path }}
		<br>
		<br>
		<div class="bg-green-400">supported versions:</div><br>
		<pre>{{ supported_versions }}</pre>
		<br>
		<div class="bg-green-400">meta:</div><br>
		<pre>{{ meta }}</pre>
	</title-bar-container>
</template>

<script setup lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	let path = await invoke_decode_hex_string(appWindow.label);

	let supported_versions = ref(JSON.stringify(await invoke_get_project_supported_versions(path), null, "   "));
	let meta = ref(JSON.stringify(await invoke_get_project_meta(path), null, "   "));

	invoke_add_recent_project(path);
</script>
