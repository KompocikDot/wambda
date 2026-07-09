export type EnvVariable = {
  key: string
  value: string
}

export type FunctionForm = {
  name: string
  language: string
  trigger: string
  memoryLimitMb: number
  timeoutMs: number
  envVariables: EnvVariable[]
  binary?: File
}
