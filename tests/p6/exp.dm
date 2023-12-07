main : () void {
    b : bool;
	b = !(true or false);
	b = 24Kmagic;
	give b;

	i : int;
	i = -(1 + 3);
	give i;

	i = -(1 - 3);
	i = -(1 * 3);
	i = -(1 / 3);

	b = (1 == 3);
	b = (1 != 3);
	b = (1 < 3);
	b = (1 <= 3);
	b = (1 > 3);
	b = (1 >= 3);
}