all: bench

bench: *.rs ../src/*.rs
	(cd .. && cargo build)
	rustc -L ../target --test bench.rs

run: bench
	./bench --bench

clean:
	rm bench
