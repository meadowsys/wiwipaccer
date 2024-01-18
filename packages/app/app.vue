<template>
	<div class="h-full p-4">
		<div class="h-full overflow-scroll p-2">
			<nuxt-layout>
				<nuxt-page />
			</nuxt-layout>
		</div>
	</div>
</template>

<style>
	html { overscroll-behavior: none }
	html, body, #__nuxt {
		height: 100%;
	}
</style>

<script setup lang="ts">
	const i18n = use_i18n();
	const locales = await invoke_read_locale_setting();
	i18n._set_locales(locales);

	const unlisten_refresh_locales = await listen_refresh_locales(
		locales => i18n._set_locales(locales.payload)
	);

	onUnmounted(unlisten_refresh_locales);
</script>
