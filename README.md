# Wambda: WASM-based FaaS runner

## Tools

- seaorm cli
- cargo
- npm/npx

## TODOs

- Add function to measure password strength ? [dropbox zxcvbn](https://github.com/shssoichiro/zxcvbn-rs)
- Add env vars encyption
- Add api keys table
- Add stats endpoints and queries
- Think of a way to pass requests and call http handlers
- Add sdks

## Notes to add in thesis (or future me)

- versions of wasmtime dependency have to match exactly or there are weird compile/type errors

## DB Schema

```mermaid
erDiagram
    users ||--o{ functions : "owns"
    functions ||--o{ runs : "triggers"

    users {
        SERIAL id PK
        TEXT full_name
        VARCHAR email
        VARCHAR password
    }

    functions {
        BIGINT id PK
        VARCHAR name
        UUID uuid UK
        VARCHAR language "enum: c, go, javascript, python, rust"
        VARCHAR trigger "enum: http, cron"
        BIGINT memory_limit "Set as Mb"
        BIGINT timeout "Set as milliseconds"
        TEXT envs "Saved as key=value"
        SERIAL owner_id FK
    }

    runs {
        SERIAL id PK
        TIMESTAMP triggered_at
        TIMESTAMP finish_time
        UUID uuid UK
        BIGINT duration "saved as miliseconds"
        VARCHAR status "enum: success, error"
        SERIAL fn_id FK
    }
```
