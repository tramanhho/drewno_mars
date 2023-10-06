projectname = drewno_mars
cov-options = --ignore-filename-regex='(/.cargo/registry)|(grammar.rs)|(fast_local.rs)' --use-color --Xdemangler=rustfilt ./target/debug/${projectname} -instr-profile=./tests/reports/test.profdata

all: 
	cargo build; cd target/debug; mv ${projectname} ../../dmc; cd ../.. 

cov-report: test.profdata
	llvm-cov report ${cov-options}

cov-show: test.profdata
	llvm-cov show ${cov-options}

test.profdata:
	RUSTFLAGS="-C instrument-coverage" cargo test --tests
	llvm-profdata merge -sparse default_*.profraw -o meow.profdata
	rm -f default_*.profraw
	mv meow.profdata ./tests/reports/test.profdata
#	PATH=$PATH:/home/t010h705/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
#	just. manually do it for now bc otherwise we have to export anyway... 
#	not worth and also every time you run it youd be adding to the path again . kinda messy....

clean: 
	rm -f dmc
	rm -f ./tests/reports/*.profdata
