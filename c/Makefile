CC=gcc

move:
	$(CC) -o move move.c

run:
	$(CC) -o run run.c paramsunpack.c

paramsunpack:
	$(CC) -shared -o libwowsunpack.dll paramsunpack.c

wowsunpacker:
	cd ../wowsunpacker && cargo build --lib --release
	move /y "..\wowsunpacker\target\release\wowsunpacker.dll" .
	$(CC) -o wowsunpacker wowsunpacker.c wowsunpacker.dll

all: move run paramsunpack

clean:
	del *.dll *.exe *.json
