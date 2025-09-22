from ._core import run, Action, AgentState, GameState, Team
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
    "Team"
]