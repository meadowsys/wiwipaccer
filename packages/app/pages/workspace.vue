<template>
	<div>
		<u-button @click="add_pack">add pack uwu</u-button>
		<br>
		<pre>{{ stuff }}</pre>
	</div>
</template>

<script setup lang="ts">
	let name = (await invoke_get_workspace_name())!;
	let mc_version = "1.18.2";
	let stuff = ref(JSON.stringify(await invoke_get_frontend_data_for({ name, mc_version }), null, "   "));

	async function add_pack() {
		await invoke_prompt_add_pack({ name });

		let frontend_data = await invoke_get_frontend_data_for({ name, mc_version });
		stuff.value = JSON.stringify(frontend_data, null, "   ");
	}
</script>
