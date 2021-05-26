(lec-2 p6)

Grammer BNF.
<assign> -> <id> = <expr>
<expr> -> <expr> + <term> | <expr> - <term> | <term>
<term> -> <term> * <factor> | <term> / <factor> | <factor>
<factor> -> <newexp> ^ <factor> | <newexp>
<newexp> -> ( <expr> ) | <id>
<id> -> A | B | C





BNFC.
Assign1. assign ::= id    "="  expr ;
Expr1.   expr   ::= expr  "+"  term   |  expr  "-"  term  |  term ;
Term1.   term   ::= term  "*"  factor |  term  "/"  factor  |  factor ; 
Factor.  factor ::=  newexp  "^"  factor  |  newexp ;
Newexp1. newexp ::= "(" expr ")" |  id ;
ID1.     id     ::= "A" | "B" | "C" ;

