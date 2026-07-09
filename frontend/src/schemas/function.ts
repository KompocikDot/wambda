import * as v from 'valibot';

export enum Language {
  C = "c",
  Go = "go",
  PYTHON = "python",
  RUST = "rust"
}

export enum Trigger {
  HTTP = 'http',
  CRON = 'cron'
}

export const LanguageSchema = v.enum_(Language);
export const TriggerSchema = v.enum_(Trigger);

export const EnvVarSchema = v.object({
  key: v.pipe(v.string(), v.minLength(1, 'Key cannot be empty')),
  value: v.string(),
});

export const CreateFunctionSchema = v.object({
  name: v.pipe(v.string(), v.minLength(1, 'Function name is required')),
  language: LanguageSchema,
  trigger: TriggerSchema,
  memoryLimit: v.pipe(v.number(), v.minValue(1)),
  timeout: v.pipe(v.number(), v.minValue(1)),
  // Update: strictly validate it as an array of our EnvVarSchema
  envs: v.array(EnvVarSchema),
  wasm: v.instance(File, 'Please upload a valid WebAssembly file'),
});

export type CreateFunctionInput = v.InferInput<typeof CreateFunctionSchema>;
export type CreateFunctionOutput = v.InferOutput<typeof CreateFunctionSchema>;
export type EnvVarInput = v.InferInput<typeof EnvVarSchema>;
export type EnvVarOutput = v.InferOutput<typeof EnvVarSchema>;
