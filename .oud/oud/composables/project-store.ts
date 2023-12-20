import { defineStore } from "pinia";

// export const use_window_state_store = defineStore("window-state", () => {
// 	return {};
// });

export const use_selected_opts_store = defineStore("selected", () => {
	let selected = ref<Record<string, string>>({});

	function selection_of(opt: string) {
		return computed(() => selected.value[opt]);
	}

	return {};
});
