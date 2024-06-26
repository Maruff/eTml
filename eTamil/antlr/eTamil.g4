grammar eTamil;

program: statement+ ;

statement: assignment | expression ;

assignment: IDENTIFIER '=' expression ;

expression: term (( '+' | '-' ) term)* ;

term: factor (( '*' | '/' ) factor)* ;

factor: IDENTIFIER | NUMBER | '(' expression ')' ;

IDENTIFIER: [a-zA-Z_][a-zA-Z_0-9]* ;
NUMBER: [0-9]+ ;
WS: [ \t\r\n]+ -> skip ;
