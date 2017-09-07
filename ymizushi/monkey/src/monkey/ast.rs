use monkey::lang::{Token, Value};

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

struct Program {
    statements: Vec<Box<Statement>>
}

impl Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }

    fn string(&self) -> String {
        let mut out = String::from("");
        for s in self.statements {
            out.push_str(s);
        }
        out
    }
}

struct Identifier {
    token: Token,
    value: String
}

impl Identifier {
    fn expression_node() {
    }

    fn token_literal(&self) -> String {
        match self.token.value {
            Some(Value::Str(s)) => s,
            None => String::from("")
        }
    }

    fn string(&self) -> string {
        self.value
    }
}





struct LetStatement {
    token: String,
    name: String,
    value: String,
}


impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn token_literal(&self) -> String {
        self.token
    }

    fn string(&self) -> String {
        let mut out = String::from("");
        out.push_str(self.token_literal());
        out.push_str(" ");
        out.push_str(self.name);
        out.push_str(" = ");

        if self.value != "" {
            out.push_str(self.value);
        }
        out.push_str(";");
        out
    }
}


struct ReturnStatement {
    token: Token,
    return_value: Expression,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {
    }

    fn token_literal(&self) -> String {
        self.token.literal
    }

    fn string(&self) -> String {
        let mut out = String::from("");
        out.push_str(self.token_literal);
        out.push_str(" ");

        if self.return_value != "" {
            out.push_str(self.return_value.string());
        }

        out.push_str(";");
        out.to_string();
    }
}

struct ExpressionStatement {
    token: Token,
    expression: Expression
}


impl ExpressionStatement {
    fn statement_node() {}

    fn token_literal(&self) -> String {
        self.token_literal()
    }

    fn string(&self) -> String {
        if self.expression != None {
            self.expression.string()
        } else {
           ""
        }
    }
}

struct IntegerLiteral {
    token: Token,
    value: i64
}

impl IntegerLiteral {
    fn expression_node() {}

    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.token.literal()
    }
}


struct PrefixExpression {
    token: Token,
    operator: String,
    right: Expression
}

impl PrefixExpression {
    fn expression_node(&self) {
    }

    fn token_literal(&self) -> String {
        self.token.literal
    }

    fn string(&self) -> String {
        let mut out = String::from("");

        out.push_str("(");
        out.push_str(self.operator);
        out.push_str(self.right.string());
        out.push_str(")");
        out
    }
}

struct InfixExpression {
    token: Token,
    left: Expression,
    operator: String,
    right: Expression
}

impl InfixExpression {
    fn expression_node(&self) {}

    fn token_literal(&self) -> String {
        let mut out = String::from("");
        out.push_str("(");
        out.push_str(self.left.string());
        out.push_str(" " + self.operator + " ");
        out.push_str(self.right.string());
        out.push_str(")");
        out.string()
    }
}

struct Boolean {
    token: Token,
    value: Boolean
}

impl Boolean {
    fn expression_node() {}
    fn token_literal(&self) -> String {
        self.token.literal
    }

    fn string(&self) -> string {
        self.token.literal
    }
}

struct IfExpression {
    token: Token,
    condition: Expression,
    consequence: BlockStatement,
    alternative: BlockStatement
}


impl IfExpression {
    fn expression_node(&self) {}

    fn token_literal(&self) -> String {
        self.token.literal
    }

    fn string(&self) -> String {
        let mut out = String::from("");
        out.push_str("if");
        out.push_str(self.condition.string());
        out.push_str(" ");
        out.push_str(self.consequence.string());
        if self.alternative != None {
            out.push_str("else ");
            out.push_str(self.alternative.string());
        }
        out.string()
    }

}

struct BlockStatement {
    token: Token,
    statements: [Statement]
}

impl BlockStatement {
    fn statement_node(&self) {}
    fn token_literal(&self) -> String {
        self.token.literal
    }

    fn string(&self) -> String {
        let mut out = String::from("");

        for s in self.statements {
            out.push_str(s.string());
        }

        out.to_string()
    }
}

struct FunctionLiteral {
    token: Token,
    parameters: [Identifier],
    body: BlockStatement
}

impl FunctionLiteral {
    fn expression_node(&self) {}

    fn token_literal(&self) -> String {
        self.token.literal
    }

    fn string(&self) -> String {
        let mut out = String::from("");
        let mut params = String::from("");
        for p in self.parameters {
            params.push_str(p.string());
        }

        out.push_str(self.token_literal());
        out.push_str("(");
        out.push_str(String::join(params, ", "));
        out.push_str(")");
        out.push_str(&self.body.string());
        out.to_string()
    }
}

struct CallExpression {
    token: Token,
    function: Expression,
    arguments: [Expression]
}

impl CallExpression {
    fn expression_node(&self) {}

    fn token_literal(&self) -> String {
        let mut out = String::from("");

        let mut args = String::from("");

        for a in self.arguments {
            args.push_str(a.string());
        }

        out.push_str(self.function.string());
        out.push_str("(");
        out.push_str(String::join(args, ","));
        out.push_str(")");

        out.string()
    }

}

impl Node for LetStatement {
    fn token_literal() -> &'static str {
        "hoge"
    }
}




