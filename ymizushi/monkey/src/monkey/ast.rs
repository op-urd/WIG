use monkey::lang::{Token, Value};

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

}

trait Node {
    fn token_literal() -> &'static str;
}


trait Statement: Node {
    fn statement_node();
}


struct LetStatement {
    token: String,
    name: String,
    value: String,
}


impl Statement for LetStatement {
    fn statement_node() {}
}

impl Node for LetStatement {
    fn token_literal() -> &'static str {
        "hoge"
    }
}

trait Expression: Node {
    fn expression_node();
}


struct Program {}
