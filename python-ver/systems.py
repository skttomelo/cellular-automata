import ecs
import math

# cellular-automata imports
from components import Position, PixelType, Velocity, Color
from global_variables import WIDTH, HEIGHT, SCALE

# sand system
class SandSystem(ecs.models.System):
    def update(self, dt):
        # get all entities in database that have PixelType
        entity_list = self.entity_manager.pairs_for_type(PixelType)
        
        # loop through each entity checking to see if it has the color yellow
        # after that we then look for entities within 1 pixel of it respectfully
        # aka down-left | down | down-right
        # entity = (Entity, Color)
        for entity in entity_list:
            if entity[1].color != Color.yellow:
                continue
            entity_pos = self.entity_manager.component_for_entity(entity[0], Position)
            entity_vel = self.entity_manager.component_for_entity(entity[0], Velocity)

            # sanity checks
            if entity_pos.y + 1 >= HEIGHT//SCALE: # unknown if I will ever implement gravity that is greater than 1
                entity_vel.vx = 0
                entity_vel.vy = 0
                continue
            if entity_pos.x + 1 == WIDTH//SCALE:
                entity_vel.vx = 0
                entity_vel.vy = 0
                continue
            if entity_pos.x + 1 < 0:
                entity_vel.vx = 0
                entity_vel.vy = 0
                continue

            # now we need to figure out if the entity will move down, down to the side, or not at all
            directions = [False, False, False] # indexes - 0 = down, 1 = down-left, 2 = down-right (if true then that direction is blocked)
            presence_of_liquid = []
            '''
            sand should flow like this (whether there is nothing or liquids):
                      #
                    / | \
                   <  V  >
            '''
            new_list = self.entity_manager.pairs_for_type(Position)
            positions_to_check = []
            for other_entity in new_list:
                distance = math.dist([entity_pos.x, entity_pos.y], [other_entity[1].x, other_entity[1].y])
                if other_entity[0] == entity[0] or distance < 1.0 or distance > 1.5:
                    continue

                if entity_pos.x == other_entity[1].x and entity_pos.y+1 == other_entity[1].y and self.entity_manager.component_for_entity(other_entity[0], PixelType).color != Color.black:
                    directions[0] = True
                if entity_pos.x-1 == other_entity[1].x and entity_pos.y+1 == other_entity[1].y:
                    pass
            if directions[0] == False:
                entity_vel.vx = 0
                entity_vel.vy = entity_vel.terminal_velocity
            else:
                entity_vel.vx = 0
                entity_vel.vy = 0

# water system
class WaterSystem(ecs.models.System):
    def update(self, dt):
        # get all entities in database that have PixelType
        entity_list = self.entity_manager.pairs_for_type(PixelType)
        
        # loop through each entity checking to see if it has the color yellow
        # after that we then look for entities within 1 pixel of it respectfully
        # aka down-left | down | down-right | left | right
        for entity in entity_list:
            if entity[1].color != Color.blue:
                continue
            entity_pos = self.entity_manager.component_for_entity(entity[0], Position)
            entity_vel = self.entity_manager.component_for_entity(entity[0], Velocity)

            # sanity checks
            if entity_pos.y + 1 >= 600//SCALE: # unknown if I will ever implement gravity that is greater than 1
                continue
            if entity_pos.x + 1 == 800//SCALE:
                continue
            if entity_pos.x + 1 < 0:
                continue

            # now we need to figure out if the entity will move down, down to the side, or not at all
            directions = [False, False, False, False, False] # indexes - 0 = down, 1 = down-left, 2 = down-right (if true then that direction is blocked)
            '''
                water should move like this (when there is nothing):
                    < - # - >
                      / | \
                     <  V  >
            '''
            new_list = self.entity_manager.pairs_for_type(Position)
            for other_entity in new_list:
                pos = self.entity_manager.component_for_entity(other_entity[0], Position)
                if other_entity[0] == entity[0]:
                    continue

                if entity_pos.x == pos.x and entity_pos.y+1 == pos.y:
                    directions[0] = True
                elif entity_pos.x-1 == pos.x and entity_pos.y+1 == pos.y:
                    directions[1] = True
                elif entity_pos.x+1 == pos.x and entity_pos.y+1 == pos.y:
                    directions[2] = True
                elif entity_pos.x-1 == pos.x and entity_pos.y == pos.y:
                    directions[3] = True
                elif entity_pos.x+1 == pos.x and entity_pos.y == pos.y:
                    directions[4] = True
            
            if directions[0] == False:
                entity_pos.y += 1
            elif directions[1] == False:
                entity_pos.y += 1
                entity_pos.x -= 1
            elif directions[2] == False:
                entity_pos.y += 1
                entity_pos.x += 1
            elif directions[3] == False:
                entity_pos.x -= 1
            elif directions[4] == False:
                entity_pos.x += 1

class DirtSystem(ecs.models.System):
    def update(self, dt):
        # get all entities in database that have PixelType
        entity_list = self.entity_manager.pairs_for_type(PixelType)
        
        # loop through each entity checking to see if it has the color yellow
        # after that we then look for entities within 1 pixel of it respectfully
        # aka down-left | down | down-right
        for entity in entity_list:
            if entity[1].color != Color.brown:
                continue
            entity_pos = self.entity_manager.component_for_entity(entity[0], Position)

            # sanity checks
            if entity_pos.y + 1 >= 600//SCALE: # unknown if I will ever implement gravity that is greater than 1
                continue
            if entity_pos.x + 1 == 800//SCALE:
                continue
            if entity_pos.x + 1 < 0:
                continue

            # now we need to figure out if the entity will move down, down to the side, or not at all
            directions = [False, False, False] # indexes - 0 = down, 1 = down-left, 2 = down-right (if true then that direction is blocked)
            presence_of_liquid = []
            become_grass = False
            '''
            dirt should flow like this (whether there is nothing or liquids):
                      #
                    / | \
                   <  V  >
            '''
            new_list = self.entity_manager.pairs_for_type(Position)
            for other_entity in new_list:
                pos = self.entity_manager.component_for_entity(other_entity[0], Position)
                if other_entity[0] == entity[0]:
                    continue
                
                if entity_pos.x == pos.x and entity_pos.y+1 == pos.y:
                    directions[0] = True
                elif entity_pos.x-1 == pos.x and entity_pos.y+1 == pos.y:
                    directions[1] = True
                elif entity_pos.x+1 == pos.x and entity_pos.y+1 == pos.y:
                    directions[2] = True
                elif directions[0] == True and directions[1] == True and directions[2] == True and entity_pos.x == pos.x and entity_pos.y-1 == pos.y:
                    become_grass = True
                else:
                    become_grass = False

            if become_grass == True:
                entity[1].color = Color.green

            if directions[0] == False:
                # handling liquid (todo)
                
                entity_pos.y += 1
            elif directions[1] == False:
                entity_pos.y += 1
                entity_pos.x -= 1
            elif directions[2] == False:
                entity_pos.y += 1
                entity_pos.x += 1

# PositionSystem is used to update entity location based off their current velocity
class PositionSystem(ecs.models.System):
    def update(self, dt):
        entity_component_list = self.entity_manager.pairs_for_type(Velocity)
        other_entity_component_list = self.entity_manager.pairs_for_type(Position)

        # entity_component = (Entity, Velocity)
        for entity_component in entity_component_list:
            entity_pos = self.entity_manager.component_for_entity(entity_component[0], Position)
            
            for other_entity_component in other_entity_component_list:
                if other_entity_component[0] == entity_component[0]:
                    continue
                if entity_component[1].vx + entity_pos.x == other_entity_component[1].x and entity_component[1].vy + entity_pos.y == other_entity_component[1].y:
                    # swap PixelTypes and reset velocities
                    temp_pixel_type_color = entity_manager.component_for_entity(entity_component[0], PixelType).color
                    entity_manager.component_for_entity(entity_component[0], PixelType).color = entity_manager.component_for_entity(other_entity_component[0], PixelType).color
                    entity_manager.component_for_entity(other_entity_component[0], PixelType).color = temp_pixel_type_color
                    break
            entity_component[1].reset_velocity()