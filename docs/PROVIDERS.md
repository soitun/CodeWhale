# Provider Registry

CodeWhale supports multiple AI providers through an OpenAI-compatible API layer.
This document catalogs each supported provider, its model families, and configuration
details.

## DeepSeek (Primary)

**Canonical name:** `deepseek`
**API base:** `https://api.deepseek.com`
**Auth:** API key via `DEEPSEEK_API_KEY` env var or `codewhale auth set --provider deepseek`
**Config:** `[provider]` section in `~/.codewhale/config.toml`

### Models

| Model ID | Type | Thinking | Context | Notes |
|---|---|---|---|---|
| `deepseek-v4-pro` | Reasoning | Yes (high/max) | 1M tokens | Primary coding model |
| `deepseek-v4-flash` | Fast | Optional (off/high) | 1M tokens | Fast lane + Fin routing |
| `deepseek-chat` | Legacy alias | No | 64K | Maps to v4-flash |
| `deepseek-reasoner` | Legacy alias | Yes | 64K | Maps to v4-pro |

### Billing
- Balance check: `GET https://api.deepseek.com/user/balance`
- `/balance` slash command available (v0.8.45+)
- Cost tracking with USD and CNY display

---

## DeepSeek CN (China Region)

**Canonical name:** `deepseek-cn`
**API base:** `https://api.deepseek.com` (same global endpoint)
**Notes:** For users in mainland China. Same models as global DeepSeek.

---

## OpenAI-Compatible Providers

These providers use the OpenAI `/v1/chat/completions` protocol.

### OpenRouter

**Canonical name:** `openrouter`
**API base:** `https://openrouter.ai/api/v1`
**Auth:** API key via `OPENROUTER_API_KEY` or custom config
**Models:** 200+ models from Anthropic, Meta, Google, Mistral, etc.
**Notes:** Usage-based pricing, model routing available

### Novita

**Canonical name:** `novita`
**API base:** Provider-specific
**Models:** Open-weight model hosting

### Fireworks

**Canonical name:** `fireworks`
**API base:** `https://api.fireworks.ai/inference/v1`
**Models:** Mixtral, Llama, Qwen, DeepSeek open-weight variants

### SiliconFlow

**Canonical name:** `siliconflow`
**API base:** Provider-specific
**Models:** Qwen, DeepSeek, Llama variants for China region

---

## Hugging Face Inference Providers

**Canonical name:** `huggingface`
**API base:** `https://router.huggingface.co/hf-inference`
**Auth:** HF token via `HF_TOKEN` or `HUGGINGFACE_TOKEN`
**Models:** `Qwen/*`, `deepseek-ai/*`, `meta-llama/*`, `mistralai/*`
**Status:** First-class provider promotion in v0.8.47

### Configuration

```toml
[provider]
provider = "huggingface"
api_key = "${HF_TOKEN}"
model = "deepseek-ai/DeepSeek-V4"
```

---

## Self-Hosted & Local

### Ollama

**Canonical name:** `ollama`
**API base:** `http://localhost:11434`
**Auth:** None (local)
**Models:** `llama3.2`, `qwen2.5`, `deepseek-r1`, `mistral`, `codellama`
**Notes:** Pull models first: `ollama pull llama3.2`

### vLLM / SGLang

**Canonical name:** Custom endpoint
**API base:** `http://localhost:8000/v1`
**Auth:** None (local) or API key
**Notes:** Set `base_url` and `model` in provider config

### NVIDIA NIM

**Canonical name:** `nvidia`
**API base:** `https://integrate.api.nvidia.com/v1`
**Auth:** NVIDIA API key
**Models:** `nvidia/llama-3.1-nemotron-70b-instruct`, etc.

---

## Custom Endpoints

Any OpenAI-compatible endpoint can be configured:

```toml
[provider]
base_url = "https://your-endpoint.com/v1"
api_key = "${YOUR_KEY}"
model = "your-model-id"
```

### Provider-specific model IDs

Some providers require specific model ID formats:
- **DeepSeek:** `deepseek-v4-pro`, `deepseek-v4-flash`
- **OpenRouter:** `openai/gpt-4o`, `anthropic/claude-sonnet-4-20250514`
- **Hugging Face:** `deepseek-ai/DeepSeek-V4`, `meta-llama/Llama-4-Maverick-17B-128E-Instruct`
- **Ollama:** `llama3.2:latest`, `qwen2.5:14b`
- **Custom:** Whatever your endpoint accepts

Use `codewhale doctor` to verify your provider configuration.
