# CodeWhale

> An open source terminal coding agent, built to bring the best available models
> to as many people as possible.

CodeWhale is a terminal coding agent — a TUI and a CLI. You point it at a model
and a project, and it gets to work: reading code, making edits, running
commands, checking results, planning multi-step tasks, and correcting itself
when something fails.

It's open source (MIT, Rust), it runs on your machine, and it works with the
models people actually use. DeepSeek and open-weight models are first-class,
but Claude, GPT, Kimi, and a local vLLM/Ollama box on your LAN are all full
peers. The goal is simple: stay current with the best research and features in
commercial coding agents, and surpass them.

Developers from all over the world have shaped CodeWhale into what it is. If
there's a model, endpoint, or feature you don't see that you want, open an issue
— that's how the project grows.

[简体中文 README](README.zh-CN.md) · [日本語 README](README.ja-JP.md) · [Tiếng Việt README](README.vi.md) · [codewhale.net](https://codewhale.net/) · [Install guide](docs/INSTALL.md) · [Provider registry](docs/PROVIDERS.md) · [Changelog](CHANGELOG.md)

[![CI](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml/badge.svg)](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/codewhale-cli?label=crates.io)](https://crates.io/crates/codewhale-cli)
[![npm](https://img.shields.io/npm/v/codewhale?label=npm)](https://www.npmjs.com/package/codewhale)
[![DeepWiki project index](https://img.shields.io/badge/DeepWiki-project-blue)](https://deepwiki.com/Hmbown/CodeWhale)

![CodeWhale running in a terminal](assets/screenshot.png)

## Install

```bash
npm install -g codewhale
codewhale --version   # 0.8.61
```

The npm wrapper (Node 18+) downloads SHA-256-verified binaries from GitHub
Releases and installs `codewhale`, `codew`, and `codewhale-tui`. Prefer building
from source? Use cargo (Rust 1.88+):

```bash
cargo install codewhale-cli --locked
cargo install codewhale-tui --locked
```

Every other path:

```bash
# Docker
docker pull ghcr.io/hmbown/codewhale:latest

# Nix
nix run github:Hmbown/CodeWhale

# Windows
scoop install codewhale        # or the NSIS installer from GitHub Releases

# CNB mirror for users who cannot reliably reach GitHub
cargo install --git https://cnb.cool/codewhale.net/codewhale --tag v0.8.61 codewhale-cli --locked --force
cargo install --git https://cnb.cool/codewhale.net/codewhale --tag v0.8.61 codewhale-tui --locked --force

# Legacy Homebrew compatibility while the formula is renamed
brew tap Hmbown/deepseek-tui
brew install deepseek-tui
```

Prebuilt archives for every platform — including Linux riscv64 — are attached
to [GitHub Releases](https://github.com/Hmbown/CodeWhale/releases). Checksums,
China mirrors, Windows specifics, and troubleshooting live in
[docs/INSTALL.md](docs/INSTALL.md).

**Upgrading from the legacy `deepseek-tui` package?** Your config, sessions,
skills, and MCP settings are preserved. See [docs/REBRAND.md](docs/REBRAND.md),
then run `codewhale doctor` to confirm.

## First Run

```bash
codewhale auth set --provider deepseek
codewhale auth status
codewhale doctor
codewhale
```

Every provider is the same one-line shape: `--provider openrouter`,
`--provider moonshot`, or point `vllm`, `sglang`, or `ollama` at your own
localhost runtime with no key at all. Have a Claude key instead? Run
`codewhale auth set --provider anthropic` — or just export
`ANTHROPIC_API_KEY` — and the native Messages adapter takes it from there.

Keys land in `~/.codewhale/config.toml`; legacy `~/.deepseek/` config is still
read for compatibility.

Useful in-session commands:

- `/provider` and `/model` switch the route and model mid-session.
- `/restore` rolls back a prior turn from side-git snapshots.
- `/skills` loads reusable workflows from `~/.codewhale/skills/`.
- `/config` edits runtime settings; `/statusline` shows the current route,
  cost, and session state.
- `! cargo test -p codewhale-tui` runs any shell command through the normal
  approval and sandbox path.

Headless, for scripts and CI:

```bash
codewhale exec --allowed-tools read_file,exec_shell --max-turns 10 "fix the failing test"
```

## The models

Twenty-five providers route through the same harness and the same tools. If the
one you want isn't here, that's a good issue to open.

- **Open models, hosted:** `deepseek` (first among equals), `openrouter`,
  `huggingface` (Inference Providers), `moonshot` (Kimi — OAuth temporarily
  broken), `zai` (GLM — recommended), `minimax`, `volcengine` (Ark),
  `nvidia-nim`, `together`, `fireworks`, `novita`, `siliconflow` /
  `siliconflow-CN`, `arcee`, `xiaomi-mimo`, `deepinfra`, `stepfun`,
  `atlascloud`, `wanjie-ark`, plus a generic `openai`-compatible route for any
  gateway.
- **Open models, self-hosted:** `vllm`, `sglang`, and `ollama` against your own
  localhost endpoints — no key required.
- **Closed providers, natively:** `anthropic` through a dedicated
  `/v1/messages` adapter with adaptive thinking, prompt-cache breakpoints, and
  signed-thinking replay — and `openai-codex`, which reuses an existing
  ChatGPT/Codex CLI login (working).

Routing is more than a base URL swap: `/reasoning` effort is translated into
each provider's wire dialect, sub-agent tiers resolve per provider, and the
system prompt's model facts are templated per-model instead of hardcoded.
Switch mid-session with `/provider` and `/model`. The full registry —
credentials, base URLs, capability boundaries — lives in
[docs/PROVIDERS.md](docs/PROVIDERS.md).

## What makes it agentic

A lot of "agents" will read a file, suggest a change, and stop — leaving you to
apply it, run it, and find out it was wrong. CodeWhale is built to go further:

- **It plans before it acts.** Multi-step work gets a real plan and a checklist,
  not a stream of guesses. You can see the plan, adjust it, and watch it update.
- **It verifies its own work.** After writing a file, it reads it back. After
  running a test, it looks at the output. A failure is evidence it adapts to,
  not a dead end it reports and forgets.
- **It runs long tasks.** Sessions persist across restarts and system sleep.
  A task that takes forty tool calls survives the forty-first.
- **It fans out sub-agents.** Independent investigations run in parallel — up
  to 20 at once — so a broad question gets answered by several focused reads,
  not one slow sequential pass.
- **It undoes cleanly.** Side-git snapshots and `/restore`, kept outside your
  repo's `.git`. When a turn goes wrong, you roll it back without touching your
  history.

The safety rails are real mechanisms, not advice the model has to remember:
approval gates, OS sandboxing (bwrap, Landlock, Seatbelt, seccomp), and a
`.codewhale/hooks.toml` hook system that can allow, deny, or ask before any tool
call. You decide how much autonomy to grant — Plan mode is read-only by default,
Agent mode asks per action, YOLO auto-approves.

## The project

CodeWhale started as one person's DeepSeek side project. Developers from
countries all over the world have made it what it is — the contributor list on
every release is the proof. The project is built in the open, issues are
triaged in the open, and releases cut from `main`.

Something I learned early in teaching: **all feedback is a gift.** Issues, PRs,
bug reports, feature ideas, "first PR"s, and curious questions all count as real
project work. Maintainers treat every report as a contribution even when the
final patch has to be narrowed, delayed, or folded into a maintainer commit —
and recurring contributors stay credited in the public record. If you hit
something that doesn't work, or you want a model that isn't listed, that's the
most useful thing you can tell the project.

- [Open issues](https://github.com/Hmbown/CodeWhale/issues) — good first
  contributions live here.
- [CONTRIBUTING.md](CONTRIBUTING.md) — set up a dev loop and open a PR.
- [Code of Conduct](CODE_OF_CONDUCT.md) — be excellent to each other.
- [Contributors](docs/CONTRIBUTORS.md) — the people who've shaped CodeWhale.

Support: [Buy me a coffee](https://www.buymeacoffee.com/hmbown).

## Where details live

The README is the short version. The rest is in docs and on
[codewhale.net](https://codewhale.net/):

- [User guide](docs/GUIDE.md) · [Install guide](docs/INSTALL.md) ·
  [Configuration](docs/CONFIGURATION.md) · [Provider registry](docs/PROVIDERS.md)
- [Modes](docs/MODES.md) — Agent, Plan, and YOLO.
- [Sub-agents](docs/SUBAGENTS.md) — roles, lifecycle, output contract, and
  recovery behavior.
- [Architecture](docs/ARCHITECTURE.md) — crate layout, runtime flow, tool system,
  extension points, and security model.
- [Fleet](docs/FLEET.md) · [WhaleFlow authoring](docs/WHALEFLOW_AUTHORING.md) ·
  [MCP](docs/MCP.md) · [Runtime API](docs/RUNTIME_API.md) ·
  [Model Lab](docs/MODEL_LAB.md)
- [Keybindings](docs/KEYBINDINGS.md) · [Sandbox & approvals](docs/SANDBOX.md)
  · [Accessibility](docs/ACCESSIBILITY.md) · [Docker](docs/DOCKER.md)
  · [Memory](docs/MEMORY.md)
- [Full docs index](docs) — everything else.

## Thanks

CodeWhale exists because of the people who use it, break it, and fix it.

- **[DeepSeek](https://github.com/deepseek-ai)** — the models and support that
  got this project started. 感谢 DeepSeek 提供模型与支持。
- **[DataWhale](https://github.com/datawhalechina)** 🐋 — for the support and for
  welcoming us into the Whale Brother family. 感谢 DataWhale 的支持。
- **[OpenWarp](https://github.com/zerx-lab/warp)** and
  **[Open Design](https://github.com/nexu-io/open-design)** — for collaborating
  on a better terminal-agent experience.
- **Every contributor** — the full per-PR record lives in
  [docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md). Thank you.

## License

[MIT](LICENSE)

> *CodeWhale is an independent community project and is not affiliated with any
> model provider.*

## Star History

[![Star History Chart](https://api.star-history.com/chart?repos=Hmbown/CodeWhale&type=date&legend=top-left)](https://www.star-history.com/?repos=Hmbown%2FCodeWhale&type=date&logscale=&legend=top-left)
