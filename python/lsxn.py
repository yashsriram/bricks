import pygame, sys
from pygame.locals import *
import numpy as np
import numpy.typing as npt
import random
from dataclasses import dataclass, field


window_size = 720


def abc_from_2p(p, q):
    a = (q[1] - p[1])
    b = -(q[0] - p[0])
    c = p[0] * q[1] - q[0]* p[1]
    return (a, b, c)


def is_point_on_segment(q, p1, p2):
    is_x_projection_on_line_segment = min(p1[0], p2[0]) <= q[0] and q[0] <= max(p1[0], p2[0])
    is_y_projection_on_line_segment = min(p1[1], p2[1]) <= q[1] and q[1] <= max(p1[1], p2[1])
    return is_x_projection_on_line_segment and is_y_projection_on_line_segment


def get_xn(p1, p2, q1, q2):
    a1, b1, c1 = abc_from_2p(p1, p2)
    a2, b2, c2 = abc_from_2p(q1, q2)
    the_A = np.array([[a1, b1], [a2, b2]])
    the_b = np.array([c1, c2])
    the_xn = np.linalg.solve(the_A, the_b)
    is_on_both = is_point_on_segment(the_xn, p1, p2) and is_point_on_segment(the_xn, q1, q2)
    return (the_xn, is_on_both)


@dataclass
class LineSegmentsXn:
    p1: npt.NDArray[np.float64] = field(default_factory=lambda: np.random.rand(2) * window_size)
    p2: npt.NDArray[np.float64] = field(default_factory=lambda: np.random.rand(2) * window_size)
    q1: npt.NDArray[np.float64] = field(default_factory=lambda: np.random.rand(2) * window_size)
    q2: npt.NDArray[np.float64] = field(default_factory=lambda: np.random.rand(2) * window_size)

    xn: npt.NDArray[np.float64] = field(init=False)
    is_on_bth: bool = field(init=False)

    def __post_init__(self):
        self.xn, self.is_on_both = get_xn(self.p1, self.p2, self.q1, self.q2)

    def draw(self, screen, color):
        screen.fill("white" if self.is_on_both else "black")
        pygame.draw.circle(screen, color, list(self.p1), 10)
        pygame.draw.circle(screen, color, list(self.p2), 10)
        pygame.draw.line(screen, color, self.p1, self.p2)
        pygame.draw.circle(screen, color, list(self.q1), 10)
        pygame.draw.circle(screen, color, list(self.q2), 10)
        pygame.draw.line(screen, color, self.q1, self.q2)
        xn_radius = 10 if self.is_on_both else 20
        pygame.draw.circle(screen, color, self.xn, xn_radius)

if __name__ == "__main__":
    screen = pygame.display.set_mode((window_size, window_size))
    pygame.init()
    clock = pygame.time.Clock()
    while True:
        for event in pygame.event.get():
            if (
                event.type == pygame.KEYDOWN and
                event.key == pygame.K_SPACE
            ):
                color = (random.randint(0, 255), random.randint(0, 255), random.randint(0, 255))
                LineSegmentsXn().draw(screen, color)

        pygame.display.flip()
        pygame.display.update()
        clock.tick(60)
