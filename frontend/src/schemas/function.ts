import * as v from 'valibot';

export const EnvVarSchema = v.object({
  key: v.pipe(v.string(), v.minLength(1, 'Key cannot be empty')),
  value: v.string(),
});

export const CreateUpdateFunctionSchema = v.object({
  name: v.pipe(v.string(), v.minLength(1, 'Function name is required')),
  memoryLimit: v.pipe(v.number(), v.minValue(1)),
  timeout: v.pipe(v.number(), v.minValue(1)),
  envs: v.array(EnvVarSchema),
  wasm: v.instance(File, 'Please upload a valid WebAssembly file'),
});


export const UpdateFunctionSchema = v.object({
  ...CreateUpdateFunctionSchema.entries,
  wasm: v.optional(v.instance(File, 'Please upload a valid WebAssembly file')),
});

export type CreateUpdateFunctionInput = v.InferInput<typeof CreateUpdateFunctionSchema>;
export type CreateUpdateFunctionOutput = v.InferOutput<typeof CreateUpdateFunctionSchema>;
export type UpdateFunctionOutput = v.InferOutput<typeof UpdateFunctionSchema>;
export type EnvVarInput = v.InferInput<typeof EnvVarSchema>;
export type EnvVarOutput = v.InferOutput<typeof EnvVarSchema>;
