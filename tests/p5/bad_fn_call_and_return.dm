//Calling something other than a function; e.g., "x();", where x is not a function name. Note: In this case, you should not type-check the actual parameters.

callNonFunc_main : (  ) void { callNonFunc2_v1 : int = 0; callNonFunc2_v1(); }

callNonFunc2_c1 : class {  } ;
callNonFunc2_main : (  ) void { callNonFunc2_c1(); }

//Calling a function with the wrong number of arguments.

callWrongArgNum_f1 : (  ) void { }
callWrongArgNum_main : (  ) void { callWrongArgNum_v1 : int = 0; callWrongArgNum_f1(callWrongArgNum_v1); }

callWrongArgNum2_f1 : ( callWrongArgNum2_v1 : int ) void { }
callWrongArgNum2_main : (  ) void { callWrongArgNum2_f1(); }

callWrongArgNum3_f1 : ( callWrongArgNum3_v1 : int, callWrongArgNum3_v2 : int ) void { }
callWrongArgNum3_main : (  ) void { 
    callWrongArgNum_v1 : int = 0; 
    callWrongArgNum_f1(callWrongArgNum_v1); 
}

//Calling a function with argument(s) of the wrong type. If there are several arguments with the wrong type, give an error message for each such argument.

callWrongArgType_f1 : ( callWrongArgType_v1 : int ) void { }
callWrongArgType_main : (  ) void { 
    callWrongArgType_v1 : bool = false; 
    callWrongArgType_f1(callWrongArgType_v1); 
}

callWrongArgType2_f1 : ( callWrongArgType2_v1 : bool ) void { }
callWrongArgType2_main : (  ) void { 
    callWrongArgType2_v1 : int = 1; 
    callWrongArgType2_f1(callWrongArgType2_v1); 
}

callWrongArgType3_f1 : ( callWrongArgType3_v1 : bool, callWrongArgType3_v2 : int ) void { }
callWrongArgType3_main : (  ) void { 
    callWrongArgType3_v1 : int = 1; 
    callWrongArgType3_f1(callWrongArgType3_v1, callWrongArgType3_v1); 
}

callWrongArgType4_f1 : ( callWrongArgType4_v1 : bool, callWrongArgType4_v2 : bool ) void { }
callWrongArgType4_main : (  ) void { 
    callWrongArgType4_v1 : int = 1; 
    callWrongArgType4_f1(callWrongArgType4_v1, callWrongArgType4_v1); 
}

callWrongArgType5_f1 : ( callWrongArgType5_v1 : bool, callWrongArgType5_v2 : bool ) void { }
callWrongArgType5_main : (  ) void { 
    callWrongArgType5_v1 : int = 1; 
    callWrongArgType5_v2 : int = 1; 
    callWrongArgType5_f1(callWrongArgType5_v1, callWrongArgType5_v2); 
}


//Using an empty return statement (i.e., one that does not return a value) in a function with a non-void return type.

retEmpty_main : (  ) void { return; }

retEmpty1_main : (  ) int { return; }

retEmpty2_main : (  ) bool { return; }

//no return on non-void, there should be no error

retEmpty3_main : (  ) void {  }

retEmpty4_main : (  ) int {  }

retEmpty5_main : (  ) bool {  }

//Returning a value from a void function.

retValueFromVoid_main : (  ) void { return 1; }

retValueFromVoid2_main : (  ) void { return false; }

retValueFromVoid3_main : (  ) void {
    retValueFromVoid3_v1 : int = 1; 
    return retValueFromVoid3_v1;
}

//Returning a value of the wrong type from a non-void function.

retWrongType_main : (  ) int { return false; }

retWrongType2_main : (  ) bool { return 1; }

retWrongType3_main : (  ) bool {
    retWrongType3_v1 : int = 1; 
    return retWrongType3_v1;
}

