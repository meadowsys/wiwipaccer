<template>
	<div class="h-full pt-10 pb-6 px-14 bg-white flex flex-col text-center" data-tauri-drag-region>
		<div class="flex-grow" data-tauri-drag-region />

		<h1 class="text-5xl font-permanent-marker select-none cursor-default" data-tauri-drag-region>wiwipaccer</h1>
		<div class="font-permanent-marker select-none cursor-default" data-tauri-drag-region>version {{ version }}</div>
		<div class="p-2" data-tauri-drag-region />
		<div class="select-none cursor-default" data-tauri-drag-region>Copyright © 2023 Meadowsys</div>
		<div class="select-none cursor-default" data-tauri-drag-region>
			License: <nuxt-link to="https://github.com/Meadowsys/wiwipaccer/blob/wiwi/LICENSE" class="text-purple-700 cursor-pointer" target="_blank">AGPL-3.0-only</nuxt-link>
		</div>
		<div class="flex flex-row">
			<div class="flex-grow" data-tauri-drag-region />
			<nuxt-link to="https://github.com/meadowsys/wiwipaccer" target="_blank">
				<github-mark-black viewBox="-49 -48 196 192" data-tauri-drag-region />
			</nuxt-link>
			<div class="flex-grow" data-tauri-drag-region />
		</div>

		<div class="flex-grow" data-tauri-drag-region />
	</div>
</template>

<script setup lang="ts">
	import { app } from "@tauri-apps/api";
	let version = await app.getVersion();
</script>
