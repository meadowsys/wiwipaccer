<template>
	<button
		@click="!disabled && emit('click')"
		:disabled="disabled"
		class="btn block btn-sm normal-case btn-outline mt-1 text-left"
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
