import { z, type ZodType } from "zod";

export function ts_result<T extends ZodType>(value: T) {
	return z.union([
		z.object({
			success: z.literal(true),
			value
		}).strict(),
		z.object({
			success: z.literal(false),
			error: z.string()
		}).strict()
	]);
}
