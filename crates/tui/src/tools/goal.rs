//! Goal tools — LLM-as-judge surface for the goal system.
//!
//! Three tools: create_goal, get_goal, update_goal.
//! The environment (engine) owns the goal state via Arc<Mutex<GoalState>>;
//! the model is the judge that decides when a goal is satisfied.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::{Value, json};

use codewhale_tools::{ToolCapability, ToolError, ToolResult};

use crate::tools::ToolContext;
use crate::tools::spec::{ApprovalRequirement, ToolSpec};
use crate::tui::app::GoalState;

/// Shared goal state — same Arc<Mutex<>> used by App.goal.
pub type SharedGoalState = Arc<Mutex<GoalState>>;

// ── CreateGoalTool ────────────────────────────────────────────────────

pub struct CreateGoalTool {
    goal_state: SharedGoalState,
}

impl CreateGoalTool {
    pub fn new(goal_state: SharedGoalState) -> Self {
        Self { goal_state }
    }
}

#[async_trait]
impl ToolSpec for CreateGoalTool {
    fn name(&self) -> &'static str {
        "create_goal"
    }

    fn description(&self) -> &'static str {
        "Create a new goal with an objective and optional token budget. The engine will track progress and prompt for continuation until the goal is marked complete via update_goal."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "objective": {
                    "type": "string",
                    "description": "The goal objective — what you are working toward."
                },
                "token_budget": {
                    "type": "integer",
                    "description": "Optional soft token budget."
                }
            },
            "required": ["objective"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WritesFiles]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Auto
    }

    async fn execute(&self, input: Value, _context: &ToolContext) -> Result<ToolResult, ToolError> {
        let objective = input["objective"].as_str().unwrap_or("");
        if objective.trim().is_empty() {
            return Ok(ToolResult::error("Goal objective cannot be empty."));
        }
        let token_budget = input["token_budget"].as_u64().map(|v| v as u32);
        let mut goal = self.goal_state.lock().unwrap();
        goal.goal_objective = Some(objective.to_string());
        goal.goal_token_budget = token_budget;
        goal.goal_started_at = Some(std::time::Instant::now());
        goal.goal_completed = false;
        let budget_str = token_budget
            .map(|b| format!(" (budget: {b} tokens)"))
            .unwrap_or_default();
        Ok(ToolResult::success(format!(
            "Goal created: \"{objective}\"{budget_str}. I will work toward this objective and audit completion before claiming it done."
        )))
    }
}

// ── GetGoalTool ───────────────────────────────────────────────────────

pub struct GetGoalTool {
    goal_state: SharedGoalState,
}

impl GetGoalTool {
    pub fn new(goal_state: SharedGoalState) -> Self {
        Self { goal_state }
    }
}

#[async_trait]
impl ToolSpec for GetGoalTool {
    fn name(&self) -> &'static str {
        "get_goal"
    }

    fn description(&self) -> &'static str {
        "Return the current goal state including objective, status, token budget, and elapsed time."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "required": []
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::ReadOnly]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Auto
    }

    async fn execute(
        &self,
        _input: Value,
        _context: &ToolContext,
    ) -> Result<ToolResult, ToolError> {
        let goal = self.goal_state.lock().unwrap();
        let status = if goal.goal_completed {
            "complete"
        } else if goal.goal_objective.is_some() {
            "active"
        } else {
            "none"
        };
        let elapsed = goal.goal_started_at.map(|t| {
            let d = t.elapsed();
            let secs = d.as_secs();
            if secs < 60 {
                format!("{secs}s")
            } else if secs < 3600 {
                format!("{}m{}s", secs / 60, secs % 60)
            } else {
                format!("{}h{}m", secs / 3600, (secs % 3600) / 60)
            }
        });
        let snapshot = json!({
            "objective": goal.goal_objective,
            "status": status,
            "token_budget": goal.goal_token_budget,
            "tokens_used": 0,
            "elapsed": elapsed,
        });
        Ok(ToolResult::json(&snapshot)
            .unwrap_or_else(|_| ToolResult::success(snapshot.to_string())))
    }
}

// ── UpdateGoalTool ────────────────────────────────────────────────────

pub struct UpdateGoalTool {
    goal_state: SharedGoalState,
}

impl UpdateGoalTool {
    pub fn new(goal_state: SharedGoalState) -> Self {
        Self { goal_state }
    }
}

#[async_trait]
impl ToolSpec for UpdateGoalTool {
    fn name(&self) -> &'static str {
        "update_goal"
    }

    fn description(&self) -> &'static str {
        "Update the goal — mark it complete with evidence, pause it, or change the objective. This is the LLM-as-judge entry point: only call complete when you have verified the objective is fully satisfied."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["active", "complete", "paused"],
                    "description": "New status: 'complete' when done, 'paused' when blocked, 'active' to resume or update."
                },
                "evidence": {
                    "type": "string",
                    "description": "When completing, briefly cite the evidence that proves the goal is done."
                },
                "objective": {
                    "type": "string",
                    "description": "New objective text (only when updating, keep empty when completing)."
                }
            },
            "required": ["status"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WritesFiles]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Auto
    }

    async fn execute(&self, input: Value, _context: &ToolContext) -> Result<ToolResult, ToolError> {
        let status = input["status"].as_str().unwrap_or("");
        let mut goal = self.goal_state.lock().unwrap();
        if goal.goal_objective.is_none() && status != "active" {
            return Ok(ToolResult::error(
                "No active goal to update. Use create_goal first.",
            ));
        }
        match status {
            "complete" => {
                goal.goal_completed = true;
                let evidence = input["evidence"].as_str().unwrap_or("");
                let note = if evidence.is_empty() {
                    String::new()
                } else {
                    format!(" Evidence: {evidence}")
                };
                Ok(ToolResult::success(format!("Goal marked complete.{note}")))
            }
            "paused" => {
                goal.goal_completed = false;
                let obj = goal.goal_objective.as_deref().unwrap_or("unknown");
                Ok(ToolResult::success(format!(
                    "Goal \"{obj}\" paused. It will not auto-continue until resumed."
                )))
            }
            "active" => {
                if let Some(new_obj) = input["objective"].as_str() {
                    goal.goal_objective = Some(new_obj.to_string());
                    goal.goal_completed = false;
                    Ok(ToolResult::success(format!("Goal updated: \"{new_obj}\"")))
                } else {
                    goal.goal_completed = false;
                    Ok(ToolResult::success(
                        "Goal resumed. Continuation will resume on the next idle turn.",
                    ))
                }
            }
            other => Ok(ToolResult::error(format!(
                "Unknown goal status: '{other}'. Use active, complete, or paused."
            ))),
        }
    }
}
