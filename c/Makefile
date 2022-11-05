CC=gcc

move:
	$(CC) -o move move.c

run:
	$(CC) -o run run.c wowsunpack.c

wowsunpack:
	$(CC) -shared -o libwowsunpack.dll wowsunpack.c

wowsunpacker:
	cd ../wowsunpacker && cargo build --lib --release
	move /y "..\wowsunpacker\target\release\wowsunpacker.dll" .
	$(CC) -o wowsunpacker wowsunpacker.c wowsunpacker.dll

all: move run wowsunpack

clean:
	del *.dll *.exe *.json
