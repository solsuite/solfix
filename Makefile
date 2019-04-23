clean:
	rm -rf ./bin
build:
	mkdir -p ./bin; flex -o ./bin/4.24.sol.c ./lexer/4.24.sol.l; clang++ ./bin/4.24.sol.c -o ./bin/lexer -ll
lexer: build
	./bin/lexer
