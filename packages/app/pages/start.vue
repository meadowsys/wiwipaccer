<template>
	<div>
		a start page / welcome page i guess
		<br>
		{{ soon }}
		<br><br>

		<!-- <u-input v-model="langs" placeholder="cheese"/>
		<u-button @click="write">write</u-button>
		<u-button @click="read">read</u-button> -->
		<div>{{ result }}</div>
	</div>
</template>

<script setup lang="ts">
	const i18n = use_i18n();
	const t = use_t();
	const soon = t("start.soon");

	const langs = ref("");
	const result = ref("");

	function write() {
		const l = langs.value.split(" ").filter(l => l.length !== 0);
		console.log(l);
		invoke_write_locale_setting(l)
			.then(() => result.value = `written: ${l.join(", ")}`);
	}

	function read() {
		invoke_read_locale_setting()
			.then(l => result.value = `read: ${l.join(", ")}`)
	}
</script>
