# GitHub Issue Integration

## Issue Reference Detection

When the user references an issue number (e.g., "#42", "issue #42", "issue 42"), you should:

1. **Fetch the issue details** using the GitHub MCP server
   - Repository: `eheimer/openworld_api`
   - Use `mcp_github_get_issue` with the issue number

2. **Check for tasks.md file** in the issue body or comments
   - If the issue contains a tasks.md or task list, use it to track progress

## Spec Creation from Issues

When creating a spec from a GitHub issue:

1. **Record the issue number** in the requirements.md file
   - Add a metadata section at the top of requirements.md
   - Format: `**GitHub Issue:** #<issue_number>`
   - This allows tracking back to the original issue when executing tasks

Example requirements.md header:
```markdown
# Requirements Document

**GitHub Issue:** #42

## Introduction
...
```

## Issue Progress Tracking

When working on an issue with a task list, follow this workflow:

### 1. First Task Started

When you begin working on the first task:
- NOTE: changing status is not supported with our MCP settings at this time, so ignore this line: **Update the issue status** to "In Progress" using `mcp_github_update_issue`
- Add a comment to the issue using `mcp_github_add_issue_comment`
- Comment should include:
  - Acknowledgment that work has started
  - Complete contents of the tasks.md or task list
  - Format as a GitHub markdown checklist

Example comment:
```
Starting work on this issue:

- [ ] Task 1 description
- [ ] Task 2 description
- [ ] Task 3 description
```