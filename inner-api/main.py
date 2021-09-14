import asyncio
import io
from typing import Optional
from enum import Enum
from fastapi import FastAPI
from fastapi.responses import StreamingResponse
from pydantic import BaseModel

from generator import TextImageGenerator
from converter import InvalidColorException, parse_color

class FontType(str, Enum):
    Mincho = "Mincho"
    Gothic = "Gothic"
    Maru = "Maru"
    BlackMincho = "BlackMincho"
    BlackGothic = "BlackGothic"

    def font_path(self):
        if self == FontType.Mincho:
            return "./resources/NotoSerifCJKjp-Bold.otf"
        elif self == FontType.Gothic:
            return "./resources/NotoSansCJKjp-Bold.otf"
        elif self == FontType.Maru:
            return "./resources/rounded-mplus-1p-black.ttf"
        elif self == FontType.BlackMincho:
            return "./resources/NotoSerifCJKjp-Black.otf"
        elif self == FontType.BlackGothic:
            return "./resources/NotoSansCJKjp-Black.otf"
        return "./resources/NotoSerifCJKjp-Bold.otf"

    def offset(self):
        if self == FontType.Mincho:
            return 36
        elif self == FontType.Gothic:
            return 36
        elif self == FontType.Maru:
            return 28
        elif self == FontType.BlackMincho:
            return 36
        elif self == FontType.BlackGothic:
            return 36
        return 36
    

app = FastAPI()


@app.get("/emoji")
async def create_emoji(
    text: str,
    font: FontType,
    size: Optional[int] = None,
    text_color: Optional[str] = None,
    background_color: Optional[str] = None,
):
    size = min(256, max(32, size or 128))
    try:
        fg_color = parse_color(text_color or "#FF0000")
        bg_color = parse_color(background_color or "transparent")
    except InvalidColorException:
        return {"message": "Invalid color"}, 400
    generator = TextImageGenerator(text, font.font_path(), size, fg_color, bg_color, font.offset())
    loop = asyncio.get_event_loop()
    img = await loop.run_in_executor(None, generator.render)
    buffer = io.BytesIO()
    img.save(buffer, format="PNG")
    buffer.seek(0)
    return StreamingResponse(buffer, media_type="image/png")
