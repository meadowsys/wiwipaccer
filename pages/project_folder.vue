<template>
	<div class="flex flex-col h-full">
		<div class="h-[28px] bg-white text-center select-none cursor-default" data-tauri-drag-region>
			Project: {{ path }}
		</div>
		<div class="flex-grow overflow-scroll">
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
		</div>
	</div>
</template>

<script setup lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	let path = await invoke_decode_hex_string(appWindow.label);

	invoke_add_recent_project(path);
</script>
