// use math_parse::MathParse;

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
pub fn create_expression(
    tokens: &Vec<Token>,
    i: &mut usize,
    inside_tuple: bool,
    ast: &Vec<AstNode>,
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
            // Will push as a string so shunting yard can handle it later just as a string

            // UNARY OPERATORS
            Token::Negative => {
                expression.push(AstNode::Operator(" -".to_string()));
            }
            Token::Exponent => {
                expression.push(AstNode::Operator(" ** ".to_string()));
            }

            // BINARY OPERATORS
            Token::Add => {
                expression.push(AstNode::Operator(" + ".to_string()));
            }
            Token::Subtract => {
                expression.push(AstNode::Operator(" - ".to_string()));
            }
            Token::Multiply => {
                expression.push(AstNode::Operator(" * ".to_string()));
            }
            Token::Divide => {
                expression.push(AstNode::Operator(" / ".to_string()));
            }
            Token::Modulus => {
                expression.push(AstNode::Operator(" % ".to_string()));
            }
            Token::Remainder => {
                expression.push(AstNode::Operator(" %% ".to_string()));
            }
            Token::Root => {
                expression.push(AstNode::Operator(" // ".to_string()));
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
    let mut compile_time_eval = true;

    match expr {
        AstNode::Expression(e) => {
            for node in e {
                match node {
                    AstNode::Literal(t) => {
                        simplified_expression.push(check_literal(
                            t,
                            type_declaration,
                            &mut current_type,
                        ));
                    }
                    // AstNode::LogicalOperator(op, precedence) => {
                    //     simplified_expression.push(AstNode::LogicalOperator(op, precedence));
                    // }
                    AstNode::Operator(op) => {
                        // If the current type is a string, then must be a + operator or create an error
                        if current_type == DataType::String && op != " + " {
                            return AstNode::Error("Can only use the + operator to manipulate strings inside string expressions".to_string());
                        }
                        if simplified_expression.len() < 1 {
                            return AstNode::Error(
                                "Must have a value to the left of an operator".to_string(),
                            );
                        }
                        simplified_expression.push(AstNode::Operator(op));
                    }
                    AstNode::ConstReference(value) | AstNode::VarReference(value) => {
                        compile_time_eval = false;
                        match &ast[value] {
                            AstNode::VarDeclaration(_, assignment, _)
                            | AstNode::Const(_, assignment, _) => {
                                let expr = *assignment.clone();

                                // Get the type and value of the original variable
                                match expr {
                                    AstNode::Literal(t) => {
                                        simplified_expression.push(check_literal(
                                            t,
                                            type_declaration,
                                            &mut current_type,
                                        ));
                                    }
                                    AstNode::RuntimeExpression(e, expr_type) => {
                                        if current_type == DataType::Inferred
                                            || current_type != expr_type
                                        {
                                            return AstNode::Error("Error Mixing types. You must explicitly convert types to use them in the same expression".to_string());
                                        }
                                        simplified_expression
                                            .push(AstNode::RuntimeExpression(e, expr_type));
                                    }
                                    _ => {
                                        return AstNode::Error("Invalid Expression".to_string());
                                    }
                                }
                            }
                            _ => {
                                return AstNode::Error("Reference not found in AST".to_string());
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
                        simplified_expression.push(eval_expression(
                            AstNode::Expression(e),
                            type_declaration,
                            ast,
                        ));
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
            AstNode::Operator(_) => {
                // Should always be a plus operator, this is enforced in the eval_expression function
                previous_node_is_plus = true;
            }
            _ => {
                return AstNode::Error("Cannot evaluate string expression at compile time. Compiler should be creating a runtime string expression".to_string());
            }
        }
    }

    AstNode::Literal(Token::StringLiteral(new_string))
}

fn check_literal(
    value: Token,
    type_declaration: &DataType,
    current_type: &mut DataType,
) -> AstNode {
    if type_declaration == &DataType::CoerseToString {
        return AstNode::Literal(value);
    }
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
        _ => AstNode::Error("Invalid Literal (check_literal)".to_string()),
    }
}
