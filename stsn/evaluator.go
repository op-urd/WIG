package main

var (
	STRUE  = &Boolean{Value: true}
	SFALSE = &Boolean{Value: false}
	SNULL  = &Null{}
)

func Eval(node Node) Object {
	switch node := node.(type) {
	// Statements
	case *Program:
		return evalStatements(node.Statements)
	case *ExpressionStatement:
		return Eval(node.Expression)
	case *IntegerLiteral:
		return &Integer{Value: node.Value}
	case *Boolean:
		return nativeBoolToBooleanObject(node.Value)
	case *PrefixExpression:
		right := Eval(node.Right)
		return evalPrefixExpression(node.Operator, right)
	}

	return nil
}

func evalPrefixExpression(operator string, right Object) Object {
	switch operator {
	case "!":
		return evalBangOperatorExpression(right)
	case "-":
		return evalMinusPrefixOperatorExpression(right)
	default:
		return SNULL
	}
}

func evalMinusPrefixOperatorExpression(right Object) Object {
	if right.Type() != INTEGER_OBJ {
		return SNULL
	}
	value := right.(*Integer).Value
	return &Integer{Value: -value}
}

func evalBangOperatorExpression(right Object) Object {
	switch right {
	case STRUE:
		return SFALSE
	case SFALSE:
		return STRUE
	case SNULL:
		return STRUE
	default:
		return SFALSE
	}
}

func nativeBoolToBooleanObject(input bool) *Boolean {
	if input {
		return STRUE
	}
	return SFALSE
}

func evalStatements(stmts []Statement) Object {
	var result Object

	for _, statement := range stmts {
		result = Eval(statement)
	}
	return result
}
