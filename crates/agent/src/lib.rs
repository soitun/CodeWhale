use std::collections::HashMap;

use codewhale_config::ProviderKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelFamily {
    DeepSeek,
    Anthropic,
    OpenAI,
    Google,
    Meta,
    Mistral,
    Qwen,
    Grok,
    Cohere,
    GptOss,
    Inferencer,
    Unknown,
}

impl ModelFamily {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeepSeek => "deepseek",
            Self::Anthropic => "anthropic",
            Self::OpenAI => "openai",
            Self::Google => "google",
            Self::Meta => "meta",
            Self::Mistral => "mistral",
            Self::Qwen => "qwen",
            Self::Grok => "grok",
            Self::Cohere => "cohere",
            Self::GptOss => "gpt-oss",
            Self::Inferencer => "inferencer",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelFamilyPalette {
    pub accent: (u8, u8, u8),
    pub accent_dim: (u8, u8, u8),
    pub thinking: (u8, u8, u8),
    pub tool_call: (u8, u8, u8),
}

impl ModelFamilyPalette {
    #[must_use]
    pub const fn for_family(family: ModelFamily) -> Self {
        match family {
            ModelFamily::DeepSeek => Self {
                accent: (72, 140, 220),
                accent_dim: (36, 76, 132),
                thinking: (52, 104, 148),
                tool_call: (80, 170, 198),
            },
            ModelFamily::Anthropic => Self {
                accent: (198, 116, 76),
                accent_dim: (112, 66, 48),
                thinking: (146, 102, 68),
                tool_call: (214, 154, 112),
            },
            ModelFamily::OpenAI => Self {
                accent: (82, 176, 150),
                accent_dim: (44, 98, 88),
                thinking: (70, 128, 112),
                tool_call: (118, 206, 176),
            },
            ModelFamily::Google => Self {
                accent: (86, 154, 228),
                accent_dim: (62, 92, 142),
                thinking: (224, 160, 72),
                tool_call: (92, 180, 116),
            },
            ModelFamily::Meta => Self {
                accent: (74, 132, 214),
                accent_dim: (42, 70, 128),
                thinking: (94, 148, 200),
                tool_call: (104, 186, 222),
            },
            ModelFamily::Mistral => Self {
                accent: (214, 122, 54),
                accent_dim: (118, 70, 42),
                thinking: (174, 104, 58),
                tool_call: (232, 158, 86),
            },
            ModelFamily::Qwen => Self {
                accent: (152, 112, 224),
                accent_dim: (78, 60, 134),
                thinking: (126, 94, 174),
                tool_call: (188, 146, 238),
            },
            ModelFamily::Grok => Self {
                accent: (184, 190, 198),
                accent_dim: (92, 98, 106),
                thinking: (132, 138, 148),
                tool_call: (216, 220, 224),
            },
            ModelFamily::Cohere => Self {
                accent: (230, 112, 136),
                accent_dim: (126, 58, 76),
                thinking: (176, 86, 110),
                tool_call: (238, 150, 166),
            },
            ModelFamily::GptOss => Self {
                accent: (92, 196, 176),
                accent_dim: (48, 110, 100),
                thinking: (76, 150, 138),
                tool_call: (128, 224, 204),
            },
            ModelFamily::Inferencer => Self {
                accent: (144, 164, 188),
                accent_dim: (74, 88, 108),
                thinking: (112, 130, 154),
                tool_call: (164, 188, 214),
            },
            ModelFamily::Unknown => Self {
                accent: (150, 160, 174),
                accent_dim: (72, 82, 96),
                thinking: (112, 122, 136),
                tool_call: (176, 188, 202),
            },
        }
    }
}

#[must_use]
pub fn model_family(model_id: &str) -> ModelFamily {
    let normalized = model_id.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return ModelFamily::Unknown;
    }

    let compact = normalized.replace(['_', '.', ':', '/', '\\'], "-");
    let family_markers = [
        (ModelFamily::GptOss, &["gpt-oss", "gptoss"][..]),
        (ModelFamily::DeepSeek, &["deepseek", "deep-seek"]),
        (ModelFamily::Anthropic, &["claude", "anthropic"]),
        (ModelFamily::Google, &["gemini", "gemma", "google"]),
        (
            ModelFamily::Meta,
            &["llama", "codellama", "meta-llama", "meta"],
        ),
        (ModelFamily::Mistral, &["mistral", "mixtral", "codestral"]),
        (ModelFamily::Qwen, &["qwen", "qwq", "qvq"]),
        (ModelFamily::Grok, &["grok", "x-ai", "xai"]),
        (ModelFamily::Cohere, &["cohere", "command-r", "command-a"]),
        (
            ModelFamily::OpenAI,
            &["gpt-5", "gpt-4", "gpt-3", "gpt4", "gpt3", "o1", "o3", "o4"][..],
        ),
    ];

    for (family, markers) in family_markers {
        if markers
            .iter()
            .any(|marker| normalized.contains(marker) || compact.contains(marker))
        {
            return family;
        }
    }

    let inferencer_markers = [
        "openrouter",
        "groq",
        "together",
        "cerebras",
        "fireworks",
        "deepinfra",
        "novita",
        "replicate",
        "nvidia-nim",
        "sglang",
        "vllm",
        "ollama",
    ];
    if inferencer_markers
        .iter()
        .any(|marker| normalized.contains(marker) || compact.contains(marker))
    {
        return ModelFamily::Inferencer;
    }

    ModelFamily::Unknown
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub provider: ProviderKind,
    pub aliases: Vec<String>,
    pub supports_tools: bool,
    pub supports_reasoning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResolution {
    pub requested: Option<String>,
    pub resolved: ModelInfo,
    pub used_fallback: bool,
    pub fallback_chain: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ModelRegistry {
    models: Vec<ModelInfo>,
    alias_map: HashMap<String, usize>,
}

impl Default for ModelRegistry {
    fn default() -> Self {
        let models = vec![
            ModelInfo {
                id: "deepseek-v4-pro".to_string(),
                provider: ProviderKind::Deepseek,
                aliases: vec![],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek-v4-flash".to_string(),
                provider: ProviderKind::Deepseek,
                aliases: vec![
                    "deepseek-chat".to_string(),
                    "deepseek-reasoner".to_string(),
                    "deepseek-r1".to_string(),
                    "deepseek-v3".to_string(),
                    "deepseek-v3.2".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek-ai/deepseek-v4-pro".to_string(),
                provider: ProviderKind::NvidiaNim,
                aliases: vec![
                    "deepseek-v4-pro".to_string(),
                    "nvidia-deepseek-v4-pro".to_string(),
                    "nim-deepseek-v4-pro".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek-ai/deepseek-v4-flash".to_string(),
                provider: ProviderKind::NvidiaNim,
                aliases: vec![
                    "deepseek-v4-flash".to_string(),
                    "deepseek-chat".to_string(),
                    "deepseek-reasoner".to_string(),
                    "nvidia-deepseek-v4-flash".to_string(),
                    "nim-deepseek-v4-flash".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "gpt-4.1".to_string(),
                provider: ProviderKind::Openai,
                aliases: vec!["gpt4.1".to_string(), "gpt-4o".to_string()],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "gpt-4.1-mini".to_string(),
                provider: ProviderKind::Openai,
                aliases: vec!["gpt-4o-mini".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            ModelInfo {
                id: "deepseek-reasoner".to_string(),
                provider: ProviderKind::WanjieArk,
                aliases: vec![
                    "wanjie-deepseek-reasoner".to_string(),
                    "ark-wanjie-deepseek-reasoner".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek/deepseek-v4-pro".to_string(),
                provider: ProviderKind::Openrouter,
                aliases: vec![
                    "deepseek-v4-pro".to_string(),
                    "openrouter-deepseek-v4-pro".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek/deepseek-v4-flash".to_string(),
                provider: ProviderKind::Openrouter,
                aliases: vec![
                    "deepseek-v4-flash".to_string(),
                    "deepseek-chat".to_string(),
                    "deepseek-reasoner".to_string(),
                    "openrouter-deepseek-v4-flash".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek/deepseek-v4-pro".to_string(),
                provider: ProviderKind::Novita,
                aliases: vec![
                    "deepseek-v4-pro".to_string(),
                    "novita-deepseek-v4-pro".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek/deepseek-v4-flash".to_string(),
                provider: ProviderKind::Novita,
                aliases: vec![
                    "deepseek-v4-flash".to_string(),
                    "deepseek-chat".to_string(),
                    "deepseek-reasoner".to_string(),
                    "novita-deepseek-v4-flash".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "accounts/fireworks/models/deepseek-v4-pro".to_string(),
                provider: ProviderKind::Fireworks,
                aliases: vec![
                    "deepseek-v4-pro".to_string(),
                    "fireworks-deepseek-v4-pro".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek-ai/DeepSeek-V4-Pro".to_string(),
                provider: ProviderKind::Sglang,
                aliases: vec![
                    "deepseek-v4-pro".to_string(),
                    "sglang-deepseek-v4-pro".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek-ai/DeepSeek-V4-Flash".to_string(),
                provider: ProviderKind::Sglang,
                aliases: vec![
                    "deepseek-v4-flash".to_string(),
                    "deepseek-chat".to_string(),
                    "deepseek-reasoner".to_string(),
                    "sglang-deepseek-v4-flash".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek-ai/DeepSeek-V4-Pro".to_string(),
                provider: ProviderKind::Vllm,
                aliases: vec![
                    "deepseek-v4-pro".to_string(),
                    "vllm-deepseek-v4-pro".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek-ai/DeepSeek-V4-Flash".to_string(),
                provider: ProviderKind::Vllm,
                aliases: vec![
                    "deepseek-v4-flash".to_string(),
                    "deepseek-chat".to_string(),
                    "deepseek-reasoner".to_string(),
                    "vllm-deepseek-v4-flash".to_string(),
                ],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "deepseek-coder:1.3b".to_string(),
                provider: ProviderKind::Ollama,
                aliases: vec![],
                supports_tools: true,
                supports_reasoning: false,
            },
        ];
        Self::new(models)
    }
}

impl ModelRegistry {
    #[must_use]
    pub fn new(models: Vec<ModelInfo>) -> Self {
        let mut alias_map = HashMap::new();
        for (idx, model) in models.iter().enumerate() {
            alias_map.entry(normalize(&model.id)).or_insert(idx);
            for alias in &model.aliases {
                alias_map.entry(normalize(alias)).or_insert(idx);
            }
        }
        Self { models, alias_map }
    }

    #[must_use]
    pub fn list(&self) -> Vec<ModelInfo> {
        self.models.clone()
    }

    #[must_use]
    pub fn resolve(
        &self,
        requested: Option<&str>,
        provider_hint: Option<ProviderKind>,
    ) -> ModelResolution {
        let mut fallback_chain = Vec::new();

        if let Some(name) = requested {
            fallback_chain.push(format!("requested:{name}"));
            if provider_hint == Some(ProviderKind::Ollama) {
                return ModelResolution {
                    requested: Some(name.to_string()),
                    resolved: ModelInfo {
                        id: name.trim().to_string(),
                        provider: ProviderKind::Ollama,
                        aliases: Vec::new(),
                        supports_tools: true,
                        supports_reasoning: false,
                    },
                    used_fallback: false,
                    fallback_chain,
                };
            }
            if let Some(provider) = provider_hint
                && let Some(model) = self
                    .models
                    .iter()
                    .find(|m| m.provider == provider && model_matches(m, name))
                    .cloned()
            {
                return ModelResolution {
                    requested: Some(name.to_string()),
                    resolved: preserve_requested_model_id_case(model, name),
                    used_fallback: false,
                    fallback_chain,
                };
            }
            if let Some(idx) = self.alias_map.get(&normalize(name)) {
                return ModelResolution {
                    requested: Some(name.to_string()),
                    resolved: preserve_requested_model_id_case(self.models[*idx].clone(), name),
                    used_fallback: false,
                    fallback_chain,
                };
            }
        }

        let provider = provider_hint.unwrap_or(ProviderKind::Deepseek);
        fallback_chain.push(format!("provider_default:{}", provider.as_str()));
        if let Some(model) = self.models.iter().find(|m| m.provider == provider).cloned() {
            return ModelResolution {
                requested: requested.map(ToOwned::to_owned),
                resolved: model,
                used_fallback: true,
                fallback_chain,
            };
        }

        let final_fallback = self.models.first().cloned().unwrap_or(ModelInfo {
            id: "deepseek-v4-pro".to_string(),
            provider: ProviderKind::Deepseek,
            aliases: Vec::new(),
            supports_tools: true,
            supports_reasoning: true,
        });
        fallback_chain.push("global_default:deepseek-v4-pro".to_string());
        ModelResolution {
            requested: requested.map(ToOwned::to_owned),
            resolved: final_fallback,
            used_fallback: true,
            fallback_chain,
        }
    }
}

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn model_matches(model: &ModelInfo, requested: &str) -> bool {
    let requested = normalize(requested);
    normalize(&model.id) == requested
        || model
            .aliases
            .iter()
            .any(|alias| normalize(alias) == requested)
}

fn preserve_requested_model_id_case(mut model: ModelInfo, requested: &str) -> ModelInfo {
    let requested = requested.trim();
    if model.id.eq_ignore_ascii_case(requested) {
        model.id = requested.to_string();
    }
    model
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_family_maps_representative_model_ids() {
        let cases = [
            ("deepseek-v4-pro", ModelFamily::DeepSeek),
            ("deepseek/deepseek-v4-flash", ModelFamily::DeepSeek),
            ("anthropic/claude-opus-4-7", ModelFamily::Anthropic),
            ("claude-3-5-sonnet", ModelFamily::Anthropic),
            ("openai/gpt-5.4", ModelFamily::OpenAI),
            ("gpt-4.1-mini", ModelFamily::OpenAI),
            ("google/gemini-3.1-pro", ModelFamily::Google),
            ("gemini-2.5-pro", ModelFamily::Google),
            ("meta-llama/llama-3.3-70b-instruct", ModelFamily::Meta),
            ("llama3.3:70b", ModelFamily::Meta),
            ("mistralai/mistral-large", ModelFamily::Mistral),
            ("qwen/qwen3-coder", ModelFamily::Qwen),
            ("x-ai/grok-4", ModelFamily::Grok),
            ("cohere/command-r-plus", ModelFamily::Cohere),
            ("openai/gpt-oss-120b", ModelFamily::GptOss),
            ("gpt-oss:20b", ModelFamily::GptOss),
            ("openrouter/auto", ModelFamily::Inferencer),
            ("unknown-model", ModelFamily::Unknown),
        ];

        for (model_id, expected) in cases {
            assert_eq!(model_family(model_id), expected, "{model_id}");
        }
    }

    #[test]
    fn routed_ids_prefer_underlying_family_over_gateway() {
        let cases = [
            (
                "openrouter/meta-llama/llama-3.3-70b-instruct",
                ModelFamily::Meta,
            ),
            ("groq/openai/gpt-oss-120b", ModelFamily::GptOss),
            ("together/qwen/qwen3-coder", ModelFamily::Qwen),
            (
                "fireworks/deepseek-ai/deepseek-v4-pro",
                ModelFamily::DeepSeek,
            ),
            ("deepinfra/google/gemini-3.1-pro", ModelFamily::Google),
        ];

        for (model_id, expected) in cases {
            assert_eq!(model_family(model_id), expected, "{model_id}");
        }
    }

    #[test]
    fn model_family_palettes_are_stable_and_distinct() {
        assert_eq!(
            ModelFamilyPalette::for_family(ModelFamily::DeepSeek).accent,
            (72, 140, 220)
        );
        assert_eq!(
            ModelFamilyPalette::for_family(ModelFamily::Qwen).tool_call,
            (188, 146, 238)
        );
        assert_ne!(
            ModelFamilyPalette::for_family(ModelFamily::DeepSeek).accent,
            ModelFamilyPalette::for_family(ModelFamily::Anthropic).accent
        );
    }

    #[test]
    fn deepseek_v4_pro_alias_stays_deepseek_by_default() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("deepseek-v4-pro"), None);

        assert_eq!(resolved.resolved.provider, ProviderKind::Deepseek);
        assert_eq!(resolved.resolved.id, "deepseek-v4-pro");
    }

    #[test]
    fn deepseek_v4_pro_alias_resolves_to_nvidia_nim_when_provider_hinted() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("deepseek-v4-pro"), Some(ProviderKind::NvidiaNim));

        assert_eq!(resolved.resolved.provider, ProviderKind::NvidiaNim);
        assert_eq!(resolved.resolved.id, "deepseek-ai/deepseek-v4-pro");
    }

    #[test]
    fn nvidia_nim_default_uses_catalog_model_id() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(None, Some(ProviderKind::NvidiaNim));

        assert_eq!(resolved.resolved.provider, ProviderKind::NvidiaNim);
        assert_eq!(resolved.resolved.id, "deepseek-ai/deepseek-v4-pro");
    }

    #[test]
    fn deepseek_v4_flash_alias_resolves_to_nvidia_nim_when_provider_hinted() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("deepseek-v4-flash"), Some(ProviderKind::NvidiaNim));

        assert_eq!(resolved.resolved.provider, ProviderKind::NvidiaNim);
        assert_eq!(resolved.resolved.id, "deepseek-ai/deepseek-v4-flash");
    }

    #[test]
    fn openrouter_default_uses_namespaced_model_id() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(None, Some(ProviderKind::Openrouter));

        assert_eq!(resolved.resolved.provider, ProviderKind::Openrouter);
        assert_eq!(resolved.resolved.id, "deepseek/deepseek-v4-pro");
    }

    #[test]
    fn wanjie_ark_default_uses_reasoner_model_id() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(None, Some(ProviderKind::WanjieArk));

        assert_eq!(resolved.resolved.provider, ProviderKind::WanjieArk);
        assert_eq!(resolved.resolved.id, "deepseek-reasoner");
        assert!(resolved.resolved.supports_reasoning);
    }

    #[test]
    fn novita_default_uses_namespaced_model_id() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(None, Some(ProviderKind::Novita));

        assert_eq!(resolved.resolved.provider, ProviderKind::Novita);
        assert_eq!(resolved.resolved.id, "deepseek/deepseek-v4-pro");
    }

    #[test]
    fn fireworks_default_uses_canonical_model_id() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(None, Some(ProviderKind::Fireworks));

        assert_eq!(resolved.resolved.provider, ProviderKind::Fireworks);
        assert_eq!(
            resolved.resolved.id,
            "accounts/fireworks/models/deepseek-v4-pro"
        );
    }

    #[test]
    fn sglang_default_uses_canonical_model_id() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(None, Some(ProviderKind::Sglang));

        assert_eq!(resolved.resolved.provider, ProviderKind::Sglang);
        assert_eq!(resolved.resolved.id, "deepseek-ai/DeepSeek-V4-Pro");
    }

    #[test]
    fn deepseek_v4_flash_alias_resolves_to_openrouter_when_provider_hinted() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("deepseek-v4-flash"), Some(ProviderKind::Openrouter));

        assert_eq!(resolved.resolved.provider, ProviderKind::Openrouter);
        assert_eq!(resolved.resolved.id, "deepseek/deepseek-v4-flash");
    }

    #[test]
    fn deepseek_v4_flash_alias_resolves_to_novita_when_provider_hinted() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("deepseek-v4-flash"), Some(ProviderKind::Novita));

        assert_eq!(resolved.resolved.provider, ProviderKind::Novita);
        assert_eq!(resolved.resolved.id, "deepseek/deepseek-v4-flash");
    }

    #[test]
    fn deepseek_v4_flash_alias_resolves_to_sglang_when_provider_hinted() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("deepseek-v4-flash"), Some(ProviderKind::Sglang));

        assert_eq!(resolved.resolved.provider, ProviderKind::Sglang);
        assert_eq!(resolved.resolved.id, "deepseek-ai/DeepSeek-V4-Flash");
    }

    #[test]
    fn vllm_default_uses_canonical_model_id() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(None, Some(ProviderKind::Vllm));

        assert_eq!(resolved.resolved.provider, ProviderKind::Vllm);
        assert_eq!(resolved.resolved.id, "deepseek-ai/DeepSeek-V4-Pro");
    }

    #[test]
    fn ollama_default_uses_small_local_model_id() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(None, Some(ProviderKind::Ollama));

        assert_eq!(resolved.resolved.provider, ProviderKind::Ollama);
        assert_eq!(resolved.resolved.id, "deepseek-coder:1.3b");
        assert!(!resolved.resolved.supports_reasoning);
    }

    #[test]
    fn ollama_requested_model_tag_is_preserved() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("qwen2.5-coder:7b"), Some(ProviderKind::Ollama));

        assert_eq!(resolved.resolved.provider, ProviderKind::Ollama);
        assert_eq!(resolved.resolved.id, "qwen2.5-coder:7b");
        assert!(!resolved.used_fallback);
    }

    #[test]
    fn deepseek_v4_flash_alias_resolves_to_vllm_when_provider_hinted() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("deepseek-v4-flash"), Some(ProviderKind::Vllm));

        assert_eq!(resolved.resolved.provider, ProviderKind::Vllm);
        assert_eq!(resolved.resolved.id, "deepseek-ai/DeepSeek-V4-Flash");
    }

    #[test]
    fn preserves_requested_model_casing_for_third_party_providers() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("DeepSeek-V4-Pro"), None);

        assert_eq!(resolved.resolved.provider, ProviderKind::Deepseek);
        assert_eq!(resolved.resolved.id, "DeepSeek-V4-Pro");
    }

    #[test]
    fn preserves_requested_model_casing_with_provider_hint() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("DeepSeek-V4-Pro"), Some(ProviderKind::Deepseek));

        assert_eq!(resolved.resolved.provider, ProviderKind::Deepseek);
        assert_eq!(resolved.resolved.id, "DeepSeek-V4-Pro");
    }

    #[test]
    fn preserves_requested_model_casing_without_surrounding_whitespace() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("  DeepSeek-V4-Pro  "), None);

        assert_eq!(resolved.resolved.provider, ProviderKind::Deepseek);
        assert_eq!(resolved.resolved.id, "DeepSeek-V4-Pro");
    }

    #[test]
    fn alias_match_does_not_override_requested_casing() {
        let registry = ModelRegistry::default();
        let resolved = registry.resolve(Some("deepseek-reasoner"), None);

        assert_eq!(resolved.resolved.provider, ProviderKind::Deepseek);
        assert_eq!(resolved.resolved.id, "deepseek-v4-flash");
    }
}
