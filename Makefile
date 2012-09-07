# adapted from https://github.com/z0w0/rusty-math/blob/master/Makefile

all:
	rustc src/om3d.rc --lib --out-dir=lib
test: all
	rustc --test -L lib test/test_om3d.rc -o test/build/test_om3d.elf
	./test/build/test_om3d.elf
clean:
	rm -R -f ./lib/*
	rm -R -f ./test/build/*