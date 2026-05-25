//! Regression coverage for DeepSeek V4 reasoning-content replay after tools.

use futures_util::StreamExt;

#[path = "../src/models.rs"]
#[allow(dead_code)]
mod models;

#[path = "support/llm_client.rs"]
mod llm_client;

use crate::llm_client::LlmClient;
use crate::llm_client::mock::{FauxStep, MockLlmClient, canned};
use crate::models::{ContentBlock, Message, MessageRequest};

fn user_message(text: &str) -> Message {
    Message {
        role: "user".to_string(),
        content: vec![ContentBlock::Text {
            text: text.to_string(),
            cache_control: None,
        }],
    }
}

fn assistant_tool_call_with_reasoning(
    reasoning_content: &str,
    id: &str,
    name: &str,
    input: serde_json::Value,
) -> Message {
    Message {
        role: "assistant".to_string(),
        content: vec![
            ContentBlock::Thinking {
                thinking: reasoning_content.to_string(),
            },
            ContentBlock::ToolUse {
                id: id.to_string(),
                name: name.to_string(),
                input,
                caller: None,
            },
        ],
    }
}

fn tool_result_message(tool_use_id: &str, content: &str) -> Message {
    Message {
        role: "user".to_string(),
        content: vec![ContentBlock::ToolResult {
            tool_use_id: tool_use_id.to_string(),
            content: content.to_string(),
            is_error: None,
            content_blocks: None,
        }],
    }
}

fn make_request(messages: Vec<Message>) -> MessageRequest {
    MessageRequest {
        model: "deepseek-v4-pro".to_string(),
        messages,
        max_tokens: 4096,
        system: None,
        tools: None,
        tool_choice: None,
        metadata: None,
        thinking: None,
        reasoning_effort: Some("high".to_string()),
        stream: Some(true),
        temperature: None,
        top_p: None,
    }
}

#[tokio::test]
async fn factory_asserts_reasoning_content_is_replayed_after_tool_call() {
    let reasoning_content = "I need the directory listing before I answer.";
    let tool_call_turn = vec![
        canned::message_start("r1"),
        canned::thinking_delta(0, reasoning_content),
        canned::tool_use_block_start(1, "call_list", "list_dir"),
        canned::tool_input_delta(1, r#"{"path":"/tmp"}"#),
        canned::block_stop(1),
        canned::message_delta("tool_use", None),
        canned::message_stop(),
    ];

    let mock = MockLlmClient::from_steps(vec![
        FauxStep::Canned(tool_call_turn),
        FauxStep::Factory(Box::new(move |request| {
            assert!(
                matches!(
                    request.messages.last().and_then(|message| message.content.first()),
                    Some(ContentBlock::ToolResult { tool_use_id, .. }) if tool_use_id == "call_list"
                ),
                "follow-up request should append the tool result after the assistant tool call"
            );

            let replayed = request
                .messages
                .iter()
                .rev()
                .find(|message| message.role == "assistant")
                .and_then(|message| {
                    message.content.iter().find_map(|block| match block {
                        ContentBlock::Thinking { thinking } => Some(thinking.as_str()),
                        _ => None,
                    })
                });

            assert_eq!(
                replayed,
                Some(reasoning_content),
                "DeepSeek V4 follow-up requests must replay reasoning_content from the assistant tool-call turn"
            );

            canned::simple_text_turn("done")
        })),
    ]);

    let mut first_stream = mock
        .create_message_stream(make_request(vec![user_message("list /tmp")]))
        .await
        .expect("first stream opens");
    while let Some(event) = first_stream.next().await {
        if matches!(
            event.expect("first event"),
            crate::models::StreamEvent::MessageStop
        ) {
            break;
        }
    }

    let follow_up = make_request(vec![
        user_message("list /tmp"),
        assistant_tool_call_with_reasoning(
            reasoning_content,
            "call_list",
            "list_dir",
            serde_json::json!({ "path": "/tmp" }),
        ),
        tool_result_message("call_list", "/tmp/file1\n/tmp/file2"),
    ]);

    let mut second_stream = mock
        .create_message_stream(follow_up)
        .await
        .expect("factory-backed stream opens");
    while let Some(event) = second_stream.next().await {
        if matches!(
            event.expect("second event"),
            crate::models::StreamEvent::MessageStop
        ) {
            break;
        }
    }

    assert_eq!(mock.call_count(), 2);
    assert_eq!(mock.remaining_turns(), 0);
}
