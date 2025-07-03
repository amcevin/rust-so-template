
.PHONY: go-client

os := $(shell uname -s)
arch := $(shell uname -m)
ifeq ($(os), Darwin)
	so_extension = dylib
else
	so_extension = so
endif

all: debug main

so: 
	cargo build --release

debug-so: 
	cargo build 

debug: debug-so
	gcc -o debug test.c -L./target/debug -lrust_so_example -lpthread -ldl

main: so
	gcc -o main test.c -L./target/release -lrust_so_example -lpthread -ldl

clean:
	rm -rf debug main 

header:
	cbindgen --lang c --output rust_so_example.h

go-client: header so
	cd go-client && go build -o go-client main.go
	./go-client/go-client

release: header 
	cargo build --release	
	mkdir -p release/${os}/${arch}
	cp target/release/librust_so_example.${so_extension} release/${os}/${arch}/
	cp rust_so_example.h release/${os}/${arch}/


install-so: so
	cp target/release/librust_so_example.${so_extension} /usr/local/lib/

remove-so:
	rm -f /usr/local/lib/librust_so_example.${so_extension}