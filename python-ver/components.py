import enum
import ecs

class Color(enum.Enum):
    white = 0
    black = 1 # nothing
    yellow = 2 # sand
    blue = 3 # water
    gray = 4 # stone
    brown = 5 # dirt
    green = 6 # grass

colors = {
    Color.white: (255,255,255),
    Color.black: (0,0,0), # nothing
    Color.yellow: (255,255,0), # sand
    Color.blue: (0,0,255), # water
    Color.gray: (155,155,155), # stone
    Color.brown: (210,105,30), # dirt
    Color.green: (0,255,0) # grass
}

class Position(ecs.models.Component):
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
    def __eq__(self, other):
        if isinstance(other, Position):
            return self.x == other.x and self.y == other.y
        return False
    def __str__(self):
        return f"Position: {self.x}, {self.y}"
class Velocity(ecs.models.Component):
    def __init__(self, vx: int, vy: int, terminal_velocity: int):
        self.vx = vx
        self.vy = vy
        self.terminal_velocity = terminal_velocity
    def reset_velocity(self):
        self.vx = 0
        self.vy = 0
    def __str__(self):
        return f"Velocity: {self.vx}, {self.vy}"
class PixelType(ecs.models.Component):
    # will be used to determine what type of logic should be performed on the pixel in the future
    def __init__(self, color: tuple):
        self.color = color
    def __str__(self):
        return f"PixelType: {self.color}"