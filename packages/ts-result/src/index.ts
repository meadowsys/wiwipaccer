import { z, type ZodType } from "zod";

export type TSResultParam<T, E> = {
	ok: T;
	err: E;
}

export function ts_result<T extends ZodType, E extends ZodType>(
	{ ok, err }: TSResultParam<T, E>
) {
	return z.union([
		z.object({
			success: z.literal(true),
			value: ok
		}).strict(),
		z.object({
			success: z.literal(false),
			error: err
		}).strict()
	]);
}
