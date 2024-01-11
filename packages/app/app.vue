<template>
	<div>
		<nuxt-page />
	</div>
</template>

<style>
	html { overscroll-behavior: none }
</style>

<script setup lang="ts">
	let i18n = use_i18n();
	await i18n.set_locale("en");
</script>
