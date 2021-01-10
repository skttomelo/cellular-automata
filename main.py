import pygame
from sys import exit

# we initialize pygame
pygame.init()

# screen
screen = pygame.display.set_mode((800,600))

pygame.display.set_caption("Save me bruv")

SCALE = 10

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
    if pygame.mouse.get_pressed()[2]:
            try:
                mouse_pos = pygame.mouse.get_pos()
                mouse_pos_scaled = (mouse_pos[0]//SCALE, mouse_pos[1]//SCALE)
                if pixels[mouse_pos_scaled[1]][mouse_pos_scaled[0]] == 0:
                    pixels[mouse_pos_scaled[1]][mouse_pos_scaled[0]] = 2
            except AttributeError:
                pass

'''
TODO: Make it to where every pixel on the screen only updates once in between frames
'''
def update():
    for y in range(600//SCALE):
        for x in range(800//SCALE):
            # we go through our array backwards that way we can see the effects live rather than everything being calculated before you see an update
            # better solution would be to give each pixel data to know if it has already changed so it won't update repeatedly until it's done before the frame is drawn

            # nothing
            if pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] == 0:
                continue
            try:
                up = pixels[(600//SCALE) - y - 2][(800//SCALE) - x - 1]
            except:
                up = -1
            try:
                down = pixels[(600//SCALE) - y][(800//SCALE) - x - 1]
            except:
                down = -1
            try:
                down_left = pixels[(600//SCALE) - y][(800//SCALE) - x - 2]
            except:
                down_left = -1
            try:
                down_right = pixels[(600//SCALE) - y][(800//SCALE) - x]
            except:
                down_right = -1
            try:
                left = pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 2]
            except:
                left = -1
            try:
                right = pixels[(600//SCALE) - y - 1][(800//SCALE) - x]
            except:
                right = -1
            # sand
            if pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] == 1:
                '''
                sand should flow like this (whether there is nothing or liquids):
                          #
                        / | \
                       <  V  >
                '''
                if down == 0:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                    pixels[(600//SCALE) - y][(800//SCALE) - x - 1] = 1
                elif down == 2:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 2
                    pixels[(600//SCALE) - y][(800//SCALE) - x - 1] = 1
                elif down_left == 0:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                    pixels[(600//SCALE) - y][(800//SCALE) - x - 2] = 1
                elif down_left == 2:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 2
                    pixels[(600//SCALE) - y][(800//SCALE) - x - 2] = 1
                elif down_right == 0:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                    pixels[(600//SCALE) - y][(800//SCALE) - x] = 1
                elif down_right == 2:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 2
                    pixels[(600//SCALE) - y][(800//SCALE) - x] = 1
            # water
            elif pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] == 2:
                '''
                water should move like this (when there is nothing):
                      < - # - >
                        / | \
                       <  V  >
                '''
                if down == 0:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                    pixels[(600//SCALE) - y][(800//SCALE) - x - 1] = 2
                elif down_left == 0:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                    pixels[(600//SCALE) - y][(800//SCALE) - x - 2] = 2
                elif down_right == 0:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                    pixels[(600//SCALE) - y][(800//SCALE) - x] = 2
                elif left == 0:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 2] = 2
                elif right == 0:
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x - 1] = 0
                    pixels[(600//SCALE) - y - 1][(800//SCALE) - x] = 2

def draw():
    for y in range(600//SCALE):
        for x in range(800//SCALE):
            rect = pygame.Rect(x*SCALE,y*SCALE,SCALE,SCALE)
            pygame.draw.rect(screen, colors[pixels[y][x]], rect)


clock = pygame.time.Clock()

# basic game loop
while True:
    events() # check inputs from user

    update() # updates the pixels on the screen

    screen.fill(white)

    draw()

    pygame.display.update()
    clock.tick(60)