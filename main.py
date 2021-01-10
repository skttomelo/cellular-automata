import pygame
from sys import exit

# we initialize pygame
pygame.init()

# screen
screen = pygame.display.set_mode((800,600))

pygame.display.set_caption("Save me bruv")

SCALE = 50

clicked_position = [-1,-1]

white = (255,255,255)
black = (0,0,0)

colors = {
    0: (0,0,0),
    1: (255,255,0),
    2: (0,0,255)
}

pixels = [[]] # pixels will be 0 = Nothing, 1 = Sand, or 2 = Water

for y in range(600//SCALE):
    for x in range(800//SCALE):
        pixels[y].append(0)
    pixels.append([])

def events():
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            exit()
        if pygame.mouse.get_pressed()[0]:
            try:
                mouse_pos = pygame.mouse.get_pos()
                mouse_pos_scaled = (mouse_pos[0]//SCALE, mouse_pos[1]//SCALE)
                if pixels[mouse_pos_scaled[1]][mouse_pos_scaled[0]] == 0:
                    pixels[mouse_pos_scaled[1]][mouse_pos_scaled[0]] = 1
            except AttributeError:
                pass

def update():
    for y in range(600//SCALE):
        for x in range(800//SCALE):
            # nothing
            if pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] == 0:
                continue
            # sand
            elif pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] == 1:
                # we need to check if there is anything below it or beside it, and move it
                try:
                    if pixels[(600//SCALE) - y][(800//SCALE) - x - 1] == 0:
                        pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                        pixels[(600//SCALE) - y][(800//SCALE) - x - 1] = 1
                    # there is something below it so we check the bottom left if there is nothing
                    elif pixels[(600//SCALE) - y][(800//SCALE) - x - 2] == 0:
                        pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                        pixels[(600//SCALE) - y][(800//SCALE) - x - 2] = 1
                    elif pixels[(600//SCALE) - y][(800//SCALE) - x] == 0:
                        pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                        pixels[(600//SCALE) - y][(800//SCALE) - x] = 1
                except:
                    pass

def draw():
    for y in range(600//SCALE):
        for x in range(800//SCALE):
            rect = pygame.Rect(x*SCALE,y*SCALE,SCALE,SCALE)
            pygame.draw.rect(screen, colors[pixels[y][x]], rect)


clock = pygame.time.Clock()

# basic game loop
while True:
    events() # check inputs from user

    update()

    screen.fill(white)

    draw()

    pygame.display.update()
    clock.tick(60)