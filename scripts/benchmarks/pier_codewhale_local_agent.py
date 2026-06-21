"""Pier adapter for running local CodeWhale Linux artifacts.

DeepSWE uses Pier instead of plain Harbor so CLI agents can reach their model
API while the task container remains otherwise air-gapped. The local Harbor
adapter already knows how to install and run CodeWhale in a task container; this
thin wrapper adds the small Pier-specific surface that Pier calls before setup.
"""

from __future__ import annotations

from pier.models.agent.install import AgentInstallSpec
from pier.models.agent.network import NetworkAllowlist
from pier.models.trial.result import AgentInfo, ModelInfo

from scripts.benchmarks.harbor.codewhale_local_agent import (
    CodeWhaleLocalAgent as HarborCodeWhaleLocalAgent,
)


class CodeWhalePierLocalAgent(HarborCodeWhaleLocalAgent):
    """Run local CodeWhale binaries under Pier/DeepSWE."""

    def install_spec(self) -> AgentInstallSpec | None:
        return None

    def network_allowlist(self) -> NetworkAllowlist:
        provider, _model = self._provider_and_model()
        domains = {
            "deepseek": ["api.deepseek.com", ".deepseek.com"],
            "openrouter": ["openrouter.ai", "api.openrouter.ai"],
            "openai": ["api.openai.com"],
            "zai": ["api.z.ai"],
            "z-ai": ["api.z.ai"],
        }.get(provider, [])
        return NetworkAllowlist(domains=domains)

    def to_agent_info(self) -> AgentInfo:
        provider, model = self._provider_and_model()
        return AgentInfo(
            name=self.name(),
            version=self.version() or "unknown",
            model_info=ModelInfo(name=model, provider=provider),
        )
