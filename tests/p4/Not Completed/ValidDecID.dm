//decl in differnt scopes

//global 
decl1 : int = 1;

//fn
decl2fn : (a : int, b: bool ) void { 
    decl2 : int = 2;
}

//global dec, fn assignment
decl3 : int;
decl3fn : (  ) void { 
    decl3 = 3;
    decl2fn ( 3, true);
}


