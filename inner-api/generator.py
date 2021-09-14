import re
from PIL import Image, ImageDraw, ImageFont

class TextImageGenerator:
    def __init__(self, text, font, image_size=128, fg_color=(255, 0, 0, 255), bg_color=(255, 255, 255, 0), offset=0):
        self.font = font
        self.image_size = image_size
        self.offset = offset
        text = re.sub("_+", "_", text)
        self.texts = text.split("_")
        self.lines = len(self.texts)
        self.bg_color = bg_color
        self.fg_color = fg_color
        self.img = Image.new(
            "RGBA", (self.image_size, self.image_size), bg_color)

    def render(self):
        for i, text in enumerate(self.texts):
            fnt = ImageFont.truetype(self.font, self.image_size)
            sz = fnt.getsize(text)
            img = Image.new("RGBA", sz, self.bg_color)
            draw = ImageDraw.Draw(img)
            draw.text((0, -self.offset), text, font=fnt,
                      fill=self.fg_color, align='center')
            img = img.resize(
                (self.image_size, self.image_size // self.lines + self.offset // self.lines))
            self.img.paste(img, (0, i * (self.image_size // self.lines)))
        return self.img
