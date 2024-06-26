grammar eTamil;

program: statement+ ;

statement: assignment | expression | printStmt ;

assignment: IDENTIFIER '=' expression ;

printStmt: 'accu' '(' expression ')' ;

expression: term (( '+' | '-' ) term)* ;

term: factor (( '*' | '/' ) factor)* ;

factor: IDENTIFIER | NUMBER | '(' expression ')' ;

IDENTIFIER: [a-zA-Z_][a-zA-Z_0-9]* ;
NUMBER: [0-9]+ ;
WS: [ \t\r\n]+ -> skip ;
