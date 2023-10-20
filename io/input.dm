class1:class{
  a:int = 12;
  field:int;
  var:perfect bool;
};

global : int;
class2 : class {
    field : class1;
};

main : () void {
    global : int;
    owo: class1;
    owo--field = 3;
    //owo--undeclared = 4;

    uwu : class2;
    uwu--field--a++;

    a : int = 3;
    a = a + 2;
}