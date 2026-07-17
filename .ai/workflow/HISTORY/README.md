# Workflow History

After a sprint reaches `completed`, `failed`, or `cancelled`, run
`pnpm workflow:archive`. It promotes a complete `<sprint-id>/` archive with a
manifest before resetting current files. Do not manually create a competing
archive for the same sprint ID.

Do not store secrets, `.env` files, API keys, private journal content, provider
payloads, runtime SQLite databases, or other personal data here.
