<template>
	<div class="h-full" :style="theme">
		<nuxt-page />
	</div>
</template>

<script setup lang="ts">
	import { listen } from "@tauri-apps/api/event";

	let theme = use_theme();
	// have theme handling be on rust side, theme preference saved there
	// then emit window events to retheme all the windows at once

	listen("theme_update", theme => {
		console.log(theme);
	});
</script>
<style>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;

	html, body, #__nuxt {
		@apply h-full bg-transparent font-andika;
	}
	@layer base {
		* {
			@apply selection:bg-[#dcc4fe]
		}
	}
</style>
