<template>
	<div>
		you have selected version <code>{{ selectedVersion }}</code> in dev mode
	</div>
</template>

<script setup lang="ts">
	defineProps<{
		selectedVersion: string;
	}>();
</script>
