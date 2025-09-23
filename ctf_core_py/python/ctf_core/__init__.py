from ._core import run, segment_is_free, Action, AgentState, GameState, Team
from typing import Protocol, runtime_checkable

@runtime_checkable
class AgentProtocol(Protocol):
    def startup(self, state: GameState) -> None: ...

    def get_actions(self, state: GameState) -> list[Action]: ...

__all__ = [
    "Action",
    "AgentProtocol",
    "AgentState",
    "GameState",
    "run",
    "segment_is_free",
    "Team"
]