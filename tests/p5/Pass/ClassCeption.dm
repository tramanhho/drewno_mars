inner : class {
    i_int: perfect int;
    i_bool: bool;
    // new : (i : int, b : bool) void {
    //     i_int = i;
    //     i_bool = b;
    // }
};

outer : class {
    o_int: int;
    o_class: inner;
};

outer_outer : class {
    o_o_class: outer;
    o_o_bool: bool;
};

main : () void {
    inst : outer_outer;
    inst--o_o_class--o_class--i_bool = false;
}