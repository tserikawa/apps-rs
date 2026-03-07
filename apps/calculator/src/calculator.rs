pub struct Calculator {
    expression: Vec<String>,
    result: Option<f32>,
    message: String,
    pub done: bool
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            expression: Vec::new(),
            message: String::new(),
            result: None,
            done: false,
        }
    }

    pub fn add_term(&mut self, term: &str) {
        self.reset();
        self.message = String::new();
        self.expression.push(term.to_string());
    }

    pub fn display_text(&self) -> String {
        if let Some(result) = self.result {
            format!("{}={}", self.expression.join(""), result.to_string())
        }else {
            self.expression.join("")
        }
    }

    pub fn clear(&mut self) {
        self.expression.clear();
    }

    pub fn reset(&mut self) {
        if self.done {
            self.clear();
            self.result = None;
            self.done = false;
        }
    }

    pub fn set_message(&mut self, message: &str) {
        self.message = String::new();
        self.message = message.to_string();
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn set_result(&mut self, value: f32) {
        self.done = true;
        self.result = Some(value);
    }

    pub fn result(& self) -> Result<f32, Box<dyn std::error::Error>>{
        let terms = self.decode();
        if terms.len() != 3 {
            Err(format!("Invalid Term Count. {}", terms.join(",")).into())
        }else{
            let first = terms[0].parse::<f32>()?;
            let operator = &terms[1];
            let second = terms[2].parse::<f32>()?;
            match operator.as_str() {
                "+" => Ok(first + second),
                "-" => Ok(first - second),
                "*" => Ok(first * second),
                _ => Ok(first / second)
            }
        }
    }

    fn decode(&self) -> Vec<String> {
        let mut output = Vec::new();
        
        let expression = self.expression.join("");
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
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator {
            expression: Vec::new(),
            message: String::new(),
            result: None,
            done: false
        }
    }
}
