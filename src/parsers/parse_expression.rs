use colour::red_ln;

use super::{
    ast_nodes::{AstNode, Reference}, collections::new_tuple,
    create_scene_node::new_scene,
};
use crate::{bs_types::DataType, Token};

/*  CAN RETURN:
     - an expression
     - a tuple of expresions
     - an error

     DOES NOT CARE ABOUT TYPES (yet)
     can return a mix of types in the same expression
     Enforcing the type is done when the expression is evaluated
     Evaluated expressions must be of the same type
*/
pub fn create_expression(
    tokens: &Vec<Token>,
    i: &mut usize,
    inside_tuple: bool,
    ast: &Vec<AstNode>,
    starting_line_number: &u32,
    data_type: &DataType,
    inside_brackets: bool,
    variable_declarations: &Vec<Reference>,
) -> AstNode {
    let mut expression = Vec::new();
    let mut current_type = data_type.to_owned();
    
    if inside_brackets {
        *i += 1;
    }

    // Loop through the expression and create the AST nodes (increment i each time)
    // Figure out the type it should be from the data
    // DOES NOT MOVE TOKENS PAST THE CLOSING TOKEN
    let mut next_number_negative = false;
    while let Some(token) = tokens.get(*i) {
        match token {
            // Conditions that close the expression
            Token::CloseParenthesis => {
                if inside_brackets {
                    if expression.is_empty() {
                        return AstNode::Empty;
                    }
                    *i += 1;
                    break;
                } else {
                    if inside_tuple {
                        break;
                    }
                    // Mismatched brackets, return an error
                    return AstNode::Error(
                        "Mismatched brackets in expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
            }

            Token::OpenParenthesis => {
                return create_expression(
                    tokens,
                    i,
                    false,
                    ast,
                    starting_line_number,
                    &DataType::Inferred,
                    true,
                    variable_declarations,
                );
            }

            Token::EOF | Token::SceneClose(_) | Token::Arrow | Token::Colon | Token::End => {
                if inside_brackets {
                    return AstNode::Error(
                        "Not enough closing parenthesis for expression. Need more ')' at the end of the expression!".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                break;
            }

            Token::Newline => {
                // Fine if inside of brackets (not closed yet)
                // Otherwise break out of the expression
                if inside_brackets {
                    continue;
                } else {
                    break;
                }
            }

            Token::Comma => {
                if inside_tuple {
                    break;
                }
                *i += 1;

                return new_tuple(
                    tokens,
                    i,
                    AstNode::Expression(expression, starting_line_number.to_owned()),
                    ast,
                    starting_line_number,
                    variable_declarations,
                );
            }

            // Check if name is a reference to another variable or function call
            Token::Variable(name) => {

                let var = variable_declarations.iter().find(|var| var.name == *name);
                match var {
                    Some(var) => {
                        // If this expression is inferring it's type from the expression
                        if current_type == DataType::Inferred {
                            current_type = var.data_type.to_owned();
                        }

                        // If the variables type is known and not the same as the type of the expression
                        // Return a type error
                        if var.data_type != DataType::Inferred && var.data_type != current_type && current_type != DataType::CoerseToString {
                            return AstNode::Error(
                                format!(
                                    "Variable {} is of type {:?}, but used in an expression of type {:?}",
                                    var.name, var.data_type, data_type
                                ),
                                starting_line_number.to_owned(),
                            );
                        }

                        if var.is_const {
                            expression.push(AstNode::ConstReference(var.name.to_owned(), var.data_type.to_owned()));
                        } else {
                            expression.push(AstNode::VarReference(var.name.to_owned(), var.data_type.to_owned()));
                        };
                    }
                    None => {
                        expression.push(AstNode::Error(
                            format!("Variable {} not found in scope", name),
                            starting_line_number.to_owned(),
                        ));
                    }
                }
            }

            // Check if is a literal
            Token::FloatLiteral(mut float) => {
                if data_type != &DataType::Float
                    && data_type != &DataType::Inferred
                    && data_type != &DataType::CoerseToString
                {
                    return AstNode::Error(
                        "Float literal used in non-float expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                if next_number_negative {
                    float = -float;
                    next_number_negative = false;
                }
                expression.push(AstNode::Literal(Token::FloatLiteral(float)));
            }
            Token::StringLiteral(string) => {
                if data_type != &DataType::String
                    && data_type != &DataType::CoerseToString
                    && data_type != &DataType::Inferred
                {
                    return AstNode::Error(
                        "String literal used in non-string expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                expression.push(AstNode::Literal(Token::StringLiteral(string.clone())));
            }

            // Scenes - Create a new scene node
            // Maybe scenes can be added together like strings
            Token::SceneHead | Token::ParentScene => {
                if data_type != &DataType::Scene && data_type != &DataType::Inferred {
                    return AstNode::Error(
                        "Scene used in non-scene expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                return new_scene(tokens, i, &ast, starting_line_number, variable_declarations);
            }

            // OPERATORS
            // Will push as a string so shunting yard can handle it later just as a string
            Token::Negative => {
                next_number_negative = true;
            }

            // BINARY OPERATORS
            Token::Add => {
                expression.push(AstNode::BinaryOperator(token.to_owned(), 1));
            }
            Token::Subtract => {
                if data_type != &DataType::Float
                    && data_type != &DataType::Inferred
                    && data_type != &DataType::CoerseToString
                {
                    return AstNode::Error(
                        "Subtraction used in non-float expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                expression.push(AstNode::BinaryOperator(token.to_owned(), 1));
            }
            Token::Multiply => {
                if data_type != &DataType::Float
                    && data_type != &DataType::Inferred
                    && data_type != &DataType::CoerseToString
                {
                    return AstNode::Error(
                        "Multiplication used in non-float expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                expression.push(AstNode::BinaryOperator(token.to_owned(), 2));
            }
            Token::Divide => {
                if data_type != &DataType::Float
                    && data_type != &DataType::Inferred
                    && data_type != &DataType::CoerseToString
                {
                    return AstNode::Error(
                        "Division used in non-float expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                expression.push(AstNode::BinaryOperator(token.to_owned(), 2));
            }
            Token::Modulus => {
                if data_type != &DataType::Float
                    && data_type != &DataType::Inferred
                    && data_type != &DataType::CoerseToString
                {
                    return AstNode::Error(
                        "Modulus used in non-float expression".to_string(),
                        starting_line_number.to_owned(),
                    );
                }
                expression.push(AstNode::BinaryOperator(token.to_owned(), 2));
            }

            // LOGICAL OPERATORS
            Token::Equal => {
                expression.push(AstNode::LogicalOperator(Token::Equal, 5));
            }
            Token::LessThan => {
                expression.push(AstNode::LogicalOperator(Token::LessThan, 5));
            }
            Token::LessThanOrEqual => {
                expression.push(AstNode::LogicalOperator(Token::LessThanOrEqual, 5));
            }
            Token::GreaterThan => {
                expression.push(AstNode::LogicalOperator(Token::GreaterThan, 5));
            }
            Token::GreaterThanOrEqual => {
                expression.push(AstNode::LogicalOperator(Token::GreaterThanOrEqual, 5));
            }
            Token::And => {
                expression.push(AstNode::LogicalOperator(Token::And, 4));
            }
            Token::Or => {
                expression.push(AstNode::LogicalOperator(Token::Or, 3));
            }

            _ => {
                expression.push(AstNode::Error(
                    format!(
                        "Invalid Expression: {:?}, must be assigned with a valid datatype",
                        token
                    ),
                    starting_line_number.to_owned(),
                ));
            }
        }

        *i += 1;
    }

    return evaluate_expression(
        AstNode::Expression(expression, starting_line_number.to_owned()),
        data_type,
        ast,
    );
}

// This function takes in an Expression node or Collection of expressions that has a Vec of Nodes to evaluate
// And evaluates everything possible at compile time (Constant Folding)
// If it returns a literal, then everything was evaluated at compile time
// Otherwise it will return an EvaluatedExpression, which has a strict type and will be evaluated at runtime
pub fn evaluate_expression(
    expr: AstNode,
    type_declaration: &DataType,
    ast: &Vec<AstNode>,
) -> AstNode {
    let mut current_type = type_declaration.to_owned();
    let mut simplified_expression = Vec::new();
    let mut runtime_nodes: usize = 0;

    // SHUNTING YARD ALGORITHM
    let mut output_stack: Vec<AstNode> = Vec::new();
    let mut operators_stack: Vec<AstNode> = Vec::new();
    match expr {
        AstNode::Expression(e, line_number) => {
            for ref node in e {
                match node {
                    AstNode::Expression(nested_e, nested_line_number) => {
                        simplified_expression.push(evaluate_expression(
                            AstNode::Expression(nested_e.to_owned(), nested_line_number.to_owned()),
                            type_declaration,
                            ast,
                        ));
                    }
                    AstNode::Literal(token) => match token {
                        Token::FloatLiteral(value) => {
                            if current_type == DataType::CoerseToString {
                                simplified_expression.push(AstNode::Literal(Token::StringLiteral(
                                    value.to_string(),
                                )));
                                continue;
                            }
                            output_stack.push(node.to_owned());
                            if current_type == DataType::Inferred {
                                current_type = DataType::Float;
                            }
                        }
                        Token::StringLiteral(_) => {
                            simplified_expression.push(node.to_owned());
                            if current_type == DataType::Inferred {
                                current_type = DataType::String;
                            }
                        }
                        Token::BoolLiteral(_) => {
                            output_stack.push(node.to_owned());
                            if current_type == DataType::Inferred {
                                current_type = DataType::Bool;
                            }
                        }
                        _ => {
                            red_ln!("Compiler error: (Eval Expression) Wrong literal type found in expression");
                        }
                    },

                    AstNode::ConstReference(_, data_type) => {
                        if current_type == DataType::Inferred {
                            current_type = data_type.to_owned();
                        }

                        match current_type {
                            DataType::Float | DataType::Bool => {
                                output_stack.push(node.to_owned());
                            }
                            DataType::String | DataType::CoerseToString => {
                                simplified_expression.push(node.to_owned());
                            }
                            _ => {
                                return AstNode::Error(
                                    format!(
                                        "unsupported data type for constants in expressions: {:?}",
                                        current_type
                                    ),
                                    line_number,
                                );
                            }
                        }
                    }

                    AstNode::VarReference(_, data_type) => {
                        if current_type == DataType::Inferred {
                            current_type = data_type.to_owned();
                        }

                        match current_type {
                            DataType::Float | DataType::Bool => {
                                output_stack.push(node.to_owned());
                            }
                            DataType::String | DataType::CoerseToString => {
                                simplified_expression.push(node.to_owned());
                            }
                            _ => {
                                return AstNode::Error(
                                    format!(
                                        "unsupported data type for variables in expressions: {:?}",
                                        current_type
                                    ),
                                    line_number,
                                );
                            }
                        }

                        runtime_nodes += 1;
                    }

                    AstNode::BinaryOperator(op, precedence) => {
                        // If the current type is a string or scene, add operator is assumed.
                        if current_type == DataType::String || current_type == DataType::Scene {
                            if op != &Token::Add {
                                return AstNode::Error(
                                    "Can only use the + operator to manipulate strings or scenes inside expressions".to_string(),
                                    line_number
                                );
                            }
                            simplified_expression.push(node.to_owned());
                            continue;
                        }

                        if current_type == DataType::CoerseToString {
                            simplified_expression.push(node.to_owned());
                        }

                        if current_type == DataType::Bool {
                            if *op != Token::Or || *op != Token::And {
                                return AstNode::Error(
                                    "Can only use 'or' and 'and' operators with booleans".to_string(),
                                    line_number
                                );
                            }
                            operators_stack.push(node.to_owned());
                        }

                        if operators_stack.last().is_some_and(|x| match x {
                            AstNode::BinaryOperator(_, p) => p >= &precedence,
                            _ => false,
                        }) {
                            output_stack.push(operators_stack.pop().unwrap());
                        }

                        operators_stack.push(node.to_owned());
                    }

                    AstNode::Scene(_, _, _, _) => {
                        if current_type == DataType::Inferred {
                            current_type = DataType::Scene;
                        }

                        if current_type != DataType::Scene {
                            return AstNode::Error(
                                "Scene used in non-scene expression".to_string(),
                                line_number,
                            );
                        }
                        output_stack.push(node.to_owned());
                    }

                    _ => {
                        red_ln!(
                            "unknown AST node found in expression when evaluating expression: {:?}",
                            node
                        );
                    }
                }
            }
        }

        AstNode::Tuple(e, line_number) => {
            for node in e {
                match node {
                    AstNode::Expression(e, line_number) | AstNode::Tuple(e, line_number) => {
                        simplified_expression.push(evaluate_expression(
                            AstNode::Expression(e, line_number),
                            type_declaration,
                            ast,
                        ));
                    }
                    _ => {
                        simplified_expression.push(node);
                    }
                }
            }

            return AstNode::Tuple(simplified_expression, line_number);
        }
        _ => {
            red_ln!("Compiler Bug: No Expression to Evaluate - eval expression passed wrong AST node: {:?}", expr);
        }
    }

    // If nothing to evaluate at compile time, just one value, return that value
    if simplified_expression.len() == 1 {
        return simplified_expression[0].clone();
    }

    // LOGICAL EXPRESSIONS
    if current_type == DataType::Bool {
        for operator in operators_stack {
            output_stack.push(operator);
        }

        return logical_constant_fold(output_stack, current_type);
    }

    // SCENE EXPRESSIONS
    // If constant scene expression, combine the scenes together and return the new scene
    if current_type == DataType::Scene && runtime_nodes == 0 {
        return concat_scene(&mut simplified_expression);
    }

    // STRING EXPRESSIONS
    // If the expression is a constant string, combine and return a string
    if current_type == DataType::String && runtime_nodes == 0 {
        return concat_strings(&mut simplified_expression);
    }

    // Scene Head Coerse to String
    if current_type == DataType::CoerseToString {
        return AstNode::RuntimeExpression(simplified_expression, current_type);
    }

    // MATHS EXPRESSIONS
    // Push everything into the stack, is now in RPN notation
    for operator in operators_stack {
        output_stack.push(operator);
    }

    // Evaluate all constants in the maths expression
    return math_constant_fold(output_stack, current_type);
}

// This will evaluate everything possible at compile time
// returns either a literal or an evaluated runtime expression
fn math_constant_fold(
    output_stack: Vec<AstNode>,
    current_type: DataType,
) -> AstNode {
    let mut stack: Vec<AstNode> = Vec::new();

    for node in &output_stack {
        match node {
            AstNode::BinaryOperator(op, _) => {
                // Make sure there are at least 2 nodes on the stack
                if stack.len() < 2 {
                    return AstNode::Error("Not enough nodes on the stack for binary operator when parsing an expression".to_string(), 0);
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                // Check if top 2 of stack are literals
                // if at least one is not then this must be a runtime expression
                // And just push the operator onto the stack instead of evaluating
                let left_value = match left {
                    AstNode::Literal(Token::FloatLiteral(value)) => value,
                    _ => {
                        stack.push(left);
                        stack.push(right);
                        stack.push(node.to_owned());
                        continue;
                    }
                };

                let right_value = match right {
                    AstNode::Literal(Token::FloatLiteral(value)) => value,
                    _ => {
                        stack.push(left);
                        stack.push(right);
                        stack.push(node.to_owned());
                        continue;
                    }
                };
                
                stack.push(AstNode::Literal(Token::FloatLiteral(match op {
                    Token::Add => left_value + right_value,
                    Token::Subtract => left_value - right_value,
                    Token::Multiply => left_value * right_value,
                    Token::Divide => left_value / right_value,
                    Token::Modulus => left_value % right_value,
                    _ => {
                        return AstNode::Error(format!("Unsupported operator: {:?}", op), 0);
                    }
                })));
            }
            // Some runtime thing
            _ => {
                stack.push(node.to_owned());
            }
        }
    }

    if stack.len() == 1 {
        return stack.pop().unwrap();
    }

    AstNode::RuntimeExpression(stack, current_type)
}

fn logical_constant_fold(
    output_stack: Vec<AstNode>,
    current_type: DataType,
) -> AstNode {
    let mut stack: Vec<AstNode> = Vec::new();

    for node in &output_stack {
        match node {
            AstNode::LogicalOperator(op, _) => {
                // Make sure there are at least 2 nodes on the stack
                if stack.len() < 2 {
                    return AstNode::Error("Not enough nodes on the stack for logical operator when parsing an expression".to_string(), 0);
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                // Check if top 2 of stack are literals
                // if at least one is not then this must be a runtime expression
                // And just push the operator onto the stack instead of evaluating
                let left_value = match left {
                    AstNode::Literal(Token::BoolLiteral(value)) => value,
                    _ => {
                        stack.push(left);
                        stack.push(right);
                        stack.push(node.to_owned());
                        continue;
                    }
                };

                let right_value = match right {
                    AstNode::Literal(Token::BoolLiteral(value)) => value,
                    _ => {
                        stack.push(left);
                        stack.push(right);
                        stack.push(node.to_owned());
                        continue;
                    }
                };
                
                stack.push(AstNode::Literal(Token::BoolLiteral(match op {
                    Token::Equal => left_value == right_value,
                    Token::And => left_value && right_value,
                    Token::Or => left_value || right_value,
                    _ => {
                        return AstNode::Error(format!("Unsupported operator: {:?}", op), 0);
                    }
                })));
            }
            // Some runtime thing
            _ => {
                stack.push(node.to_owned());
            }
        }
    }

    if stack.len() == 1 {
        return stack.pop().unwrap();
    }

    AstNode::RuntimeExpression(stack, current_type)
}

// Concat strings at COMPILE TIME ONLY
fn concat_strings(simplified_expression: &mut Vec<AstNode>) -> AstNode {
    let mut new_string = String::new();
    let mut previous_node_is_plus = false;

    for node in simplified_expression {
        match node {
            AstNode::Literal(Token::StringLiteral(string)) => {
                if previous_node_is_plus || new_string.is_empty() {
                    new_string.push_str(string);
                    previous_node_is_plus = false;
                } else {
                    // Syntax error, must have a + operator between strings when concatinating
                    red_ln!(
                        "Syntax Error: Must have a + operator between strings when concatinating"
                    );
                }
            }
            AstNode::BinaryOperator(_, _) => {
                // Should always be a plus operator, this is enforced in the eval_expression function
                previous_node_is_plus = true;
            }
            _ => {
                red_ln!("Compiler Bug: Cannot evaluate string expression at compile time. Compiler should be creating a runtime string expression");
            }
        }
    }

    AstNode::Literal(Token::StringLiteral(new_string))
}

fn concat_scene(simplified_expression: &mut Vec<AstNode>) -> AstNode {
    let mut new_scene: AstNode = AstNode::Scene(Vec::new(), Vec::new(), Vec::new(), Vec::new());

    for node in simplified_expression {
        match node {
            AstNode::Scene(vec1, vec2, vec3, vec4) => match new_scene {
                AstNode::Scene(ref mut v1, ref mut v2, ref mut v3, ref mut v4) => {
                    v1.append(vec1);
                    v2.append(vec2);
                    v3.append(vec3);
                    v4.append(vec4);
                }
                _ => {
                    red_ln!("Compiler Bug: Cannot evaluate scene expression at compile time. Compiler should be creating a runtime scene expression");
                }
            },
            _ => {
                red_ln!("Compiler Bug: Cannot evaluate scene expression at compile time. Compiler should be creating a runtime scene expression");
            }
        }
    }

    new_scene
}

fn _check_literal(
    value: Token,
    type_declaration: &DataType,
    current_type: &mut DataType,
    line_number: u32,
) -> AstNode {
    if type_declaration == &DataType::CoerseToString {
        return AstNode::Literal(value);
    }
    match value {
        Token::FloatLiteral(_) => {
            if type_declaration == &DataType::Inferred {
                *current_type = DataType::Float;
            } else if type_declaration != &DataType::Float {
                return AstNode::Error(
                    "Error Mixing types. You must explicitly convert types to use them in the same expression".to_string(),
                    line_number
                );
            }
            AstNode::Literal(value)
        }
        Token::StringLiteral(_) => {
            if type_declaration == &DataType::Inferred {
                *current_type = DataType::String;
            } else if type_declaration != &DataType::String {
                return AstNode::Error(
                    "Error Mixing types. You must explicitly convert types to use them in the same expression".to_string(),
                    line_number
                );
            }

            AstNode::Literal(value)
        }
        _ => AstNode::Error("Invalid Literal (check_literal)".to_string(), line_number),
    }
}

pub fn check_if_arg(tokens: &Vec<Token>, i: &mut usize) -> bool {
    if *i >= tokens.len() {
        return false;
    }
    match &tokens[*i] {
        // Check if open bracket, literal or prefixed unary operator
        Token::OpenParenthesis
        | Token::Negative
        | Token::StringLiteral(_)
        | Token::BoolLiteral(_)
        | Token::RawStringLiteral(_)
        | Token::FloatLiteral(_) => true,
        _ => false,
    }
}
