from wowsdirectory import GameDirectory

if __name__ == '__main__':
    wows_dir = GameDirectory()
    wows_dir.locate()
    game_dir = wows_dir.get_game_directory(GameDirectory.GAME_SERVER_WW)
    for server in GameDirectory.GAME_SERVERS:
        print(wows_dir.get_game_directory(server))

    # test doc
    print(GameDirectory.__doc__)
    print(GameDirectory.locate.__doc__)
