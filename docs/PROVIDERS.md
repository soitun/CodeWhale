# Provider Registry

CodeWhale supports multiple AI providers through an OpenAI-compatible API layer.
This document catalogs each supported provider, its model families, and configuration
details. See [CONFIGURATION.md](CONFIGURATION.md) for config file syntax, profiles,
and environment-variable overrides.

## Provider overview

| Provider | Canonical name | API base URL | Auth method | Self-hosted |
|---|---|---|---|---|
| DeepSeek Platform | `deepseek` | `https://api.deepseek.com/beta` | API key (DEEPSEEK_API_KEY) | No |
| NVIDIA NIM | `nvidia-nim` | `https://integrate.api.nvidia.com/v1` | NVIDIA API key | No |
| OpenAI / compatible | `openai` | `https://api.openai.com/v1` | API key | No |
| AtlasCloud | `atlascloud` | `https://api.atlascloud.ai/v1` | API key | No |
| Wanjie Ark | `wanjie-ark` | `https://maas-openapi.wanjiedata.com/api/v1` | API key | No |
| OpenRouter | `openrouter` | `https://openrouter.ai/api/v1` | API key | No |
| Novita | `novita` | `https://api.novita.ai/v1` | API key | No |
| Fireworks AI | `fireworks` | `https://api.fireworks.ai/inference/v1` | API key | No |
| SGLang | `sglang` | `http://localhost:30000/v1` | Optional token | Yes |
| vLLM | `vllm` | `http://localhost:8000/v1` | Optional token | Yes |
| Ollama | `ollama` | `http://localhost:11434/v1` | Optional token | Yes |

Providers not listed above — such as Hugging Face Inference Providers, SiliconFlow,
or any other OpenAI-compatible gateway — use the built-in `openai` provider with a
custom `base_url`. See [Custom endpoints](#custom-endpoints).

---

## DeepSeek Platform

**Canonical name:** `deepseek`
**API base:** `https://api.deepseek.com/beta`
**Auth:** API key via `DEEPSEEK_API_KEY` env var, `codewhale auth set --provider deepseek`, or `~/.deepseek/config.toml`
**Model format:** Short aliases (`deepseek-v4-pro`, `deepseek-v4-flash`)

### Models

| Model ID | Type | Thinking | Context | Notes |
|---|---|---|---|---|
| `deepseek-v4-pro` | Reasoning | Yes (high/max) | 1M tokens | Primary coding model |
| `deepseek-v4-flash` | Fast | Optional | 1M tokens | Fast lane + Fin agent routing |
| `deepseek-chat` | Legacy alias | — | 64K | Resolves to v4-flash |
| `deepseek-reasoner` | Legacy alias | — | 64K | Resolves to v4-flash |

### Configuration

```toml
provider = "deepseek"

[providers.deepseek]
api_key = "YOUR_DEEPSEEK_API_KEY"
base_url = "https://api.deepseek.com/beta"
model = "deepseek-v4-pro"
```

### Billing

- Balance check: `GET https://api.deepseek.com/user/balance`
- `/balance` slash command available (v0.8.45+)
- Cost tracking with USD and CNY display

---

## NVIDIA NIM

**Canonical name:** `nvidia-nim`
**API base:** `https://integrate.api.nvidia.com/v1`
**Auth:** NVIDIA API key via `NVIDIA_API_KEY` (or `NVIDIA_NIM_API_KEY`) env var
**Model format:** `deepseek-ai/deepseek-v4-pro`, `deepseek-ai/deepseek-v4-flash`

### Models

| Model ID | Type | Thinking | Notes |
|---|---|---|---|
| `deepseek-ai/deepseek-v4-pro` | Reasoning | Yes | Pro hosted on NVIDIA infrastructure |
| `deepseek-ai/deepseek-v4-flash` | Fast | Optional | Flash hosted on NVIDIA infrastructure |

### Configuration

```toml
provider = "nvidia-nim"

[providers.nvidia_nim]
api_key = "YOUR_NVIDIA_API_KEY"
base_url = "https://integrate.api.nvidia.com/v1"
model = "deepseek-ai/deepseek-v4-pro"
```

### Aliases

The provider also responds to `nvidia`, `nim`.

---

## OpenAI / OpenAI-compatible gateways

**Canonical name:** `openai`
**API base:** `https://api.openai.com/v1`
**Auth:** API key via `OPENAI_API_KEY` env var
**Model format:** Passthrough — model IDs are sent unchanged to the endpoint

This is the generic OpenAI-compatible provider. Use it for any third-party
service that implements the OpenAI Chat Completions API: Hugging Face Inference
Providers, SiliconFlow, Groq, Together AI, Perplexity, xAI, DeepInfra, etc.

### Built-in registered models

| Model ID | Type | Thinking | Notes |
|---|---|---|---|
| `gpt-4.1` | Default | Yes | Registered default for unknown OpenAI models |
| `gpt-4.1-mini` | Fast | No | Smaller/cheaper variant |

Any other model ID you pass is forwarded unchanged to the base URL.

### Configuration

```toml
provider = "openai"

[providers.openai]
api_key = "YOUR_OPENAI_COMPATIBLE_API_KEY"
base_url = "https://your-gateway.example/v1"
model = "your-model-id"
```

### Common OpenAI-compatible gateways

| Gateway | Base URL | Notes |
|---|---|---|
| **Hugging Face Inference Providers** | `https://router.huggingface.co/hf-inference` | Auth via `HF_TOKEN`. Models: `deepseek-ai/*`, `Qwen/*`, `meta-llama/*`, `mistralai/*` |
| **SiliconFlow** | Provider-specific | Qwen, DeepSeek, Llama variants for China region |
| **Groq** | `https://api.groq.com/openai/v1` | LPU inference for Llama, Mixtral, Gemma |
| **Together AI** | `https://api.together.xyz/v1` | 200+ open models |
| **DeepInfra** | `https://api.deepinfra.com/v1` | Open-weight model hosting |
| **xAI** | `https://api.x.ai/v1` | Grok models |

For non-local `http://` gateways, launch with `DEEPSEEK_ALLOW_INSECURE_HTTP=1` only on a trusted network.

---

## AtlasCloud

**Canonical name:** `atlascloud`
**API base:** `https://api.atlascloud.ai/v1`
**Auth:** API key via `ATLASCLOUD_API_KEY` env var
**Model format:** `deepseek-ai/deepseek-v4-flash` (default)

### Configuration

```toml
provider = "atlascloud"

[providers.atlascloud]
api_key = "YOUR_ATLASCLOUD_API_KEY"
base_url = "https://api.atlascloud.ai/v1"
model = "deepseek-ai/deepseek-v4-flash"
```

### Aliases

The provider also responds to `atlas-cloud`, `atlas_cloud`, `atlas`.

---

## Wanjie Ark

**Canonical name:** `wanjie-ark`
**API base:** `https://maas-openapi.wanjiedata.com/api/v1`
**Auth:** API key via `WANJIE_ARK_API_KEY` (or `WANJIE_API_KEY`) env var
**Model format:** Account-scoped model IDs. Default: `deepseek-reasoner`

Model access is account-scoped on Wanjie Ark. Use the exact model ID enabled
on your Wanjie account.

### Configuration

```toml
provider = "wanjie-ark"

[providers.wanjie_ark]
api_key = "YOUR_WANJIE_API_KEY"
base_url = "https://maas-openapi.wanjiedata.com/api/v1"
model = "deepseek-reasoner"
```

### Aliases

The provider also responds to `wanjie`, `wanjie_ark`, `ark-wanjie`, `wanjie-maas`.

---

## OpenRouter

**Canonical name:** `openrouter`
**API base:** `https://openrouter.ai/api/v1`
**Auth:** API key via `OPENROUTER_API_KEY` env var
**Model format:** `provider/model-id` (e.g. `deepseek/deepseek-v4-pro`)

OpenRouter is a multi-provider gateway with usage-based pricing and optional
model routing. Over 200 models available.

### Models

| Model ID | Type | Thinking | Notes |
|---|---|---|---|
| `deepseek/deepseek-v4-pro` | Reasoning | Yes | Via OpenRouter |
| `deepseek/deepseek-v4-flash` | Fast | Optional | Via OpenRouter |

Other model IDs (e.g. `openai/gpt-4o`, `anthropic/claude-sonnet-4-20250514`)
are forwarded unchanged.

### Configuration

```toml
provider = "openrouter"

[providers.openrouter]
api_key = "YOUR_OPENROUTER_API_KEY"
base_url = "https://openrouter.ai/api/v1"
model = "deepseek/deepseek-v4-pro"
```

---

## Novita

**Canonical name:** `novita`
**API base:** `https://api.novita.ai/v1`
**Auth:** API key via `NOVITA_API_KEY` env var
**Model format:** `deepseek/deepseek-v4-pro`, `deepseek/deepseek-v4-flash`

Novita hosts open-weight model inference.

### Configuration

```toml
provider = "novita"

[providers.novita]
api_key = "YOUR_NOVITA_API_KEY"
base_url = "https://api.novita.ai/v1"
model = "deepseek/deepseek-v4-pro"
```

---

## Fireworks AI

**Canonical name:** `fireworks`
**API base:** `https://api.fireworks.ai/inference/v1`
**Auth:** API key via `FIREWORKS_API_KEY` env var
**Model format:** `accounts/fireworks/models/deepseek-v4-pro`

### Models

| Model ID | Notes |
|---|---|
| `accounts/fireworks/models/deepseek-v4-pro` | DeepSeek V4 Pro on Fireworks |

### Configuration

```toml
provider = "fireworks"

[providers.fireworks]
api_key = "YOUR_FIREWORKS_API_KEY"
base_url = "https://api.fireworks.ai/inference/v1"
model = "accounts/fireworks/models/deepseek-v4-pro"
```

### Aliases

The provider also responds to `fireworks-ai`.

---

## SGLang

**Canonical name:** `sglang`
**API base:** `http://localhost:30000/v1`
**Auth:** Optional API token (`SGLANG_API_KEY`). No key required for localhost.
**Model format:** `deepseek-ai/DeepSeek-V4-Pro`, `deepseek-ai/DeepSeek-V4-Flash`

Self-hosted SGLang OpenAI-compatible server. Loopback endpoints skip API-key
enforcement by default.

### Configuration

```toml
provider = "sglang"

[providers.sglang]
# api_key = "OPTIONAL_SGLANG_TOKEN"
base_url = "http://localhost:30000/v1"
model = "deepseek-ai/DeepSeek-V4-Pro"
```

### Aliases

The provider also responds to `sg-lang`.

---

## vLLM

**Canonical name:** `vllm`
**API base:** `http://localhost:8000/v1`
**Auth:** Optional API token (`VLLM_API_KEY`). No key required for localhost.
**Model format:** `deepseek-ai/DeepSeek-V4-Pro`, `deepseek-ai/DeepSeek-V4-Flash`

Self-hosted vLLM OpenAI-compatible server. Loopback endpoints skip API-key
enforcement by default.

### Configuration

```toml
provider = "vllm"

[providers.vllm]
# api_key = "OPTIONAL_VLLM_TOKEN"
base_url = "http://localhost:8000/v1"
model = "deepseek-ai/DeepSeek-V4-Pro"
```

### Aliases

The provider also responds to `v-llm`.

---

## Ollama

**Canonical name:** `ollama`
**API base:** `http://localhost:11434/v1`
**Auth:** Optional token (`OLLAMA_API_KEY`). No key required for localhost.
**Model format:** Ollama tags (e.g. `deepseek-coder:1.3b`, `qwen2.5-coder:7b`, `llama3.2:latest`)

Self-hosted local inference. Pull models first with `ollama pull <model>`.

Unlike other providers, Ollama preserves the exact model tag you pass. Any
model ID is forwarded unchanged.

### Configuration

```toml
provider = "ollama"

[providers.ollama]
# api_key = "OPTIONAL_OLLAMA_TOKEN"
base_url = "http://localhost:11434/v1"
model = "deepseek-coder:1.3b"
```

### Aliases

The provider also responds to `ollama-local`.

---

## Custom endpoints

Any OpenAI-compatible endpoint that does not have a dedicated provider name
can use the built-in `openai` provider with a custom `base_url`:

```toml
provider = "openai"
default_text_model = "your-model-id"

[providers.openai]
api_key = "YOUR_API_KEY"
base_url = "https://your-endpoint.example/v1"
```

### Provider-specific model ID formats

| Provider | Model ID format | Example |
|---|---|---|
| DeepSeek Platform | Short alias | `deepseek-v4-pro` |
| NVIDIA NIM | `deepseek-ai/*` | `deepseek-ai/deepseek-v4-pro` |
| OpenAI / compatible | Passthrough | `gpt-4.1`, any custom ID |
| AtlasCloud | `deepseek-ai/*` | `deepseek-ai/deepseek-v4-flash` |
| Wanjie Ark | Account-scoped | `deepseek-reasoner` |
| OpenRouter | `provider/model` | `deepseek/deepseek-v4-pro` |
| Novita | `deepseek/*` | `deepseek/deepseek-v4-pro` |
| Fireworks | `accounts/fireworks/models/*` | `accounts/fireworks/models/deepseek-v4-pro` |
| SGLang | `deepseek-ai/DeepSeek-V4-*` | `deepseek-ai/DeepSeek-V4-Pro` |
| vLLM | `deepseek-ai/DeepSeek-V4-*` | `deepseek-ai/DeepSeek-V4-Pro` |
| Ollama | `model:tag` | `qwen2.5-coder:7b` |
| Hugging Face | `org/model` | `deepseek-ai/DeepSeek-V4` |

### Custom HTTP headers

OpenAI-compatible gateways that need extra request headers can set
`http_headers` under the provider table:

```toml
[providers.openai]
api_key = "YOUR_KEY"
base_url = "https://gateway.example/v1"
http_headers = { "X-Model-Provider-Id" = "your-model-provider" }
```

Environment override: `DEEPSEEK_HTTP_HEADERS=X-Model-Provider-Id=value,X-Gateway-Route=dev`.

### Non-local HTTP endpoints

For a non-local `http://` gateway, launch with:

```bash
DEEPSEEK_ALLOW_INSECURE_HTTP=1 codewhale
```

Loopback addresses (`localhost`, `127.0.0.1`, `[::1]`, `0.0.0.0`) are allowed by default.

---

## Switching providers at runtime

Use the `/provider` slash command in the TUI to switch the active provider
without restarting:

```
/provider deepseek
/provider nvidia-nim
/provider openai
/provider ollama
```

Or pass `--provider` at launch:

```bash
codewhale --provider nvidia-nim
```

Run `codewhale doctor` to verify your provider configuration.
