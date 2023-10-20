// multiply declared identifier 
multiple : int = 3;
multiple : int = 3;
multiple2 : int;
multiple2 : int;

// void type in declaration
invalid_type : void; 

// undeclared identifier
undeclared_fn : () void {
    if (undeclared < 3) {
        give "";
        inner_scope : int = 0;
        if (true) {
            give inner_scope;
        }
    }
    // out of scope 
    give inner_scope;
}


