<template>
	<div>
		path/label: {{ path }}
	</div>
</template>

<script setup lang="ts">
	import { window } from "@tauri-apps/api";
	let path = await invoke_decode_hex_string(window.appWindow.label);
</script>
