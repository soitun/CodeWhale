//! Balance: query the active provider's account balance or credit status.
//!
//! Live balance dispatch is planned for a future release (#2019). Until then,
//! this command prints the provider's billing endpoint and instructions for
//! manual balance checking, plus the current session cost summary.

use crate::config::ApiProvider;
use crate::tui::app::App;

use super::CommandResult;

/// Query provider account balance / credits.
pub fn balance(app: &mut App) -> CommandResult {
    let provider = app.api_provider;
    let session_cost = app.displayed_session_cost_for_currency(app.cost_currency);
    let cost_label = app.format_cost_amount(session_cost);
    let token_usage = app.session.total_conversation_tokens;

    let provider_info = match provider {
        ApiProvider::Deepseek | ApiProvider::DeepseekCN => {
            format!(
                "DeepSeek billing: https://platform.deepseek.com/usage\n\
                 API balance endpoint: GET https://api.deepseek.com/user/balance\n\
                 (Requires Bearer token auth with your API key)\n\n\
                 Live balance dispatch is planned for a future release (#2019).\n\
                 For now, check your balance on the DeepSeek Platform dashboard."
            )
        }
        ApiProvider::Openrouter => {
            format!(
                "OpenRouter credits: https://openrouter.ai/credits\n\
                 Live balance dispatch is planned for a future release (#2019)."
            )
        }
        ApiProvider::Novita => {
            format!(
                "Novita billing: check your provider dashboard.\n\
                 Live balance dispatch is planned for a future release (#2019)."
            )
        }
        ApiProvider::Fireworks => {
            format!(
                "Fireworks billing: https://fireworks.ai/account/billing\n\
                 Live balance dispatch is planned for a future release (#2019)."
            )
        }
        _ => {
            format!(
                "Balance check is not supported for {} yet.\n\
                 Check the provider dashboard for account balance details.",
                provider.display_name()
            )
        }
    };

    CommandResult::message(format!(
        "{provider_info}\n\n\
         This session: {cost_label}  |  {token_usage} tokens"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::tui::app::{App, TuiOptions};
    use std::path::PathBuf;

    fn create_test_app() -> App {
        let options = TuiOptions {
            model: "deepseek-v4-flash".to_string(),
            workspace: PathBuf::from("."),
            config_path: None,
            config_profile: None,
            allow_shell: false,
            use_alt_screen: true,
            use_mouse_capture: false,
            use_bracketed_paste: true,
            max_subagents: 1,
            skills_dir: PathBuf::from("."),
            memory_path: PathBuf::from("memory.md"),
            notes_path: PathBuf::from("notes.txt"),
            mcp_config_path: PathBuf::from("mcp.json"),
            use_memory: false,
            start_in_agent_mode: true,
            skip_onboarding: true,
            yolo: false,
            resume_session_id: None,
            initial_input: None,
        };
        App::new(options, &Config::default())
    }

    #[test]
    fn test_balance_shows_deepseek_info() {
        let mut app = create_test_app();
        app.api_provider = ApiProvider::Deepseek;
        let result = balance(&mut app);
        let msg = result.message.unwrap();
        assert!(msg.contains("platform.deepseek.com"));
        assert!(msg.contains("/user/balance"));
    }

    #[test]
    fn test_balance_shows_session_cost() {
        let mut app = create_test_app();
        let result = balance(&mut app);
        let msg = result.message.unwrap();
        assert!(msg.contains("tokens"));
    }
}
