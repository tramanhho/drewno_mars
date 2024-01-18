//Multiply declared identifier

//init same type
mult1 : int = 1 ;
mult1 : int = 1 ;

//init dif type
mult2 : int = 1 ;
mult2 : bool = 1;

//uninit same type
mult3 : int;
mult3 : int;

//uninit dif type
mult4 : int;
mult4 : bool;

//uninit same perfect type
mult5 : perfect int;
mult5 : perfect int;

//uninit dif perfect type
mult6 : perfect int;
mult6 : perfect bool;

//uninit dif some perfect type 
mult7 : int;
mult7 : perfect int;

//uninit same type diff scope 
mult8 : int;
mult8class : class { mult8 : int; } ;

