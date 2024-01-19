<template>
	<div>
		a start page / welcome page i guess
		<br>
		{{ soon }}
		<br><br>
		List of workspaces <u-button @click="refresh_workspaces" icon="i-ph-arrow-clockwise"></u-button>
		<br>
		<ul>
			<li v-for="w in workspaces">{{ w }}</li>
		</ul>

		<input v-model="new_workspace_name" type="text" class="border rounded-md py-1 px-2">

		<!-- <u-input v-model="langs" placeholder="cheese"/>
		<u-button @click="write">write</u-button>
		<u-button @click="read">read</u-button> -->
		<div>{{ result }}</div>
		<span v-if="available">available aha</span>
		<span v-else>not avaliable</span>
		<u-button @click="create_and_open">do the thing</u-button>
	</div>
</template>

<script setup lang="ts">
	const i18n = use_i18n();
	const t = use_t();
	const soon = t("start.soon");

	const langs = ref("");
	const result = ref("");

	const workspaces = ref<Array<string>>([]);
	const new_workspace_name = ref("");
	const available = ref(false);
	watchDebounced(new_workspace_name, val => {
		invoke_check_workspace_name_is_available(val)
			.then(a => available.value = a);
	}, { debounce: 500, maxWait: 10_000 });


	function refresh_workspaces() {
		invoke_list_existing_workspaces()
			.then(l => workspaces.value = l);
	}

	async function create_and_open() {
		let name = new_workspace_name.value;
		console.log("eeh?");

		await invoke_create_new_workspace(name);
		console.log("eh?");
		await invoke_open_workspace(name);
		console.log("ehhhh?????");
	}
</script>
