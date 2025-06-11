# Contributor Guide

## Tasks

- When asked to complete a task, or when asked to complete the next task, **ALWAYS** check `/tasks/process-task-list-codex.md` and follow that process.
- When asked to work fully autonomously, use the following process:
  1. Complete the next task, as if the user asked you to "Complete the next task", including running all tests and verifying recent changes.
  2. Determine if any issues arose during the completion of the task that requires project manager input. Determine if project scope is unreasonably increasing, if tooling is not functioning properly, or if you are unable to understand how to complete the sub-task.
     - If so, **STOP** and mark the task complete, then ask the user how to proceed.
     - Unreasonable project scope creep is more than ~3 tasks and/or sub-tasks added in one task completion cycle.
  3. If no issues were encountered and the project passes all tests, continue to step 1. and continue completing sub-tasks until all tasks are complete.
