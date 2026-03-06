import pygame, sys
from pygame.locals import *
import numpy as np
import random

pygame.init()

width = 1080
height = 1080
screen = pygame.display.set_mode((width, height))

clicked_points = []


current_color = (100, 100, 100)


def sample_color():
    global current_color
    current_color = (random.randint(0, 255), random.randint(0, 255), random.randint(0, 255))


def get_line_abc_form(p1, p2):
    a = (p2[1] - p1[1])
    b = -(p2[0] - p1[0])
    c = p1[0] * p2[1] - p2[0]* p1[1]
    return (a, b, c)


def is_on_line_segment(p1, p2, xn):
    is_on_line_segment_x_projection = min(p1[0], p2[0]) <= xn[0] and xn[0] <= max(p1[0], p2[0])
    is_on_line_segment_y_projection = min(p1[1], p2[1]) <= xn[1] and xn[1] <= max(p1[1], p2[1])
    return is_on_line_segment_x_projection and is_on_line_segment_y_projection


def get_line_segment_xn(p1, p2, q1, q2):
    a1, b1, c1 = get_line_abc_form(p1, p2)
    a2, b2, c2 = get_line_abc_form(q1, q2)
    the_A = np.array([[a1, b1], [a2, b2]])
    the_b = np.array([c1, c2])
    the_xn = np.linalg.solve(the_A, the_b)
    is_on_both = is_on_line_segment(p1, p2, the_xn) and is_on_line_segment(q1, q2, the_xn)
    return (the_xn, is_on_both)


while True:
  for event in pygame.event.get():
    if event.type==QUIT:
      pygame.quit()
      sys.exit()
    elif event.type == MOUSEBUTTONDOWN:
        # Point
        print(event.pos)
        pygame.draw.circle(screen, current_color, event.pos, 10)
        clicked_points.append(event.pos)
        # Line segment
        if len(clicked_points) % 2 == 0:
            pygame.draw.line(screen, current_color, clicked_points[-2], clicked_points[-1])
        # Intersection
        if len(clicked_points) % 4 == 0:
            xn, is_on_both = get_line_segment_xn(*clicked_points[-4:])
            print(xn)
            xn_radius = 10 if is_on_both else 20
            pygame.draw.circle(screen, current_color, xn, xn_radius)
            sample_color()

  pygame.display.update()
