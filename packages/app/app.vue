<template>
	<nuxt-layout>
		<nuxt-page />
	</nuxt-layout>
</template>

<style>
	html { overscroll-behavior: none }
	html, body, #__nuxt {
		height: 100%;
	}
</style>

<script setup lang="ts">
	let i18n = use_i18n();
	await i18n.set_locale("en");
</script>
