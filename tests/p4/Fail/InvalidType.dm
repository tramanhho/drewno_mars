//Invalid type in declaration

//init void var
invaltype1 : void = 1 ;

//fn passed void param
invaltype2fn1 : ( invaltype2 : void ) void { } //invalid type (void)->void
invaltype2fn2 : (  ) void { 
    invaltype3 : void;
    invaltype2fn1 ( invaltype2 ); //invalid peram void
}

//Invalid type in declaration and Multiply declared identifier
invaltype4 : int;
invaltype4 : void = 1;