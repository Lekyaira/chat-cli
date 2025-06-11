# Task List Management

Guidelines for managing task lists in markdown files to track progress on completing a PRD

## Task Implementation

- **Sub-tasks, not patent tasks:** When asked to complete the next task, you are meant to complete the next sub-task, not parent task. Parent tasks are not assigned and are completed by completing all sub-tasks only.
- **One sub-task at a time by default:** When operating interactively, do **NOT** start the next sub‑task until prompted.
- **Autonomous Mode:** When explicitly authorized or instructed to operate autonomously, proceed through all sub-tasks in order, one at a time, following the full task list until finished or a stopping condition is met (see AGENTS.md and `Stoppage and Escalation` section for stopping conditions).
- **Completion protocol:**
  1. When you finish a **sub‑task**, immediately mark it as completed by changing `[ ]` to `[x]`.
  2. If **all** subtasks underneath a parent task are now `[x]`, also mark the **parent task** as completed.
  3. Update the project README.md to reflect changes made to the project.

## Task List Maintenance

1. **Update the task list as you work:**

   - Mark tasks and subtasks as completed (`[x]`) per the protocol above.
   - Add new tasks as they emerge.

2. **Maintain the “Relevant Files” section:**
   - List every file created or modified.
   - Give each file a one‑line description of its purpose.

## AI Instructions

When working with task lists, the AI must:

1. Regularly update the task list file after finishing any significant work.
2. Follow the completion protocol:
   - Mark each finished **sub‑task** `[x]`.
   - Mark the **parent task** `[x]` once **all** its subtasks are `[x]`.
3. Add newly discovered tasks.
4. Keep “Relevant Files” accurate and up to date.
5. Before starting work, check which sub‑task is next.

## Stoppage and Escalation

The agent must **immediately STOP autonomous work and escalate to the user** if any of the following conditions are met during task execution:

### Stopping Conditions

- **Project Completion**

  - All sub-tasks and tasks in the current task list are marked `[x]` (completed).

- **Unclear Requirements**

  - The agent is unable to determine how to implement the next sub-task or is missing critical information.
  - The requirements are ambiguous, contradictory, or incomplete.

- **Scope Creep**

  - More than 3 new sub-tasks or tasks are added in the course of completing a single sub-task or task, or the overall project is expanding uncontrollably.

- **Tooling/Environment Failures**

  - Required tooling, dependencies, or services are not working or cannot be installed/configured.
  - The agent is unable to run tests or verify code changes due to technical issues.

- **User Input Needed**

  - A decision, clarification, or approval from the user is required before proceeding.
  - A significant change in project direction, architecture, or approach is required.

- **Other Issues**
  - Any error or obstacle occurs that prevents further progress with the current instructions.
  - The agent is unsure if continuing is correct or might cause unwanted side effects.

**When a stopping condition is encountered:**

1. Mark the current sub-task as blocked (optionally add `[blocked]` or similar notation).
2. Summarize the issue and the reason for escalation in the project notes or in your output.
3. Prompt the user for guidance on how to proceed.

---

**Note:**  
These stopping conditions are meant to ensure that autonomous work does not continue blindly if the project encounters unexpected issues, grows beyond the intended scope, or needs human input to proceed safely and correctly.
