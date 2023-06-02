<template>
	<div class="h-full flex-grow py-10 px-6" data-tauri-drag-region>
		<div class="overflow-scroll h-full flex flex-col">
			<div v-if="p.recents.length > 0">
				<div class="flex flex-row pb-4">
					<span class="font-permanent-marker text-2xl">Recents</span>
					<div class="flex-grow" data-tauri-drag-region />
					<button
						class="btn btn-outline btn-sm normal-case"
						@click="invoke_clear_recent_projects"
					>
						Clear recents
					</button>
				</div>
				<div :class="joiner" class="border border-base-300 rounded-lg">
					<div
						v-for="recent, i in p.recents"
						:key="recent.path"
						class="collapse bg-base-200 border-b-2 border-b-base-300 last:border-b-0 rounded-lg"
						:class="joiner_item"
					>
						<input
							type="checkbox"
							:checked="current_checked === i"
							@change="current_checked === i ? current_checked = -1 : current_checked = i"
						/>
						<div class="collapse-title font-bold select-none">{{ recent.name }}</div>
						<div class="collapse-content select-none overflow-scroll whitespace-nowrap">
							<div class="px-16 pt-4 border-t border-t-base-300" />
							Path: <code>{{ recent.path }}</code>
							<div class="h-2" />
							<button
								class="btn btn-outline btn-sm rounded-lg mr-4 normal-case btn-primary"
								@click="invoke_open_project(recent.path)"
							>
								Open
							</button>
							<button
								class="btn btn-outline btn-sm rounded-lg mr-4 normal-case"
								@click="current_checked = -1, invoke_remove_recent_project(recent.path)"
							>
								Remove from recents
							</button>
						</div>
					</div>
				</div>
			</div>

			<div v-else class="text-center select-none cursor-default" data-tauri-drag-region>
				No recent projects
			</div>
			<div class="flex-grow" data-tauri-drag-region />
		</div>
	</div>
</template>

<script setup lang="ts">
	const p = defineProps<{
		recents: Array<{ name: string, path: string }>;
	}>();

	const joiner = computed(() => p.recents.length > 1 ? ["join", "join-vertical"] : []);
	const joiner_item = computed(() => p.recents.length > 1 ? ["join-item"] : []);

	const current_checked = ref(-1);
</script>

<!--
<style scoped>
	.hyphens {
		/* todo figure this out lol
		   class is supposed to be on the <button> with the v-for */
		hyphens: auto;
	}
</style> -->
