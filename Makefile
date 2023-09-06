projectname = drewno_mars

all: 
	cargo build; cd target/debug; mv ${projectname} ../../dmc; cd ../.. 

clean: 
	rm dmc
