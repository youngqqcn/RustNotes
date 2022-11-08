lib:
	rustc --crate-type=lib rary.rs

main:lib
	rustc main.rs --extern rary=library.rlib --edition=2021

clean:
	@rm -f main
	@rm -f library.rlib