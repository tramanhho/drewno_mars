//Printing a function; e.g., give f, where f is a function name.

printf_f1 : (  ) int { return 1; }
printf_main : (  ) void { give printf_f1; }

//Printing a class-type value function); e.g., give c, where c is a class name or instance of a class.

printc_c1 : class {  } ;
printc_main : (  ) void { give printc_c1; }

//Printing a void value (note: this can only happen if there is an attempt to write the return value from a void function); e.g., give f(), where f is a void function.

printv_f1 : (  ) void { }
printv_main : (  ) void { give printv_f1(); }

//Reading a function: e.g., take f, where f is a function name.

readf_f1 : (  ) int { return 1; }
readf_main : (  ) void { take readf_f1; }

//Reading a class: e.g., take c, where c is a class name or instance of a class.

readc_c1 : class {  } ;
readc_main : (  ) void { take readc_c1; }
