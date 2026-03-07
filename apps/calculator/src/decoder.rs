pub fn decode(expression: &str) -> Vec<String> {
        let mut output = Vec::new();
        
        let mut current = String::new();
        for c in expression.chars(){
            if c.is_digit(10){
                current.push(c);
            }else if ['+', '-', '*', '/'].contains(&c) {
                output.push(current.clone());
                current = String::new();
                output.push(c.to_string());
            }
        }
        
        if current.len() > 0 {
            output.push(current);
        }

        output
}