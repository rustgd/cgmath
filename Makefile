# adapted from https://github.com/z0w0/rusty-math/blob/master/Makefile

all:
	rustc src/om3d.rc --lib --out-dir=lib
test: all
	rustc --test -L lib test/*-test.rs -o test/build/test.elf
	./test/build/test.elf
clean:
	rm -R -f ./lib/*
	rm -R -f ./test/*.elf