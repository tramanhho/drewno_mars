inner : class {
    i_field: int;
};

outer : class {
    o_field: inner;
};

main : () void {
    inst : outer;
    inst--o_field--i_field = true;
}