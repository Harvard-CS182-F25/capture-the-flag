from ._core import run, segment_is_free, Action, AgentState, GameState, Team
from typing import Protocol, runtime_checkable

@runtime_checkable
class AgentProtocol(Protocol):
    def startup(self, state: GameState) -> None: ...

    def get_actions(self, state: GameState) -> list[Action]: ...

def point_is_free(point: tuple[float, float], timeout_ms: int | None = None) -> bool:
    return segment_is_free(point, point, timeout_ms=timeout_ms)

__all__ = [
    "Action",
    "AgentProtocol",
    "AgentState",
    "GameState",
    "point_is_free",
    "run",
    "segment_is_free",
    "Team"
]