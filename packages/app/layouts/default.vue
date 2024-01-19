<template>
	<div v-if="false" class="h-full p-4 bg-white dark:bg-black rounded-lg border border-gray-200 dark:border-gray-800 select-none cursor-default">
		<!-- todo: switch `flex-row` below to `flex-row-col` when on windows/linux -->
		<div class="h-8 select-none flex-row flex" data-tauri-drag-region>
			<!-- <u-icon
				name="i-ph-x-circle-fill"
				@click="close_window"
				class="
					h-6 w-4 mx-1
					bg-gray-300 hover:bg-red-400 active:bg-red-600
					dark:bg-gray-800 dark:hover:bg-red-400
				"
			/>
			<u-icon
				name="i-ph-minus-circle-fill"
				@click="minimise_window"
				class="
					h-6 w-4 mx-1
					bg-gray-300 hover:bg-yellow-400 active:bg-yellow-600
					dark:bg-gray-800 dark:hover:bg-yellow-400
				"
			/> -->
			<!-- it don't wanna work h -->
			<!-- <u-icon name="i-ph-plus-circle-fill" @click="maximise_window" class="h-6 w-4 mr-2 hover:bg-green-400" /> -->
		</div>
		<div>
			<slot />
		</div>
	</div>
	<template v-else>
		<slot />
	</template>
</template>

<script setup lang="ts">
	import { getCurrent } from "@tauri-apps/api/window";

	const current = getCurrent();

	const maximised = ref(false);

	const close_window = () => current.close();
	const minimise_window = () => current.minimize();
	const maximise_window = () => {
		if (maximised.value) {
			current.maximize().then(() => maximised.value = true);
		} else {
			current.unmaximize().then(() => maximised.value = false);
		}
	}
</script>
