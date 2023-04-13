<template>
	<div class="h-full" data-tauri-drag-region>
		<nuxt-page />
	</div>
</template>

<style>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;

	html, body, #__nuxt {
		@apply h-full bg-transparent font-andika;
	}
</style>
