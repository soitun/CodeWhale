# ORCA Connector Compatibility

Status: **Draft / scaffold** — tracking issue defines scope and acceptance.

ORCA Lab is a physical-AI training platform that advertises "fast direct
connect" for terminal coding agents (the same slot it offers to Claude Code /
OpenClaw). This document specifies how DeepSeek-TUI exposes itself as an
ORCA-connectable agent so an ORCA session can drive it as the coding brain for
a robot / simulation workspace.

## Design principle

DeepSeek-TUI already ships a Codex-style **app-server** transport
(`crates/app-server`) speaking JSON-RPC 2.0 over **stdio** and **HTTP**. ORCA
connectivity is implemented as a thin *connector adapter* on top of that
transport — **not** a new agent runtime. ORCA connects exactly the way it
connects to other agents: it spawns the binary in stdio mode (or dials the
HTTP listener) and performs a handshake.

```
deepseek app-server --stdio        # ORCA spawns this and speaks JSON-RPC
deepseek app-server --host : --port # or dials the HTTP listener
```

## Connector handshake

ORCA opens a session with a single discovery call before issuing work:

| Method            | Purpose                                                        |
| ----------------- | -------------------------------------------------------------- |
| `orca/handshake`  | Negotiate connector protocol version, advertise agent identity |
| `orca/capabilities` | Enumerate the ORCA-facing method surface and event stream    |

`orca/handshake` returns:

- `connector` — `"deepseek-tui"`
- `connector_protocol` — semver of this connector contract (starts at `0.1`)
- `agent` — model family the connector drives (`DeepSeek V4`)
- `transports` — `["stdio", "http"]`
- `session_model` — how ORCA maps a robot/sim run onto agent state
  (ORCA "session" ⇒ app-server **thread**)

## Mapping ORCA concepts onto the existing transport

| ORCA concept        | DeepSeek-TUI app-server                                  |
| ------------------- | -------------------------------------------------------- |
| Connect / open      | `orca/handshake` → `thread/create` → `thread/start`      |
| Send a task / step  | `thread/message` (or `prompt/run` for one-shot)          |
| Resume after pause  | `thread/resume`                                          |
| Stream tokens/tools | app-server event stream (`response_delta`, `tool_call_*`)|
| Tool execution      | `tool` route / `invoke_tool` (approval policy applies)   |
| Disconnect          | `shutdown`                                                |

No new business logic is required for the core loop: ORCA work items become
`thread/message` calls; reasoning and tool-call events already flow through the
existing event channel.

## Scope of the scaffold in this branch

This branch lands the **handshake seam only**:

- `orca/handshake` and `orca/capabilities` added to the stdio dispatch and
  advertised in the top-level `capabilities` method list.
- This document.

It deliberately does **not** yet implement: ORCA auth/token exchange, an HTTP
`/orca/handshake` route, robot-session lifecycle hooks, or telemetry
forwarding. Those are tracked as follow-up acceptance items in the issue.

## Open questions (resolve in the tracking issue)

1. Does ORCA authenticate the connector (bearer token / mTLS), and at the
   transport or handshake layer?
2. Does ORCA expect server→client notifications (push events) or strict
   request/response polling over stdio?
3. Should the robot/sim workspace path arrive via `thread/start { cwd }` or a
   dedicated `orca/*` parameter?
