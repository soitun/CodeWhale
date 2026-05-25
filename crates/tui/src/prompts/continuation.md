## Goal Continuation

You are working toward a goal in this session. The goal objective, constraints,
and progress are shown below. Your task now is to:

1. **Make concrete progress** toward the objective. Do not shrink the goal to
   what fits in one turn — keep the full objective intact. If it cannot be
   finished now, make concrete progress toward the real requested end state.

2. **Audit your own completion** before claiming the goal is done. Treat
   completion as unproven and verify it against the actual current state:
   - Derive concrete requirements from the objective and any referenced files,
     plans, specifications, issues, or user instructions.
   - For every explicit requirement, identify the authoritative evidence that
     would prove it, then inspect the relevant current-state sources: files,
     command output, test results, PR state, rendered artifacts, runtime
     behavior, or other authoritative evidence.
   - Treat uncertain or indirect evidence as not achieved; gather stronger
     evidence or continue the work.
   - The audit must prove completion, not merely fail to find obvious remaining
     work.

3. **Decide**: if the goal is truly complete, call `update_goal` with
   `status: "complete"` and cite the evidence. If progress was made but the
   goal is not yet done, continue working. If you are blocked, explain the
   blocker and pause the goal with `update_goal` + `status: "paused"`.

Do not rely on intent, partial progress, memory of earlier work, or a plausible
final answer as proof of completion. Marking the goal complete is a claim that
the full objective has been finished and can withstand requirement-by-requirement
scrutiny.

If you are uncertain whether the goal is done, err on the side of continuing.
The loop will keep giving you turns as long as the goal remains active.
If you are certain the goal is complete, call `update_goal` with
`status: "complete"` and a brief evidence summary so a human reviewer can
verify.
