pub struct QueryParser<'a> {
    pub operators: &'a [&'a str; 3],
}

// Would be interesting to create a BooleanQueryParser
// and maybe others ?
// Must think harder on how to organise code
impl<'a> QueryParser<'a> {
    pub fn new() -> Self {
        QueryParser {
            operators: &["AND", "OR", "NOT"],
        }
    }

    pub fn tokenize_parenthesis(tokens: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::with_capacity(tokens.len());
        let parenthesis: &[_] = &['(', ')'];
        for token in tokens {
            if token.contains(parenthesis) {
                for c in token.chars() {
                    if c == '(' {
                        result.push(c.to_string());
                    }
                }
                result.push(token.trim_matches(parenthesis).to_string());
                for c in token.chars() {
                    if c == ')' {
                        result.push(c.to_string());
                    }
                }
            } else {
                result.push(token);
            }
        }
        result
    }

    pub fn to_postfix(&self, query: &str) -> Option<Vec<String>> {
        let unwanted: &[_] = &[' ', '\t', '\n'];
        let trimmed = query.trim_matches(unwanted);
        let mut tokens: Vec<String> = trimmed
            .split_terminator(' ')
            .map(|s| s.to_string())
            .collect();
        tokens.retain(|e| !e.is_empty());
        let tokens = Self::tokenize_parenthesis(tokens);
        let mut op_stack: Vec<String> = Vec::new();
        let mut res: Vec<String> = Vec::with_capacity(tokens.len());
        for token in tokens {
            if self.operators.contains(&&token[..]) {
                while op_stack.len() > 0
                    && ((op_stack.last().unwrap() == "NOT" && token != "NOT")
                        || &token == op_stack.last().unwrap())
                    && op_stack.last().unwrap() != "("
                {
                    let op = op_stack.pop().unwrap();
                    res.push(op);
                }
                op_stack.push(token);
            } else if token == "(" {
                op_stack.push(token);
            } else if token == ")" {
                if op_stack.len() == 0 {
                    return None;
                }
                let mut found_left_par = false;
                while let Some(op) = op_stack.pop() {
                    if op == "(" {
                        found_left_par = true;
                        break;
                    } else {
                        res.push(op);
                    }
                }
                if !found_left_par {
                    return None;
                }
            } else {
                res.push(token);
            }
        }
        if op_stack.len() > 0 {
            while let Some(op) = op_stack.pop() {
                res.push(op);
            }
        }
        Some(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_to_postfix() {
        let qp = QueryParser::new();
        assert_eq!(
            qp.to_postfix("boat AND time"),
            Some(vec![
                "boat".to_string(),
                "time".to_string(),
                "AND".to_string()
            ])
        );
        assert_eq!(
            qp.to_postfix("boat OR time"),
            Some(vec![
                "boat".to_string(),
                "time".to_string(),
                "OR".to_string()
            ])
        );
        assert_eq!(
            qp.to_postfix("NOT boat"),
            Some(vec!["boat".to_string(), "NOT".to_string()])
        );
        assert_eq!(
            qp.to_postfix("NOT boat AND time"),
            Some(vec![
                "boat".to_string(),
                "NOT".to_string(),
                "time".to_string(),
                "AND".to_string()
            ])
        );
        assert_eq!(
            qp.to_postfix("     \t \n   NOT    (boat    AND    time)\n\t\t\t"),
            Some(vec![
                "boat".to_string(),
                "time".to_string(),
                "AND".to_string(),
                "NOT".to_string(),
            ])
        );
        assert_eq!(
            qp.to_postfix("tent AND NOT (blood AND sweat) OR tree"),
            Some(vec![
                "tent".to_string(),
                "blood".to_string(),
                "sweat".to_string(),
                "AND".to_string(),
                "NOT".to_string(),
                "tree".to_string(),
                "OR".to_string(),
                "AND".to_string(),
            ])
        );
    }
}
