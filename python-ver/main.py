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

def events():
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            exit()
    # left mouse button pressed
    if pygame.mouse.get_pressed()[0]:
        mouse_pos = pygame.mouse.get_pos()
        pos = Position(mouse_pos[0]//SCALE, mouse_pos[1]//SCALE)
        empty = True
        for entity in entity_manager.pairs_for_type(Position):
            if entity[1].x == pos.x and entity[1].y == pos.y:
                empty = False
                break
        if empty == True:
            new_entity = entity_manager.create_entity()
            vel = Velocity(0,0,1)
            pixel_type = PixelType(Color.yellow)

            entity_manager.add_component(new_entity, pos)
            entity_manager.add_component(new_entity, vel)
            entity_manager.add_component(new_entity, pixel_type)
    if pygame.mouse.get_pressed()[2]:
        mouse_pos = pygame.mouse.get_pos()
        pos = Position(mouse_pos[0]//SCALE, mouse_pos[1]//SCALE)
        empty = True
        for entity in entity_manager.pairs_for_type(Position):
            if entity[1].x == pos.x and entity[1].y == pos.y:
                empty = False
                break
        if empty == True:
            new_entity = entity_manager.create_entity()
            vel = Velocity(0,0,1)
            pixel_type = PixelType(Color.blue)

            entity_manager.add_component(new_entity, pos)
            entity_manager.add_component(new_entity, vel)
            entity_manager.add_component(new_entity, pixel_type)
    if pygame.mouse.get_pressed()[1]:
        mouse_pos = pygame.mouse.get_pos()
        pos = Position(mouse_pos[0]//SCALE, mouse_pos[1]//SCALE)
        empty = True
        for entity in entity_manager.pairs_for_type(Position):
            if entity[1].x == pos.x and entity[1].y == pos.y:
                empty = False
                break
        if empty == True:
            new_entity = entity_manager.create_entity()
            vel = Velocity(0,0,1)
            pixel_type = PixelType(Color.brown)

            entity_manager.add_component(new_entity, pos)
            entity_manager.add_component(new_entity, vel)
            entity_manager.add_component(new_entity, pixel_type)


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