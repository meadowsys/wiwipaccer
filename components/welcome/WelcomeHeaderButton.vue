<template>
	<button
		@click="!disabled && emit('click')"
		:disabled="disabled"
		class="
			px-2 py-1 text-left rounded cursor-pointer border-2 border-black border-opacity-0 bg-black bg-opacity-0
			hover:border-opacity-10 hover:bg-opacity-10
			active:bg-opacity-20 active:shadow-inner

			disabled:text-neutral-400 disabled:cursor-default
			disabled:hover:bg-opacity-0 disabled:hover:border-opacity-0
			disabled:active:shadow-none
		"
	>
		<slot />
	</button>
</template>

<script setup lang="ts">
	defineProps<{
		disabled?: boolean;
	}>();
	let emit = defineEmits<{
		(e: "click"): void
	}>();
</script>
