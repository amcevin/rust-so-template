
all: debug main

so: 
	cargo build --release

debug-so: 
	cargo build 

debug: so
	gcc -o debug test.c -L./target/debug -lrust_so_example -lpthread -ldl

main: so
	gcc -o main test.c -L./target/release -lrust_so_example -lpthread -ldl

clean:
	rm -rf debug main 