# GitHub Issue Integration

## Issue References

When user mentions issue number (e.g., "#42"):
1. Fetch details using `mcp_github_get_issue` (repo: `eheimer/movies`)
2. Check for task list in issue body/comments

## Spec Creation

Record issue number in requirements.md: `**GitHub Issue:** #<number>`

## Progress Tracking

On first task: Add comment with task checklist using `mcp_github_add_issue_comment`