# drewno_mars

Drew, 

To run this project, you do not need a makefile. You do, however, need to install both Rust and `cargo`, the rust package manager. Instructions are found [here](https://www.rust-lang.org/learn/get-started).

After installing Rust and `cargo`, `cd` within the project directory. **Note**, this is not within the `src` folder, it is one level above it. If you `ls`, you should see something like this: 

```shell
$ ls
Cargo.lock  Cargo.toml  README.md  src  target  TEAM.md
```

To both compile and run the program, enter the following command: 
```shell
$ cargo run --quiet <infile.dm> -t <tokens.txt> 2> <errors.txt>
```

Here is an example input: 
```shell
$ cargo run --quiet ./src/io/input2.dm -t ./src/io/tokens.txt 2>./src/io/errors.txt
```