<template>
	<div class="h-full pt-10 pb-6 px-14 text-center flex flex-col" data-tauri-drag-region>
		<div class="flex-grow" data-tauri-drag-region />

		<div class="font-permanent-marker select-none cursor-default text-5xl" data-tauri-drag-region>wiwipaccer</div>
		<div class="font-permanent-marker select-none cursor-default" data-tauri-drag-region>version {{ version }}</div>

		<div class="p-2" data-tauri-drag-region />

		<div class="select-none cursor-default" data-tauri-drag-region>Copyright &copy; 2023 Meadowsys</div>
		<div class="select-none cursor-default" data-tauri-drag-region>
			License: <nuxt-link class="link-primary link-hover" to="https://github.com/meadowsys/wiwipaccer/blob/wiwi/LICENSE" target="_blank">AGPL-3.0-only</nuxt-link>
		</div>

		<div class="p-2" data-tauri-drag-region />

		<div class="flex flex-row">
			<div class="flex-grow" data-tauri-drag-region />

			<nuxt-link class="btn btn-outline btn-square" to="https://wiwipaccer.kiwin.gay" target="_blank" title="wiwipaccer website">
				<span class="i-octicon-globe-16 text-2xl" />
			</nuxt-link>
			<div class="w-2" />
			<nuxt-link class="btn btn-outline btn-square" to="https://github.com/meadowsys/wiwipaccer" target="_blank" title="wiwipaccer github repository">
				<span class="i-octicon-mark-github-16 text-2xl" />
			</nuxt-link>

			<div class="flex-grow" data-tauri-drag-region />
		</div>

		<div class="flex-grow" data-tauri-drag-region />
	</div>
</template>

<script setup lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	let version = appWindow.label.replace(/^about-(.*)$/, "$1");
	version = await invoke_decode_hex_string(version);
</script>
