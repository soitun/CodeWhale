# CodeWhale User Guide

A practical guide to getting productive with CodeWhale — the DeepSeek-first
agentic terminal for open-source coding models. This covers the interactive
TUI, not the one-shot CLI or automation paths. For those, see the
[README](../README.md) and [Runtime API](RUNTIME_API.md).

## 1. Getting started

### Install

Pick one path. All of them put `codewhale` (dispatcher) and `codewhale-tui`
(runtime) on your `PATH`. The short alias `codew` works everywhere.

```bash
# npm (Node.js wrapper)
npm install -g codewhale

# Cargo (Rust 1.88+)
cargo install codewhale-cli --locked
cargo install codewhale-tui --locked

# Homebrew (macOS)
brew tap Hmbown/deepseek-tui
brew install deepseek-tui
```

### First launch

```bash
codewhale
```

CodeWhale prompts for your [DeepSeek API key](https://platform.deepseek.com/api_keys)
on first launch. The key is saved to `~/.codewhale/config.toml`. You can also
set it ahead of time:

```bash
codewhale auth set --provider deepseek
# or
export DEEPSEEK_API_KEY="your-key"
```

Run `codewhale doctor` to verify connectivity.

## 2. Key concepts

CodeWhale is an **agentic terminal**: it can read your files, search your
codebase, run shell commands, edit code, and apply patches — all with
structured tools the model chooses. Every tool use is visible in the
transcript and most are gated behind an approval prompt.

| Concept | What it means |
|---|---|
| **Turn** | One prompt → model response cycle. The model may use many tools inside one turn. |
| **Session** | A saved conversation. Survives restart. Resumable, forkable, exportable. |
| **Tools** | Structured actions: `read_file`, `grep_files`, `exec_shell`, `edit_file`, `apply_patch`, etc. |
| **Sub-agent** | A background child agent launched with `agent_open`. Runs independently. |
| **Skill** | A reusable instruction file (`SKILL.md`). Activated with `/skill`. |
| **RLM** | Persistent Python REPL session for data exploration and batch processing. |
| **Checklist** | Granular progress tracking inside a turn. The model uses `checklist_write`. |

## 3. The TUI layout

```
┌──────────────────────────────────────────────────┬──────────────┐
│  Header: session title, model, mode, token count  │              │
├──────────────────────────────────────────────────┤   Sidebar    │
│                                                  │  Work/Tasks/ │
│              Transcript pane                     │  Agents/     │
│  (scrollable, selectable, yankable)              │  Context     │
│                                                  │              │
├──────────────────────────────────────────────────┤              │
│  Status area: live tool calls, queued drafts     │              │
├──────────────────────────────────────────────────┤              │
│  Composer: type a message or /slash-command       │              │
└──────────────────────────────────────────────────┴──────────────┘
```

- **Transcript**: scroll with arrows/`j`/`k`, select with `v`, yank with `y`.
  Press `Esc` to return focus to the composer.
- **Sidebar**: toggle with `Ctrl-Shift-E`. Cycle panels with `Tab` when
  focused. Use `Alt-1` through `Alt-4` or `Alt-0` to jump directly.
- **Composer**: type messages, `/slash commands`, or `@file` mentions.
  `Alt-Enter` inserts a newline.

## 4. Modes

Press `Tab` to cycle modes: **Plan → Agent → YOLO**. Press `Shift-Tab` to
cycle reasoning effort: **off → high → max**.

| Mode | Behavior | Tools | Approvals |
|---|---|---|---|
| **Plan** 🔍 | Design-first. Explore, read, plan. No changes. | Read-only | — |
| **Agent** 🤖 | Multi-step tool use with approval gates. | All | Shell & paid tools |
| **YOLO** ⚡ | Auto-approve everything. Trusted repos only. | All | None |

Modes are separate from model routing. `Tab` cycles modes; `/model auto`
controls model and thinking selection. See [MODES.md](MODES.md) for details.

You can also override approval behavior at runtime with `/config` → edit
`approval_mode`: `suggest` (default), `auto`, or `never`.

## 5. Model routing

CodeWhale is DeepSeek-first. The default models are `deepseek-v4-pro` and
`deepseek-v4-flash`.

### Auto-routing (`/model auto`)

When model is set to `auto`, CodeWhale makes a small routing call before each
turn using **Fin** — a low-latency `deepseek-v4-flash` path with thinking off.
Fin decides:

- **Model**: `deepseek-v4-flash` or `deepseek-v4-pro`
- **Thinking**: `off`, `high`, or `max`

Short/simple turns stay cheap on Flash. Complex coding, debugging, or
architecture work escalates to Pro with appropriate reasoning depth.

### Manual control

```bash
# CLI flags
codewhale --model deepseek-v4-flash "summarize this"
codewhale --model deepseek-v4-pro --thinking high "design a migration"

# Inside the TUI
/model deepseek-v4-pro
/model auto
```

### Other providers

CodeWhale supports multiple API providers: NVIDIA NIM, OpenRouter, AtlasCloud,
Wanjie Ark, Novita, Fireworks, SGLang, vLLM, Ollama, and generic OpenAI-compatible
endpoints. Use `/provider` to switch or `codewhale --provider <name>` at launch.

## 6. Slash commands

Type `/` in the composer to open the command palette, or `Ctrl-K` to search
commands by name. Here are the most useful ones:

### Essential

| Command | Action |
|---|---|
| `/help` | Searchable help overlay |
| `/model <name>` | Switch model |
| `/mode <plan\|agent\|yolo>` | Switch TUI mode |
| `/provider <name>` | Switch API provider |
| `/config` | Open the settings editor |
| `/theme <name>` | Switch colour theme |

### Sessions

| Command | Action |
|---|---|
| `/save [path]` | Save current session |
| `/sessions` | Browse and resume past sessions |
| `/rename <title>` | Rename current session |
| `/fork` | Branch session into a sibling |
| `/compact` | Summarise long context to save tokens |
| `/export [path]` | Export session to a file |
| `/load [path]` | Load a session from a file |

### Work management

| Command | Action |
|---|---|
| `/goal <objective>` | Set a session objective with optional token budget |
| `/subagents` | List running sub-agents |
| `/agent [N] <task>` | Launch a sub-agent (N = count for parallel work) |
| `/task add <prompt>` | Create a durable background task |
| `/jobs` | Manage background shell jobs |
| `/queue` | Manage queued follow-up drafts |
| `/stash` | Stash and recover composer drafts |

### Code & workspace

| Command | Action |
|---|---|
| `/init` | Scaffold project config |
| `/workspace [path]` | Show or switch workspace |
| `/diff` | Show working-tree diff |
| `/undo` | Revert last tool edit or turn |
| `/retry` | Resend the last user prompt |
| `/review <target>` | Run a structured code review |
| `/restore [N]` | Restore files from side-git snapshots |
| `/lsp [on\|off]` | Toggle LSP diagnostics |

### Skills & tools

| Command | Action |
|---|---|
| `/skills` | List installed skills |
| `/skill <name>` | Activate a skill |
| `/skill install github:<owner>/<repo>` | Install community skill |
| `/mcp` | Configure MCP servers |
| `/rlm [N] <input>` | Open a recursive Python REPL session |

### Info & debug

| Command | Action |
|---|---|
| `/cost` | Show session token cost |
| `/balance` | Query provider account balance |
| `/tokens` | Show token counts |
| `/context` | Show current context window usage |
| `/system` | Show the active system prompt |
| `/status` | Show session/runtime status |
| `/cache` | Inspect prefix-cache telemetry |
| `/home` | Dashboard overview |
| `/links` | DeepSeek platform links |
| `/feedback` | Send feedback or bug report |

Full catalog: [KEYBINDINGS.md](KEYBINDINGS.md).

## 7. Sessions

Sessions are saved conversations that survive restarts.

### Save and resume

CodeWhale auto-saves after each turn. You can also save manually:

```text
/save                     # save with auto-generated title
/save my-feature-review   # save with a custom name
```

Resume from the TUI:

- `Ctrl-R` opens the session picker
- `/sessions` does the same
- `codewhale resume --last` from the CLI
- `codewhale --continue` / `-c` resumes the most recent session in the current workspace

### Fork

`/fork` creates a sibling copy of the current session, preserving the parent
lineage. This is the safe way to explore an alternative direction without
overwriting the original conversation.

### Compact

Long sessions consume context window. `/compact` asks the model to summarise
the conversation so far, freeing token budget for new work. Compaction
preserves key decisions, task state, and recent context.

### Export and load

```text
/export ~/Desktop/session-export.md    # save to a portable file
/load ~/Desktop/session-export.md      # restore from a file
```

## 8. Sub-agents

Sub-agents are background child instances that run independently. The parent
launches one with a focused task and can continue working while it runs.

The model orchestrates sub-agents through three tools:

- **`agent_open`**: launch a child with a task and a role
- **`agent_eval`**: wait for and fetch the child's result
- **`agent_close`**: cancel a running child

### Roles

| Role | Stance | Typical use |
|---|---|---|
| `general` | Flexible, follows parent instructions | Multi-step tasks |
| `explore` | Read-only, maps code fast | "Find every call site of X" |
| `plan` | Analyse and produce strategy | "Design the migration" |
| `review` | Read-and-grade with severity | "Audit this PR" |
| `implementer` | Land a specific change | "Rewrite bar.rs::Foo::bar" |
| `verifier` | Run tests, report outcome | "Run cargo test --workspace" |

See [SUBAGENTS.md](SUBAGENTS.md) for the full taxonomy and context-forking
behaviour.

## 9. Tips

### Workflow

- **Start in Plan mode** for unfamiliar code. Let the model explore and
  propose before making changes.
- **Use `/goal`** to keep a session objective visible in the Work sidebar.
- **Stash drafts** with `Ctrl-S` when you're interrupted mid-thought.
  Recover with `/stash pop`.
- **Queue follow-ups** with `Tab` while a turn is running. The queued
  message becomes the next prompt automatically.
- **Use `@file` mentions** to attach specific files or directories to
  your prompt. Frecency ranking means files you often reference float up.
- **Fork before large experiments**: `/fork` gives you a safe branch of
  the conversation without risking the main session.

### Cost control

- Use `--model auto` or `/model auto` to let Fin route simple turns to
  the cheaper Flash model.
- Watch `/cost` to track cumulative token spend.
- `/compact` when a session gets long — it saves tokens on every
  subsequent turn.
- Set a token budget with `/goal "fix auth bug" budget: 50000` to cap
  the session.

### Keyboard efficiency

| Shortcut | What it does |
|---|---|
| `F1` | Help overlay |
| `Ctrl-K` | Command palette |
| `Ctrl-R` | Session picker |
| `Ctrl-S` | Stash current draft |
| `Alt-R` | Search prompt history |
| `Ctrl-O` | Activity detail / reasoning timeline |
| `Ctrl-L` | Refresh screen |
| `Esc` | Cancel / dismiss / back |

### Shell integration

The model uses `exec_shell` for build, test, format, and lint commands.
Dedicated tools (`read_file`, `grep_files`, `edit_file`, `apply_patch`)
are preferred over shell equivalents — they return structured output and
avoid platform-specific escaping.

## 10. Where to go next

| Document | Topic |
|---|---|
| [KEYBINDINGS.md](KEYBINDINGS.md) | Every keyboard shortcut, by context |
| [MODES.md](MODES.md) | Plan / Agent / YOLO in depth |
| [CONFIGURATION.md](CONFIGURATION.md) | Full config reference |
| [SUBAGENTS.md](SUBAGENTS.md) | Sub-agent role taxonomy |
| [MEMORY.md](MEMORY.md) | Persistent user memory |
| [MCP.md](MCP.md) | Model Context Protocol integration |
| [INSTALL.md](INSTALL.md) | Platform-specific install notes |
| [DOCKER.md](DOCKER.md) | Docker images and volumes |
| [TOOL_SURFACE.md](TOOL_SURFACE.md) | Every tool and its niche |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Codebase internals |

## FAQ

### What's the difference between CodeWhale and the old `deepseek-tui`?

CodeWhale is the renamed, current product. The old `deepseek-tui` name,
npm package, Cargo crates, and `~/.deepseek/` config directory are
backward-compatible during the rename transition. New installs should use
`codewhale` and `~/.codewhale/`. See [REBRAND.md](REBRAND.md).

### Do `DEEPSEEK_*` environment variables still work?

Yes. `DEEPSEEK_API_KEY`, `DEEPSEEK_BASE_URL`, and other `DEEPSEEK_*`
variables are unchanged. CodeWhale remains DeepSeek-first.

### Can I use models other than DeepSeek?

Yes, through provider adapters: NVIDIA NIM, OpenRouter, AtlasCloud,
Wanjie Ark, Novita, Fireworks, SGLang, vLLM, Ollama, and generic
OpenAI-compatible endpoints. DeepSeek models remain the default and
best-tested path.

### How do I cancel a running turn?

Press `Esc`. Cancellation is a stack: first it closes menus/modals, then
cancels the active turn, then clears the composer. `Ctrl-C` also cancels
and is a faster path when a turn is running.

### Where are sessions stored?

`~/.codewhale/sessions/`. Legacy sessions in `~/.deepseek/sessions/` are
still discoverable. Session files are JSON — portable and inspectable.
