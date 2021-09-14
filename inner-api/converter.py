import re

color = re.compile("(?:#|0x)?([0-9a-fA-F]{3,8})")

class InvalidColorException(Exception):
    pass

def parse_color(argument: str):
    with open("./resources/palette.txt") as f:
        lines = f.readlines()
    colors = {col.replace(" ", "-").replace("\n", "").strip("-"): code.replace("\n", "") for col,
                code in zip(lines[::2], lines[1::2])}
    if argument in colors.keys():
        argument = colors[argument]
    if color.match(argument):
        c = color.findall(argument)[0]
        a = 255
        if len(c) in [3, 4]:
            r = int(c[0] * 2, 16)
            g = int(c[1] * 2, 16)
            b = int(c[2] * 2, 16)
            if len(c) == 4:
                a = int(c[3] * 2, 16)
        elif len(c) in [6, 8]:
            r = int(c[0:2], 16)
            g = int(c[2:4], 16)
            b = int(c[4:6], 16)
            if len(c) == 8:
                a = int(c[6:8], 16)
        else:
            raise InvalidColorException(
                f"Invalid Color code: {argument}")
        return (r, g, b, a)
    else:
        raise InvalidColorException(
            f"Invalid Color code: {argument}")