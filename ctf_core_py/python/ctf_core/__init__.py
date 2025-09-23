from ._core import run, segment_is_free, Action, AgentState, GameState, Team
from typing import Protocol, runtime_checkable

@runtime_checkable
class AgentProtocol(Protocol):
    def __init__(self, side: Team) -> None: ...

    def startup(self, initial_state: GameState) -> None: ...

    def get_action(self, game_state: GameState, agent_state: AgentState) -> Action: ...

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