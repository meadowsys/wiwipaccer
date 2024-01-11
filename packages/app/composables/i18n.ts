import { t_key } from "~/plugins/i18n";

export function use_t() {
	return inject(t_key)!;
}
