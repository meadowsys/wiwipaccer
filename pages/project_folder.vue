<template>
	<title-bar-container :title="`Project: ${path}`">
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
		path: {{ path }}<br>
	</title-bar-container>
</template>

<script setup lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	let path = await invoke_decode_hex_string(appWindow.label);

	invoke_add_recent_project(path);
</script>
