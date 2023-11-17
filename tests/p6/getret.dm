meow : () int { return 1; }

main : () int {
    meow();
	c : int;
	c = meow();
	return meow();
}