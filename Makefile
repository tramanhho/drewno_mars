projectname = compilers_project1

all: 
	cargo build; cd target/debug; mv ${projectname} ../../dmc; cd ../.. 

clean: 
	rm dmc