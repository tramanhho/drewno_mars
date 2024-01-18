//Applying an invalid operand to an assignment (either as the target or source). e.g., "f = g;", where f and/or g are function names.

gets_op_f1 : (  ) bool { return true; }
gets_op_f2 : (  ) bool { return true; }
gets_op_main : (  ) int { 
    gets_op_f1 = gets_op_f2;
}

gets4_op_f1 : (  ) bool { return true; }
gets4_op_main : (  ) int { 
    gets4_op_v1 : int = 1;
    gets4_op_v1 = gets4_op_f1;
}

gets5_op_f1 : (  ) bool { return true; }
gets5_op_main : (  ) int { 
    gets5_op_v1 : int = 1;
    gets5_op_f1 = gets5_op_v1;
}

//Applying an assignment to two valid but incompatible types. e.g., "f = g;", where f is a bool and g is an int.

gets_type_main : (  ) int { 
    gets_type_v1 : int = 1;
    gets_type_v2 : bool = true;
    gets_type_v2 = gets_type_v1;
}

gets2_type_main : (  ) int { 
    gets2_type_v1 : int = 1;
    gets2_type_v2 : bool = true;
    gets2_type_v1 = gets2_type_v2;
}

//Applying an assignment where the LHS is not an lval. e.g., b = 1; where b is declared perfect

lval_main : (  ) int { 
    lval_v1 : perfect int = 1;
    lval_v1 = 1;
}
