myClass: class {
    myInt: perfect int;
    myBool: bool;
    new : (i : int, b : bool) void {
        myInt = i;
        myBool = b;
    }
};

main : () void {
    inst : myClass;
    inst--new(1, true);
    give inst--myInt; // should print 1?
}