<template>
	<div>
		<code>
			{{ stuff }}
		</code>
	</div>
</template>

<script setup lang="ts">
	let name = (await invoke_get_workspace_name())!;
	let mc_version = "1.18.2";
	let stuff = JSON.stringify(await invoke_get_frontend_data_for({ name, mc_version }), null, "   ");
	// invoke_get_frontend_data_for({
	// 	name: "uwu",
	// })
</script>
