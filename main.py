import pygame
import ecs
import enum
import datetime
import math
from sys import exit

# cellular-automata imports
from components import Position, PixelType, Velocity, Color, colors
from systems import SandSystem, WaterSystem, DirtSystem, PositionSystem
from global_variables import WIDTH, HEIGHT, SCALE

# we initialize pygame
pygame.init()

# screen
screen = pygame.display.set_mode((WIDTH,HEIGHT))

pygame.display.set_caption("Save me bruv")

entity_manager = ecs.managers.EntityManager()
system_manager = ecs.managers.SystemManager(entity_manager)

# add systems to system manager
system_manager.add_system(SandSystem())
system_manager.add_system(WaterSystem())
system_manager.add_system(DirtSystem())
system_manager.add_system(PositionSystem())

# add all entities to entity_manager
for x in range(WIDTH//SCALE):
    for y in range(HEIGHT//SCALE):
        new_entity = entity_manager.create_entity()
        new_pos = Position(x,y)
        new_vel = Velocity(0,0,1)
        new_pixel_type = PixelType(Color.black)

        entity_manager.add_component(new_entity, new_pos)
        entity_manager.add_component(new_entity, new_vel)
        entity_manager.add_component(new_entity, new_pixel_type)

def events():
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            exit()
    # left mouse button pressed
    if pygame.mouse.get_pressed()[0]:
        mouse_pos = pygame.mouse.get_pos()
        pos = Position(mouse_pos[0]//SCALE, mouse_pos[1]//SCALE)
        for entity in entity_manager.pairs_for_type(Position):
            entity_pixel_type = entity_manager.component_for_entity(entity[0], PixelType)
            if entity[1].x == pos.x and entity[1].y == pos.y and entity_pixel_type.color == Color.black:
                entity_pixel_type.color = Color.yellow
                break


def draw():
    for entity in entity_manager.pairs_for_type(Position):
        pos = entity_manager.component_for_entity(entity[0], Position)
        rect = pygame.Rect(pos.x*SCALE,pos.y*SCALE,SCALE,SCALE)
        pygame.draw.rect(screen, colors[entity_manager.component_for_entity(entity[0], PixelType).color], rect)


clock = pygame.time.Clock()

# basic game loop
while True:
    events() # check inputs from user

    system_manager.update(datetime.datetime.now().timestamp()) # updates the pixels on the screen

    screen.fill(colors[Color.black])

    draw()

    pygame.display.update()
    clock.tick(60)