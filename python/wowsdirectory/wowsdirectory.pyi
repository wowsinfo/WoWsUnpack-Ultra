from typing import List


class GameDirectory:
    GAME_SERVER_WW: int
    GAME_SERVER_CN: int
    GAME_SERVER_PT: int
    GAME_SERVERS: List[int]

    def __init__(self) -> None: ...
    def locate(self) -> None: ...
    def get_game_directory(self, server: int) -> str: ...
