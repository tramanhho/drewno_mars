//Applying an arithmetic operator (+, -, *, /, ++, or --) to an invalid type (e.g. a basic bool type, a function).

math_main : (  ) int { 
    math_v1 : int = 1;
    math_v2 : bool = true;
    return math_v1 + math_v2; 
}

math2_main : (  ) int { 
    math2_v1 : bool = true;
    math2_v2 : int = 1;
    return math2_v1 + math2_v2; 
}

math3_main : (  ) int { 
    math3_v1 : bool = true;
    math3_v2 : bool = true;
    return math3_v1 + math3_v2; 
}

math4_main : (  ) int { 
    math4_v1 : bool = true;
    math4_v2 : bool = true;
    return math4_v1 - math4_v2; 
}

math5_main : (  ) int { 
    math5_v1 : bool = true;
    math5_v2 : bool = true;
    return math5_v1 * math5_v2; 
}

math6_main : (  ) int { 
    math6_v1 : bool = true;
    math6_v2 : bool = true;
    return math6_v1 / math6_v2; 
}

math7_main : (  ) int { 
    math7_v1 : bool = true;
    math7_v1++; 
}

math8_main : (  ) int { 
    math8_v1 : bool = true;
    math8_v1--; 
}

math9_main : (  ) int { 
    math9_v1 : int = 1;
    return math9_v1 + true; 
}

math10_f1 : (  ) bool { return true; }
math10_main : (  ) int { 
    math10_v1 : int = 1;
    return math10_v1 + math10_f1(); 
}

math11_main : (  ) int { 
    math11_v1 : bool = true;
    return -math11_v1; 
}


//An invalid operand given to a relational operator (<, >, <=, >=).

cmp_main : (  ) bool { 
    cmp_v1 : int = 1;
    cmp_v2 : bool = true;
    return cmp_v1 < cmp_v2; 
}

cmp2_main : (  ) bool { 
    cmp2_v1 : bool = false;
    cmp2_v2 : bool = true;
    return cmp2_v1 < cmp2_v2; 
}

cmp3_main : (  ) bool { 
    cmp3_v1 : int = 1;
    cmp3_v2 : bool = true;
    return cmp3_v2 < cmp3_v1; 
}

cmp4_main : (  ) bool { 
    cmp4_v1 : int = 1;
    cmp4_v2 : bool = true;
    return cmp4_v1 > cmp4_v2; 
}

cmp5_main : (  ) bool { 
    cmp5_v1 : int = 1;
    cmp5_v2 : bool = true;
    return cmp5_v1 <= cmp5_v2; 
}

cmp6_main : (  ) bool { 
    cmp6_v1 : int = 1;
    cmp6_v2 : bool = true;
    return cmp6_v1 >= cmp6_v2; 
}

cmp7_f1 : (  ) bool { return true; }
cmp7_main : (  ) bool { 
    cmp6_v7 : int = 1;
    return cmp7_f1() < cmp6_v7; 
}

cmp8_f1 : (  ) bool { return true; }
cmp8_main : (  ) bool { 
    return cmp8_f1() < cmp8_f1(); 
}

//Applying an (in)equality operator to two valid but incompatible operands, such as an int to a bool.

cmp9_main : (  ) bool { 
    cmp9_v1 : int = 1;
    cmp9_v2 : bool = true;
    return cmp9_v1 == cmp9_v2; 
}

cmp10_main : (  ) bool { 
    cmp10_v1 : int = 1;
    cmp10_v2 : bool = true;
    return cmp10_v1 != cmp10_v2; 
}

//An invalid operand given to a logical operator (!, &&, ||).

logic_main : (  ) bool { 
    logic_v1 : int = 1;
    logic_v2 : bool = true;
    return logic_v1 and logic_v2; 
}

logic2_main : (  ) bool { 
    logic2_v1 : int = 1;
    logic2_v2 : bool = true;
    return logic2_v1 and logic2_v1; 
}

logic3_main : (  ) bool { 
    logic3_v1 : int = 1;
    logic3_v2 : bool = true;
    return ! logic3_v1; 
}

logic4_main : (  ) bool { 
    logic4_v1 : int = 1;
    logic4_v2 : bool = true;
    return logic4_v1 or logic4_v1; 
}

logic5_f1 : (  ) int { return 1; }
logic5_main : (  ) bool { 
    logic5_v7 : bool = true;
    return logic5_f1() or logic5_v7; 
}

logic6_f1 : (  ) int { return 1; }
logic6_main : (  ) bool { 
    return logic6_f1() or logic6_f1(); 
}

//Using a non-bool expression as a condition (such as an if statement, or loop.

cond_main : (  ) void { 
    if( 1 ) { }
}

cond2_main : (  ) void {
    cond2_v1 : int = 1; 
    if( cond2_v1 ) { }
}

cond3_main : (  ) void { 
    while( 1 ) { }
}

cond4_f1 : (  ) int { return 1; }
cond4_main : (  ) void { 
    if( cond4_f1() ) { }
}

//Applying an (in)equality operator to an invalid operand, such as a void function call result, or classes, or a function name (not a call) .

equals_f1 : (  ) int { return 1; }
equals_main : (  ) bool { 
    return equals_f1 == equals_f1;
}

equals2_c1 : class {  } ;
equals2_main : (  ) bool { 
    return equals2_c1 == equals2_c1;
}

equals3_f1 : (  ) void { }
equals3_main : (  ) bool { 
    return equals3_f1() == equals3_f1();
}

