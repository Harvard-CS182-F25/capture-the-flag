from ._core import run, run_headless, segment_is_free, Action, AgentState, FlagState, FlagStatus, GameState, Team, Config, DefenseBot
from typing import Protocol, runtime_checkable
import sys

Position = tuple[float, float]
Velocity = tuple[float, float]

dbg = lambda *values, **kwargs: print(*values, file=sys.stderr, **kwargs)

@runtime_checkable
class AgentProtocol(Protocol):
    def __init__(self, side: Team) -> None: ...

    def startup(self, initial_state: GameState) -> None: ...

    def get_action(self, game_state: GameState, agent_state: AgentState) -> Action: ...

def point_is_free(point: Position, side: Team, timeout_ms: int = 100) -> bool:
    return segment_is_free(point, point, side, timeout_ms=timeout_ms)

__all__ = [
    "Action",
    "AgentProtocol",
    "AgentState",
    "DefenseBot",
    "Config",
    "GameState",
    "FlagState",
    "FlagStatus",
    "point_is_free",
    "run",
    "run_headless",
    "segment_is_free",
    "Team"
]