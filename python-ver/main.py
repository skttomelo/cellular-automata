import pygame
import block_data
from pygame.locals import *
from sys import exit

# init
pygame.init()

white = (255,255,255)
black = (0,0,0)
sand_color = (255,255,224)

# initialize list of blocks
class Entity:
    def __init__(self, x: int, y: int, type: int):
        self.x = x
        self.y = y
        self.gravity = 1
        self.type = 0 # 0 = empty, 1 = sand, 2 = water

blocks = []

# screen
screen = pygame.display.set_mode((800,600))
for x in range(800):
    for y in range(600):
    blocks.append(Entity(x,y,0))

pygame.display.set_caption("cellular automata physics")

def events():
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            exit()

def update():
    for x in range(800):
        for y in range(600):
            # nothing
            if blocks[x*y].type == 0:
                continue

def draw():
    for x in range(800):
        for y in range(600):
            # nothing
            if blocks[x*y].type == 0:
                continue



clock = pygame.time.Clock()


# basic game loop
while True:
    events() # check inputs from user

    update()

    screen.fill(black)

    draw()

    pygame.display.update()
    clock.tick(60)