from ._core import run, run_headless, segment_is_free, Action, AgentState, FlagState, FlagStatus, GameState, Team, Config, DefenseBot
from typing import Optional, Protocol, runtime_checkable
import sys
import matplotlib.pyplot as plt
import multiprocessing as mp

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

def plot(nodes: list, goal_positions: Optional[list[Position]] = None):
    fig, ax = plt.subplots()
    ax.set_aspect('equal', 'box')
    ax.set_xlim(-60, 60)
    ax.set_ylim(-60, 60)
    ax.grid(True, which='both', linestyle='--', linewidth=0.5)
    ax.invert_yaxis()

    outer = [
        ((-50.0, 50.0), (50.0, 50.0)),
        ((50.0, 50.0), (50.0, -50.0)),
        ((50.0, -50.0), (-50.0, -50.0)),
        ((-50.0, -50.0), (-50.0, 50.0)),
    ];

    side_bars = [
        ((-45.0, 45.0), (-45.0, 5.0)),
        ((-45.0, -5.0), (-45.0, -45.0)),
        ((45.0, 45.0), (45.0, 5.0)),
        ((45.0, -5.0), (45.0, -45.0)),
    ];

    middle = [
        ((-10.0, 5.0), (10.0, 5.0)),
        ((-10.0, -5.0), (10.0, -5.0)),
    ];

    diamond_left_edges = [
        ((-5.0, 30.0), (-35.0, 0.0)),
        ((-35.0, 0.0), (-5.0, -30.0)),
        ((-5.0, -30.0), (25.0, 0.0)),
        ((25.0, 0.0), (5.0, 20.0)),
    ];

    diamond_right_edges = [
        ((5.0, 30.0), (35.0, 0.0)),
        ((35.0, 0.0), (5.0, -30.0)),
        ((5.0, 30.0), (-25.0, 0.0)),
        ((-25.0, 0.0), (-5.0, -20.0)),
    ];

    for wall in outer + side_bars + middle + diamond_left_edges + diamond_right_edges:
        (x1, y1), (x2, y2) = wall
        ax.plot([x1, x2], [y1, y2], color='black', linewidth=1.0)

    for node in nodes:
        if node.parent is not None:
            ax.plot([node.position[0], node.parent.position[0]], [node.position[1], node.parent.position[1]], color='blue', linewidth=0.5, marker='o', markersize=2)
        else:
            ax.plot([node.position[0]], [node.position[1]], color='blue', linewidth=0.5, marker='o', markersize=2)

    if goal_positions is not None:
        x_positions = [goal[0] for goal in goal_positions]
        y_positions = [goal[1] for goal in goal_positions]
        ax.scatter(x_positions, y_positions, color='red', marker='x', s=64, label='Goals')

    plt.show()

def visualize_planner(planner, goal_positions: Optional[list[Position]] = None):
    nodes = planner.nodes
    process = mp.Process(target=plot, args=(nodes, goal_positions))
    process.start()

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