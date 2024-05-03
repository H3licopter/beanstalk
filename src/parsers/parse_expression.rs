use super::{ast::AstNode, build_ast::find_var_declaration_index, collections::new_tuple};
use crate::{bs_types::DataType, Token};

/*  CAN RETURN:
     - a literal
     - an expression
     - an empty expression for functions
     - a collection of expressions or literals


     DOES NOT CARE ABOUT TYPES (yet)
     can return a mix of types in the same expression
     Enforcing the type is done when the expression is evaluated
     Evaluated expressions must be of the same type
*/
pub fn create_expression(tokens: &Vec<Token>, i: &mut usize, inside_tuple: bool, ast: &Vec<AstNode>) -> AstNode {
    let mut expression = Vec::new();

    // Check if value is wrapped in brackets and move on until first value is found
    let mut bracket_nesting: i32 = 0;
    while &tokens[*i] == &Token::OpenParenthesis {
        bracket_nesting += 1;
        *i += 1;
    }

    // Loop through the expression and create the AST nodes (increment i each time)
    // Figure out the type it should be from the data
    // DOES NOT MOVE TOKENS PAST THE CLOSING TOKEN
    while let Some(token) = tokens.get(*i) {
        match token {
            // Conditions that close the expression
            Token::CloseParenthesis => {
                if bracket_nesting > 0 {
                    bracket_nesting -= 1;

                    // is empty tuple '()'
                    if bracket_nesting == 0 && expression.is_empty() {
                        return AstNode::Empty;
                    }

                    continue;
                }

                if inside_tuple {
                    *i -= 1;
                }

                break;
            }
            Token::EOF | Token::Newline | Token::SceneClose(_) => {
                if bracket_nesting == 0 {
                    break;
                }

                return AstNode::Error(
                    "Not enough closing parenthesis for expression. Need more ')' at the end of the expression!".to_string(),
                );
            }

            Token::Comma => {
                if inside_tuple {
                    break;
                }
                return new_tuple(tokens, i, AstNode::Expression(expression), ast);
            }

            // Check if name is a reference to another variable or function call
            Token::VarReference(id) => {
                expression.push(AstNode::VarReference(find_var_declaration_index(ast, id)));
            }
            Token::ConstReference(id) => {
                expression.push(AstNode::ConstReference(find_var_declaration_index(ast, id)));
            }

            // Check if is a literal
            Token::IntLiteral(int) => {
                expression.push(AstNode::Literal(Token::IntLiteral(*int)));
            }
            Token::StringLiteral(string) => {
                expression.push(AstNode::Literal(Token::StringLiteral(string.clone())));
            }
            Token::FloatLiteral(float) => {
                expression.push(AstNode::Literal(Token::FloatLiteral(*float)));
            }

            // OPERATORS
            // Assign precedence

            // UNARY OPERATORS
            Token::Negative => {
                expression.push(AstNode::UnaryOperator(Token::Negative, 10));
            }
            Token::Exponent => {
                expression.push(AstNode::UnaryOperator(Token::Exponent, 8));
            }

            // BINARY OPERATORS
            Token::Add => {
                expression.push(AstNode::BinaryOperator(Token::Add, 6));
            }
            Token::Subtract => {
                expression.push(AstNode::BinaryOperator(Token::Subtract, 6));
            }
            Token::Multiply => {
                expression.push(AstNode::BinaryOperator(Token::Multiply, 7));
            }
            Token::Divide => {
                expression.push(AstNode::BinaryOperator(Token::Divide, 7));
            }
            Token::AddAssign => {
                expression.push(AstNode::BinaryOperator(Token::AddAssign, 6));
            }
            Token::SubtractAssign => {
                expression.push(AstNode::BinaryOperator(Token::SubtractAssign, 6));
            }
            Token::Equal => {
                expression.push(AstNode::BinaryOperator(Token::Equal, 5));
            }
            Token::LessThan => {
                expression.push(AstNode::BinaryOperator(Token::LessThan, 5));
            }
            Token::LessThanOrEqual => {
                expression.push(AstNode::BinaryOperator(Token::LessThanOrEqual, 5));
            }
            Token::GreaterThan => {
                expression.push(AstNode::BinaryOperator(Token::GreaterThan, 5));
            }
            Token::GreaterThanOrEqual => {
                expression.push(AstNode::BinaryOperator(Token::GreaterThanOrEqual, 5));
            }
            Token::Modulus => {
                expression.push(AstNode::BinaryOperator(Token::Modulus, 7));
            }
            Token::Remainder => {
                expression.push(AstNode::BinaryOperator(Token::Remainder, 7));
            }
            Token::Root => {
                expression.push(AstNode::BinaryOperator(Token::Root, 8));
            }
            Token::ExponentAssign => {
                expression.push(AstNode::BinaryOperator(Token::ExponentAssign, 8));
            }
            Token::MultiplyAssign => {
                expression.push(AstNode::BinaryOperator(Token::MultiplyAssign, 7));
            }
            Token::DivideAssign => {
                expression.push(AstNode::BinaryOperator(Token::DivideAssign, 7));
            }
            Token::ModulusAssign => {
                expression.push(AstNode::BinaryOperator(Token::ModulusAssign, 7));
            }
            Token::RootAssign => {
                expression.push(AstNode::BinaryOperator(Token::RootAssign, 8));
            }
            Token::RemainderAssign => {
                expression.push(AstNode::BinaryOperator(Token::RemainderAssign, 7));
            }

            // LOGICAL OPERATORS
            Token::And => {
                expression.push(AstNode::BinaryOperator(Token::And, 4));
            }
            Token::Or => {
                expression.push(AstNode::BinaryOperator(Token::Or, 3));
            }

            _ => {
                expression.push(AstNode::Error(
                    "Invalid Expression, must be assigned wih a valid datatype".to_string(),
                ));
            }
        }

        *i += 1;
    }

    AstNode::Expression(expression)
}

// This function takes in an Expression node or Collection of expressions that has a Vec of Nodes to evaluate
// And evaluates everything possible at compile time (Constant Folding)
// If it returns a literal, then everything was evaluated at compile time
// Otherwise it will return an EvaluatedExpression, which has a strict type and will be evaluated at runtime
pub fn eval_expression(expr: AstNode, type_declaration: &DataType, ast: &Vec<AstNode>) -> AstNode {
    let mut current_type = type_declaration.to_owned();

    let mut simplified_expression = Vec::new();

    // TO DO: ACTUALLY IMPLIMENT CONSTANT FOLDING HERE!!!!!
    match expr {
        AstNode::Expression(e) => {
            for node in e {
                match node {
                    AstNode::Literal(t) => {
                        simplified_expression.push(check_literal(t, type_declaration, &mut current_type));
                    }
                    AstNode::BinaryOperator(op, precedence) => {
                        simplified_expression.push(AstNode::BinaryOperator(op, precedence));
                    }

                    AstNode::ConstReference(value) => {
                        match &ast[value] {
                            AstNode::VarDeclaration(_, assignment, _) | AstNode::Const(_, assignment, _) => {
                                let expr = *assignment.clone();

                                // Get the type and value of the original variable
                                match expr {
                                    AstNode::Literal(t) => {
                                        simplified_expression.push(check_literal(t, type_declaration, &mut current_type));
                                    }
                                    AstNode::EvaluatedExpression(e, expr_type) => {
                                        if current_type == DataType::Inferred || current_type != expr_type {
                                            return AstNode::Error("Error Mixing types. You must explicitly convert types to use them in the same expression".to_string());
                                        }
                                        simplified_expression.push(AstNode::EvaluatedExpression(e, expr_type));
                                    }
                                    _ => {
                                        return AstNode::Error("Invalid Expression".to_string());
                                    }
                                }
                            }
                            _ => {
                                println!("ConstReference not found in AST")
                            }
                        }
                    }

                    _ => {}
                }
            }
        }

        AstNode::Tuple(e) => {
            for node in e {
                match node {
                    AstNode::Expression(e) | AstNode::Tuple(e) => {
                        simplified_expression.push(eval_expression(AstNode::Expression(e), type_declaration, ast));
                    }
                    _ => {
                        simplified_expression.push(node);
                    }
                }
            }

            return AstNode::Tuple(simplified_expression);
        }
        _ => {
            return AstNode::Error("No Expression to Evaluate".to_string());
        }
    }

    if current_type == DataType::String {
        return concat_strings_if_adjacent(&mut simplified_expression);
    }

    if simplified_expression.len() == 1 {
        return simplified_expression[0].clone();
    }

    AstNode::EvaluatedExpression(simplified_expression, current_type)
}

fn concat_strings_if_adjacent(simplified_expression: &mut Vec<AstNode>) -> AstNode {
    let mut new_expr = Vec::new();
    let mut new_string = String::new();
    let mut previous_node_is_string = true;
    for node in simplified_expression {
        match node {
            AstNode::Literal(Token::StringLiteral(string)) => {
                if previous_node_is_string || new_string.is_empty() {
                    new_string.push_str(string);
                } else {
                    new_string.push_str(string);
                    new_expr.push(AstNode::Literal(Token::StringLiteral(new_string)));
                    new_string = string.clone();
                }
            }
            _ => {
                previous_node_is_string = false;
            }
        }
    }

    if new_expr.len() > 0 {
        AstNode::EvaluatedExpression(new_expr, DataType::String)
    } else {
        AstNode::Literal(Token::StringLiteral(new_string))
    }
}

fn check_literal(value: Token, type_declaration: &DataType, current_type: &mut DataType) -> AstNode {
    match value {
        Token::IntLiteral(_) => {
            if type_declaration == &DataType::Inferred {
                *current_type = DataType::Int;
                
            } else if type_declaration != &DataType::Int {
                return AstNode::Error("Error Mixing types. You must explicitly convert types to use them in the same expression".to_string());
            }
            AstNode::Literal(value)
        }
        Token::FloatLiteral(_) => {
            if type_declaration == &DataType::Inferred {
                *current_type = DataType::Float;
            } else if type_declaration != &DataType::Float {
                return AstNode::Error("Error Mixing types. You must explicitly convert types to use them in the same expression".to_string());
            }
            AstNode::Literal(value)
        }
        Token::StringLiteral(_) => {
            if type_declaration == &DataType::Inferred {
                *current_type = DataType::String;
            } else if type_declaration != &DataType::String {
                return AstNode::Error("Error Mixing types. You must explicitly convert types to use them in the same expression".to_string());
            }
            
            AstNode::Literal(value)
        }
        _ => {
            AstNode::Error("Invalid Literal (check_literal)".to_string())
        }
    }
}

/*
while there are tokens to be read:
    read a token
    if the token is:

    - a number:
        put it into the output queue


    - a function:
        push it onto the operator stack


    - an operator o1:
        while (
            there is an operator o2 at the top of the operator stack which is not a left parenthesis,
            and (o2 has greater precedence than o1 or (o1 and o2 have the same precedence and o1 is left-associative))
        ):
            pop o2 from the operator stack into the output queue
        push o1 onto the operator stack


    - a ",":
        while the operator at the top of the operator stack is not a left parenthesis:
             pop the operator from the operator stack into the output queue


    - a left parenthesis (i.e. "("):
        push it onto the operator stack


    - a right parenthesis (i.e. ")"):
        while the operator at the top of the operator stack is not a left parenthesis:
            {assert the operator stack is not empty}
            /* If the stack runs out without finding a left parenthesis, then there are mismatched parentheses. */
            pop the operator from the operator stack into the output queue
        {assert there is a left parenthesis at the top of the operator stack}
        pop the left parenthesis from the operator stack and discard it
        if there is a function token at the top of the operator stack, then:
            pop the function from the operator stack into the output queue

            After the while loop, pop the remaining items from the operator stack into the output queue.

while there are tokens on the operator stack:
If the operator token on the top of the stack is a parenthesis, then there are mismatched parentheses.
    {assert the operator on top of the stack is not a (left) parenthesis}
    pop the operator from the operator stack onto the output queue



*/

/*
    // Find the end of the expression and check if it is assigned a data type at the end
    let mut expression_end = *i;
    if bracket_nesting > 0 {
        // Find the last closing bracket and end expression there
        let mut total_open_brackets = bracket_nesting;
        while &expression_end < &tokens.len() {
            if &tokens[expression_end] == &Token::OpenParenthesis {
                total_open_brackets += 1;
            } else if &tokens[expression_end] == &Token::CloseParenthesis {
                if total_open_brackets < 1 {
                    break;
                }
                total_open_brackets -= 1;
            }

            expression_end += 1;
        }
    } else {
        // Find the next newline, comma or final closing bracket and end expression there
        while &expression_end < &tokens.len() {
            match &tokens[expression_end] {
                Token::Newline | Token::Comma | Token::SceneClose(_) | Token::CloseParenthesis => {
                    break;
                }
                _ => {
                    expression_end += 1;
                }
            }
        }
    }

*/
