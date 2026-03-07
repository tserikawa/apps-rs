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
            format!("{}{}", self.expression.join(""), result.to_string())
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
            message: String::new(),
            result: None,
            done: false
        }
    }
}
