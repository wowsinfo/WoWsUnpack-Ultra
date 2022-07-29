CC=gcc

move:
	$(CC) -o move move.c

run:
	$(CC) -o run run.c wowsunpack.c

wowsunpack:
	$(CC) -shared -o libwowsunpack.dll wowsunpack.c

all: move run wowsunpack

clean:
	del *.dll *.exe *.json
