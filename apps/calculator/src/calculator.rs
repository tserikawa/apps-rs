pub struct Calculator {
    expression: Vec<String>,
    message: String
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            expression: Vec::new(),
            message: String::new()
        }
    }

    pub fn add_term(&mut self, term: &str) {
        self.message = String::new();
        self.expression.push(term.to_string());
    }

    pub fn display_text(&self) -> String {
        self.expression.join("")
    }

    pub fn clear(&mut self) {
        self.expression.clear();
    }

    pub fn set_message(&mut self, message: &str) {
        self.message = String::new();
        self.message = message.to_string();
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn result(&self) -> Result<f32, Box<dyn std::error::Error>>{
        if self.expression.len() != 4 {
            Err("Invalid Term Count".into())
        }else{
            let first = self.expression[0].parse::<f32>()?;
            let operator = &self.expression[1];
            let second = self.expression[2].parse::<f32>()?;
            match operator.as_str() {
                "+" => Ok(first + second),
                "-" => Ok(first - second),
                "*" => Ok(first * second),
                _ => Ok(first / second)
            }
        }
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator {
            expression: Vec::new(),
            message: String::new()
        }
    }
}
