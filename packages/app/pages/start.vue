<template>
	<div>
		a start page / welcome page i guess
		<br>
		{{ soon }}
		<br>
		<u-button @click="invoke_open_workspace_dialog">open workspace</u-button>
		<br><br>
		<u-button @click="dutch">dutch</u-button>
		<u-button @click="english">english</u-button>
	</div>
</template>

<script setup lang="ts">
	const i18n = use_i18n();
	const t = use_t();
	const soon = t("start.soon");

	function dutch() {
		i18n.set_locales(["nl"]);
	}
	function english() {
		i18n.set_locales(["en"]);
	}
</script>
