# Spec Management

## Spec Cleanup Policy

When creating a new spec, check if other spec directories exist in `.kiro/specs/`. If they do:

1. **Ask the user** if they want to remove the old spec(s) before creating the new one
2. List the existing spec directories so the user can see what would be removed
3. Explain that this keeps only one active spec at a time, while preserving old specs in git history

Example prompt:

```
I see there's an existing spec: .kiro/specs/dirty-record-handling/
Would you like me to remove it before creating the new spec? This keeps your workspace clean while preserving the old spec in git history.
```

## Rationale

For single-developer workflows, maintaining one active spec at a time:

- Keeps the working directory clean
- Preserves historical context when checking out old commits
- Avoids accumulation of outdated specs
- Each commit contains the spec relevant to that change

## Exception

If the user is explicitly working on multiple features simultaneously, they may decline the cleanup. Respect their choice and proceed with creating the new spec.
