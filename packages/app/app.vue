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
	use_i18n().set_locales(["en"]);
</script>
