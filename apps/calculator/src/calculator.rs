use crate::decoder::decode;

pub struct Calculator {
    expression: Vec<String>,
    result: Option<f32>,
    message: String,
    pub done: bool,
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
        } else {
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

    pub fn result(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let terms = decode(self.expression.join("").as_str());
        if terms.len() != 3 {
            Err(format!("Invalid Term Count. {}", terms.join(",")).into())
        } else {
            let first = terms[0].parse::<f32>()?;
            let operator = &terms[1];
            let second = terms[2].parse::<f32>()?;
            match operator.as_str() {
                "+" => Ok(first + second),
                "-" => Ok(first - second),
                "*" => Ok(first * second),
                _ =>  {
                    match second {
                        0.0 => Err("0 division".into()),
                        _ => Ok(first / second)
                    }
                },
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
            done: false,
        }
    }
}
