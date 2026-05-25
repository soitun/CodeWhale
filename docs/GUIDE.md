# CodeWhale User Guide

A practical walkthrough for getting the most out of CodeWhale — a DeepSeek-first
agentic terminal for coding with open-weight models.

## Getting Started

```bash
# Install (pick one)
npm install -g codewhale                          # npm
cargo install codewhale-cli --locked --force      # Cargo (need both)
cargo install codewhale-tui --locked --force
brew install deepseek-tui                         # Homebrew

# Set your API key
export DEEPSEEK_API_KEY="sk-..."
codewhale auth set --provider deepseek            # or save to config
codewhale doctor                                  # verify setup

# Launch
codewhale
codewhale --model auto                            # auto-routing
codewhale -p "Fix the failing test in src/lib.rs" # one-shot
```

## Key Features at a Glance

| Feature | What it does |
|---|---|
| **Model auto-routing** | `--model auto` picks the right model + thinking level per turn |
| **Thinking-mode streaming** | See DeepSeek reasoning blocks in real time |
| **Three modes** | Plan (read-only), Agent (interactive approval), YOLO (auto-approved) |
| **Sub-agents** | Dispatch parallel workers for file ops, search, review, and verification |
| **Session save/resume** | Checkpoint long sessions and fork conversations |
| **MCP protocol** | Connect external tools via Model Context Protocol |
| **RLM sessions** | Persistent Python REPL for batch analysis over large files |
| **Skills system** | Installable instruction packs from GitHub |
| **1M-token context** | Prefix-cache-aware cost tracking and optional compaction |

## The Three Modes

### Plan Mode
Read-only exploration. The agent can read files, search code, and browse the web
but cannot edit, run commands, or modify state. Use for understanding a codebase
before committing to changes.

```bash
codewhale --mode plan
# or inside a session: /mode plan
```

### Agent Mode
Interactive with per-action approval. The agent proposes tool calls (edits,
shell commands, git operations) and you approve or deny each one. This is the
default mode and the safest for sensitive work.

```bash
codewhale                          # default is agent mode
```

### YOLO Mode
Auto-approved. All tool calls execute without prompting. Use when you trust the
workspace state and want uninterrupted work — the agent runs until the task is
done or you interrupt.

```bash
codewhale --yolo
```

## Model Auto-Routing

Use `/model auto` or `--model auto` to let CodeWhale decide how much reasoning
power each turn needs. A cheap routing call (Fin) inspects your request and
picks:

- **Model**: `deepseek-v4-flash` (fast) or `deepseek-v4-pro` (deep reasoning)
- **Thinking**: `off`, `high`, or `max`

Short lookups stay on Flash with thinking off. Architecture, debugging, and
security review move up to Pro with higher thinking. You can also lock to a
fixed model:

```bash
/model deepseek-v4-pro          # force Pro for this session
/model deepseek-v4-flash        # force Flash
Shift + Tab                     # cycle thinking: off → high → max
```

## Slash Commands

Type `/` in the composer to see the command palette. Essential commands:

| Command | Action |
|---|---|
| `/help` | Show all commands |
| `/model auto` | Enable auto-routing |
| `/mode plan` | Switch to Plan mode |
| `/yolo` | Switch to YOLO mode |
| `/sessions` | Open session picker (Ctrl+R) |
| `/save` | Save current session |
| `/compact` | Compress context to free space |
| `/theme` | Switch color theme |
| `/skills` | List installed skills |
| `/balance` | Check provider balance (coming soon) |
| `/doctor` | Run setup diagnostics |
| `/voice` | Voice input via STT helper |
| `/quit` | Exit |

## Sessions

CodeWhale saves your conversation automatically. Key session commands:

- **Ctrl+R** — open session picker to resume, fork, or rename past sessions
- **`/save`** — save the current session to disk
- **`--continue` / `-c`** — resume your most recent session for this workspace
- **Fork** — copy a session into a new branch to explore alternatives

Sessions live in `~/.codewhale/sessions/` (or `~/.deepseek/sessions/` for
legacy installs). The session picker shows titles, timestamps, and parent
lineage for forked sessions.

## Sub-Agents (Brother Whales)

Sub-agents run in parallel — like a concurrent task queue. Use them when you
need to:

- Search multiple directories at once
- Run independent file operations
- Delegate verification to a reviewer
- Offload long-running analysis

The parent agent stays responsive while children work. When a sub-agent
finishes, its findings appear in the transcript with a summary card. Finished
sub-agents show their whale-species name in the sidebar.

```text
agent_open → child starts working (whale name: "Blue")
agent_open → second child starts (whale name: "Beluga")
... parent continues working ...
<subagent.done> Blue finished: "Found 3 matches in src/"
<subagent.done> Beluga finished: "Tests pass, 0 failures"
```

## RLM Sessions

For large files, batch classification, or structured analysis, use RLM
(Recursive Language Model) sessions:

- **`rlm_open`** — load a file, URL, or session object into a Python REPL
- **`rlm_eval`** — run Python code against the loaded context
- **`rlm_session_objects`** — list symbolic session:// refs for inspection
- **`rlm_close`** — tear down the session

RLM keeps large payloads out of the main transcript. Use helpers like `peek`,
`search`, `chunk`, and `sub_query_batch` for efficient analysis.

## Tips

### Keep Context Lean
- Suggest `/compact` when context passes 60% (check the footer)
- Use RLM for large-file analysis instead of repeated reads
- Close sub-agents when their work is integrated

### Prefix-Cache Economics
- DeepSeek caches shared prefixes at 128-token granularity (~90% discount)
- Prefer appending to existing messages over editing old ones
- The cache-hit chip in the footer turns red below 40% — time to consolidate

### Keyboard Shortcuts
| Key | Action |
|---|---|
| Ctrl+R | Session picker |
| Shift+Tab | Cycle thinking level |
| Ctrl+C | Cancel / interrupt |
| Ctrl+D | Quit |
| Enter | Send message |
| Escape | Dismiss picker/modal |

### Cost Tracking
The footer shows per-turn and session-level token usage with cost estimates.
The cache-hit chip tells you how stable your prefix is. CNY display activates
automatically when the session locale is `zh-Hans`.

## Next Steps

- [Architecture overview](ARCHITECTURE.md) — codebase internals
- [Configuration reference](CONFIGURATION.md) — every setting explained
- [MCP integration](MCP.md) — connect external tools
- [Sub-agents deep dive](SUBAGENTS.md) — role taxonomy and lifecycle
- [Keybindings catalog](KEYBINDINGS.md) — full shortcut reference
- [RLM branching roadmap](RLM_BRANCHING_ROADMAP.md) — future RLM features
