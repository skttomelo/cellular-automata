import pygame
import ecs
import enum
import datetime
import math
from sys import exit

# we initialize pygame
pygame.init()

# screen
screen = pygame.display.set_mode((800,600))

pygame.display.set_caption("Save me bruv")

SCALE = 10

entity_manager = ecs.managers.EntityManager()
system_manager = ecs.managers.SystemManager(entity_manager)

class Color(enum.Enum):
    white = 0
    black = 1 # nothing
    yellow = 2 # sand
    blue = 3 # water

colors = {
    Color.white: (255,255,255),
    Color.black: (0,0,0), # nothing
    Color.yellow: (255,255,0), # sand
    Color.blue: (0,0,255), # water
}

class Position(ecs.models.Component):
    def __init__(self, x: int, y: int):
        super().__init__()
        self.x = x
        self.y = y
class PixelType(ecs.models.Component):
    # will be used to determine what type of logic should be performed on the pixel in the future
    def __init__(self, color: tuple):
        super().__init__()
        self.color = color

# sand system
class SandSystem(ecs.models.System):
    def update(self, dt):
        # get all entities in database that have PixelType
        entity_list = self.entity_manager.pairs_for_type(PixelType)
        
        # loop through each entity checking to see if it has the color yellow
        # after that we then look for entities within 1 pixel of it respectfully
        # aka down-left | down | down-right
        for entity in entity_list:
            if self.entity_manager.component_for_entity(entity[0], PixelType).color != Color.yellow:
                continue
            entity_pos = self.entity_manager.component_for_entity(entity[0], Position)
            
            # search for the other entities close to it
            close_entities = {}
            for other_entity in entity_list:
                if other_entity[0] == entity[0]:
                    continue
                pos = self.entity_manager.component_for_entity(other_entity[0], Position)
                if math.dist([entity_pos.x,entity_pos.y],[pos.x,pos.y]) == 1:
                    close_entities[(pos.x,pos.y)] = other_entity[0] # we use the position of the entity as the key so it is easier to do logic checks later
            
            
            # sanity checks
            if entity_pos.y + 1 >= 600//SCALE: # unknown if I will ever implement gravity that is greater than 1
                continue
            if entity_pos.x + 1 == 800//SCALE:
                continue
            if entity_pos.x + 1 < 0:
                continue

            # now we need to figure out if the entity will move down, down to the side, or not at all    
            if (entity_pos.x, entity_pos.y+1) not in close_entities:
                entity_pos.y += 1
            elif (entity_pos.x-1, entity_pos.y+1) not in close_entities:
                entity_pos.y += 1
                entity_pos.y -= 1
            elif (entity_pos.x+1, entity_pos.y+1) not in close_entities:
                entity_pos.y += 1
                entity_pos.y += 1

# add SandSystem to system manager
system_manager.add_system(SandSystem())
        
        

# water system (TODO)

# create all pixels in relation to scale
# actually I don't think I need this block of code simply because accounting for black pixels is going to tank performance...
# pixels = [[]] # pixels will be 0 = Nothing, 1 = Sand, or 2 = Water
# for x in range(800//SCALE):
#     for y in range(600//SCALE):
#         entity = entity_manager.create_entity()
#         entity_manager.add_component(entity, Position(x,y))
#         entity_manager.add_component(entity, PixelType(Color.black))

def events():
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            exit()
    # left mouse button pressed
    if pygame.mouse.get_pressed()[0]:
        try:
            new_entity = entity_manager.create_entity()
            mouse_pos = pygame.mouse.get_pos()
            entity_manager.add_component(new_entity, Position(mouse_pos[0]//SCALE, mouse_pos[1]//SCALE))
            entity_manager.add_component(new_entity, PixelType(Color.yellow))
        except AttributeError:
            pass
    # right mouse button pressed
    # if pygame.mouse.get_pressed()[2]:
    #         try:
    #             mouse_pos = pygame.mouse.get_pos()
    #             mouse_pos_scaled = (mouse_pos[0]//SCALE, mouse_pos[1]//SCALE)
    #             if pixels[mouse_pos_scaled[1]][mouse_pos_scaled[0]] == 0:
    #                 pixels[mouse_pos_scaled[1]][mouse_pos_scaled[0]] = 2
    #         except AttributeError:
    #             pass

'''
TODO: Make it to where every pixel on the screen only updates once in between frames
'''
def update():
    system_manager.update(datetime.datetime.now().timestamp())
    # for y in range(600//SCALE):
    #     for x in range(800//SCALE):
    #         # we go through our array backwards that way we can see the effects live rather than everything being calculated before you see an update
    #         # better solution would be to give each pixel data to know if it has already changed so it won't update repeatedly until it's done before the frame is drawn

    #         # nothing
    #         if pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] == 0:
    #             continue
    #         try:
    #             up = pixels[(600//SCALE) - y - 2][(800//SCALE) - x - 1]
    #         except:
    #             up = -1
    #         try:
    #             down = pixels[(600//SCALE) - y][(800//SCALE) - x - 1]
    #         except:
    #             down = -1
    #         try:
    #             down_left = pixels[(600//SCALE) - y][(800//SCALE) - x - 2]
    #         except:
    #             down_left = -1
    #         try:
    #             down_right = pixels[(600//SCALE) - y][(800//SCALE) - x]
    #         except:
    #             down_right = -1
    #         try:
    #             left = pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 2]
    #         except:
    #             left = -1
    #         try:
    #             right = pixels[(600//SCALE) - y - 1][(800//SCALE) - x]
    #         except:
    #             right = -1
    #         # sand
    #         if pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] == 1:
    #             '''
    #             sand should flow like this (whether there is nothing or liquids):
    #                       #
    #                     / | \
    #                    <  V  >
    #             '''
    #             if down == 0:
    #                 pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
    #                 pixels[(600//SCALE) - y][(800//SCALE) - x - 1] = 1
    #             # elif down == 2:
    #             #     pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 2
    #             #     pixels[(600//SCALE) - y][(800//SCALE) - x - 1] = 1
    #             elif down_left == 0:
    #                 pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
    #                 pixels[(600//SCALE) - y][(800//SCALE) - x - 2] = 1
    #             # elif down_left == 2:
    #             #     pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 2
    #             #     pixels[(600//SCALE) - y][(800//SCALE) - x - 2] = 1
    #             elif down_right == 0:
    #                 pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
    #                 pixels[(600//SCALE) - y][(800//SCALE) - x] = 1
    #             # elif down_right == 2:
    #             #     pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 2
    #             #     pixels[(600//SCALE) - y][(800//SCALE) - x] = 1
    #         # water
    #         elif pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] == 2:
    #             '''
    #             water should move like this (when there is nothing):
    #                   < - # - >
    #                     / | \
    #                    <  V  >
    #             '''
    #             if down == 0:
    #                 pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
    #                 pixels[(600//SCALE) - y][(800//SCALE) - x - 1] = 2
    #             elif down_left == 0:
    #                 pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
    #                 pixels[(600//SCALE) - y][(800//SCALE) - x - 2] = 2
    #             elif down_right == 0:
    #                 pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
    #                 pixels[(600//SCALE) - y][(800//SCALE) - x] = 2
    #             elif left == 0:
    #                 pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
    #                 pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 2] = 2
    #             elif right == 0:
    #                 pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
    #                 pixels[(600//SCALE) - y - 1][(800//SCALE) - x] = 2

def draw():
    for entity in entity_manager.pairs_for_type(Position):
        pos = entity_manager.component_for_entity(entity[0], Position)
        rect = pygame.Rect(pos.x*SCALE,pos.y*SCALE,SCALE,SCALE)
        pygame.draw.rect(screen, colors[entity_manager.component_for_entity(entity[0], PixelType).color], rect)


clock = pygame.time.Clock()

# basic game loop
while True:
    events() # check inputs from user

    update() # updates the pixels on the screen

    screen.fill(colors[Color.black])

    draw()

    pygame.display.update()
    clock.tick(60)