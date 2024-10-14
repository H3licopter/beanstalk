// use math_parse::MathParse;

use colour::red_ln;

use super::{ast_nodes::AstNode, collections::new_tuple, create_scene_node::new_scene};
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
) -> AstNode {
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
    let mut next_number_negative = false;
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

                break;
            }

            Token::EOF | Token::SceneClose(_) | Token::Arrow => {
                if bracket_nesting != 0 {
                    return AstNode::Error(
                        "Not enough closing parenthesis for expression. Need more ')' at the end of the expression!".to_string(),
                        starting_line_number.to_owned(),
                    );
                }

                *i -= 1;
                break;
            }

            Token::Newline => {
                // Fine if inside of brackets (not closed yet)
                // Otherwise break out of the expression
                if bracket_nesting > 0 {
                    continue;
                } else {
                    break;
                }
            }

            Token::Comma => {
                if inside_tuple {
                    break;
                }
                return new_tuple(tokens, i, AstNode::Expression(expression, starting_line_number.to_owned()), ast, starting_line_number);
            }

            // Check if name is a reference to another variable or function call
            Token::VarReference(id) => {
                expression.push(AstNode::VarReference(id.to_string()));
            }
            Token::ConstReference(id) => {
                expression.push(AstNode::ConstReference(id.to_string()));
            }

            // Check if is a literal
            Token::FloatLiteral(mut float) => {
                if data_type != &DataType::Float && data_type != &DataType::Inferred {
                    return AstNode::Error("Float literal used in non-float expression".to_string(), starting_line_number.to_owned());
                }
                if next_number_negative {float = -float; next_number_negative = false;}
                expression.push(AstNode::Literal(Token::FloatLiteral(float)));
            }
            Token::StringLiteral(string) => {
                if data_type != &DataType::String && data_type != &DataType::Inferred {
                    return AstNode::Error("String literal used in non-string expression".to_string(), starting_line_number.to_owned());
                }
                expression.push(AstNode::Literal(Token::StringLiteral(string.clone())));
            }

            // Scenes - Create a new scene node
            // Maybe scenes can be added together like strings
            Token::SceneHead => {
                if data_type != &DataType::Scene && data_type != &DataType::Inferred {
                    return AstNode::Error("Scene used in non-scene expression".to_string(), starting_line_number.to_owned());
                }
                expression.push(new_scene(tokens, i, &ast, starting_line_number));
            }

            // OPERATORS
            // Will push as a string so shunting yard can handle it later just as a string
            Token::Negative => {
                next_number_negative = true;
            }

            // BINARY OPERATORS
            Token::Add => {
                expression.push(AstNode::BinaryOperator(token.to_owned()));
            }
            Token::Subtract => {
                if data_type != &DataType::Float && data_type != &DataType::Inferred {
                    return AstNode::Error("Subtraction used in non-float expression".to_string(), starting_line_number.to_owned());
                }
                expression.push(AstNode::BinaryOperator(token.to_owned()));
            }
            Token::Multiply => {
                if data_type != &DataType::Float && data_type != &DataType::Inferred {
                    return AstNode::Error("Multiplication used in non-float expression".to_string(), starting_line_number.to_owned());
                }
                expression.push(AstNode::BinaryOperator(token.to_owned()));
            }
            Token::Divide => {
                if data_type != &DataType::Float && data_type != &DataType::Inferred {
                    return AstNode::Error("Division used in non-float expression".to_string(), starting_line_number.to_owned());
                }
                expression.push(AstNode::BinaryOperator(token.to_owned()));
            }
            Token::Modulus => {
                if data_type != &DataType::Float && data_type != &DataType::Inferred {
                    return AstNode::Error("Modulus used in non-float expression".to_string(), starting_line_number.to_owned());
                }
                expression.push(AstNode::BinaryOperator(token.to_owned()));
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
                    "Invalid Expression, must be assigned wih a valid datatype".to_string(),
                    starting_line_number.to_owned(),
                ));
            }
        }

        *i += 1;
    }

    return evaluate_expression(AstNode::Expression(expression, starting_line_number.to_owned()), data_type, ast);
}

// This function takes in an Expression node or Collection of expressions that has a Vec of Nodes to evaluate
// And evaluates everything possible at compile time (Constant Folding)
// If it returns a literal, then everything was evaluated at compile time
// Otherwise it will return an EvaluatedExpression, which has a strict type and will be evaluated at runtime
pub fn evaluate_expression(expr: AstNode, type_declaration: &DataType, ast: &Vec<AstNode>) -> AstNode {
    let mut current_type = type_declaration.to_owned();
    let mut simplified_expression = Vec::new();
    let mut compile_time_eval = true;

    match expr {
        AstNode::Expression(e, line_number) => {
            for node in e {
                match node {
                    AstNode::Literal(t) => {
                        simplified_expression.push(check_literal(
                            t,
                            type_declaration,
                            &mut current_type,
                            line_number
                        ));
                    }
                    AstNode::Scene(_, _, _, _) => {
                        simplified_expression.push(node);
                    }
                    AstNode::BinaryOperator(ref op) => {
                        // If the current type is a string, then must be a + operator or create an error
                        if current_type == DataType::String && *op != Token::Add {
                            return AstNode::Error(
                                "Can only use the + operator to manipulate strings inside string expressions".to_string(),
                                line_number
                            );
                        }
                        if simplified_expression.len() < 1 {
                            return AstNode::Error(
                                "Must have a value to the left of an operator".to_string(),
                                line_number
                            );
                        }
                        simplified_expression.push(node);
                    }

                    // Eventually should be unpacking the const value
                    AstNode::ConstReference(_) | AstNode::VarReference(_) => {
                        compile_time_eval = false;
                        simplified_expression.push(node)
                    }

                    _ => {}
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

    // If nothing to evaluate at compile time
    if simplified_expression.len() == 1 {
        return simplified_expression[0].clone();
    }
    // If the expression is a string, then either return a string or a runtime expression
    if current_type == DataType::String && compile_time_eval {
        return concat_strings(&mut simplified_expression);
    }

    // Maths expression constant folding will go here eventually
    // Will need to evaluate anything possible in the expression at compiletime
    // For now, just have the whole the expression evaluated at runtime
    AstNode::RuntimeExpression(simplified_expression, current_type)
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
                }
            }
            AstNode::BinaryOperator(_) => {
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

fn check_literal(
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
        _ => AstNode::Error("Invalid Literal (check_literal)".to_string(),
            line_number
        ),
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
        _ => {
            *i -= 1;
            false
        }
    }
}
