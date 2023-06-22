use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use regex::Regex;

trait Token {
    fn dbg(&self);
}

impl std::fmt::Debug for dyn Token {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.dbg();
        Ok(())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Class {
    access_modifier: Option<String>,
    name: Option<String>,
    properties: Vec<Property>,
    functions: Vec<Function>,
}
impl Token for Class {
    fn dbg(&self) {
        dbg!(self);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Property {
    access_modifier: Option<String>,
    data_type: Option<String>,
    name: Option<String>,
}
impl Token for Property {
    fn dbg(&self) {
        dbg!(self);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Function {
    access_modifier: Option<String>,
    return_type: Option<String>,
    name: Option<String>,
    parameters: Vec<String>,
}
impl Token for Function {
    fn dbg(&self) {
        dbg!(self);
    }
}

struct Tokenizer<'a> {
    words: Vec<&'a str>,
    class_stack: Vec<Class>,
    tokens: Vec<Box<dyn Token>>,
    equals: bool,
}

impl <'a> Tokenizer <'a> {

    fn new(words: Vec<&'a str>) -> Tokenizer<'a> {
        dbg!(&words);
        Tokenizer {
            words,
            class_stack: vec![],
            tokens: vec![],
            equals: false
        }
    }

    fn push_prop_to_stack(&mut self, property: Property) {
        match self.class_stack.last_mut() {
            Some(class) => {
                class.properties.push({
                    // dbg!(&property);
                    property
                }); 
                // dbg!(&class.properties);
            },
            None => self.tokens.push(Box::new({
                // dbg!(&property);
                property
            } ))
        }
    }

    fn push_func_to_stack(&mut self, function: Function) {
        match self.class_stack.last_mut() {
            Some(class) => class.functions.push({
                // dbg!(&function);
                function
            }),
            None => self.tokens.push(Box::new({ dbg!(&function); function} ))
        }
    }

    fn run (mut self) -> Vec<Box<dyn Token>> {
        while !self.words.is_empty() {
            // println!("of course we gonna run tokenize() again... we have {} words!!!!!!!!!!!!!! XDDDD", self.words.len());
            self.tokenize();
        }
        // dbg!(&self.tokens);
        self.tokens
    }

    fn tokenize (&mut self) {
        let access_modifier_regex: regex::Regex = Regex::new(r"^\s*(public|private|protected)").unwrap();

        let word = self.words.remove(0);
        // dbg!(word);

        let access_modifier = match access_modifier_regex.captures(word) {
            Some(capture) => Some(capture.get(1).unwrap().as_str().to_string()),
            None => None,
        };

        // println!("{}", self.words.len());

        if self.words.len() == 0 {
            return;
        }

        // next: class or type --- DECISION: class x (function or variable)
        match self.words[0] {

            // closes the class, removing it from the stack
            "}" => match self.class_stack.pop() {
                Some(class) => {
                    // println!("Adding class to tokens from stack: {:?}", class);
                    // dbg!(&class);
                    self.tokens.push(Box::new(class));
                    return;
                },
                None => println!("no class on stack")
            },

            // the "class" keyword. Identifies that the next token is the name of the class
            "class" => {
                // println!("match is class");
                self.words.remove(0);
                let class = Class {
                    access_modifier,
                    name: Some(self.words.remove(0).to_owned()),
                    properties: Vec::new(),
                    functions: Vec::new(),
                };
                self.class_stack.push(class);

                // match possible "extends" token here
            },

            // not "class", skip one token and match the next
            _=> match self.words[2] {
                
                // property
                ";" | "=" if { self.equals = true; true } => {
                    // println!("match is property");

                    // create the token
                    let token = Property {
                        access_modifier,
                        data_type: Some(self.words.remove(0).to_owned()),
                        name: Some(self.words.remove(0).to_owned()),
                    };

                    // if there's a class on the stack, push the token to the class.
                    // Otherwise, push the token to self.tokens
                    self.push_prop_to_stack(token);

                    // if match was "=", remove the next word until we find a ";"
                    if self.equals {
                        while !self.words.is_empty() && self.words[0] != ";" {
                            self.words.remove(0);
                        }
                        self.equals = false;
                    }
                },

                // function
                "(" => {
                    // println!("match is function");

                    // create the token
                    let mut function_token = Function {
                        access_modifier,
                        return_type: Some(self.words.remove(0).to_owned()),
                        name: Some(self.words.remove(0).to_owned()),
                        parameters: vec![],
                    };

                    self.words.remove(0);

                    let mut param_opt: Option<String> = None;
                    // dbg!(&parameter_token);
                    while !self.words.is_empty() {
                        // dbg!(self.words[0]);
                        match self.words.remove(0) {
                            ")" => {
                                if let Some(param_str) = param_opt {
                                    function_token.parameters.push(param_str.to_owned());
                                }
                                break;
                            },
                            "," => {
                                if let Some(param_str) = param_opt {
                                    function_token.parameters.push(param_str.to_owned());
                                }
                                param_opt = Some(self.words.remove(0).to_owned());
                            },
                            str_in_opt => {
                                match &mut param_opt {
                                    Some(param_str) => param_str.push_str(str_in_opt),
                                    None => param_opt = Some(str_in_opt.to_owned())
                                }
                            }
                        }
                    }

                    if self.words[0] == "{" {
                        let mut open_blocks = 1;
                        self.words.remove(0);
                        while open_blocks > 0 {
                            // dbg!(&self.words[0]);
                            match self.words.remove(0) {
                                "{" => open_blocks += 1,
                                "}" => open_blocks -= 1,
                                _ => {}
                            }
                        }
                    }

                    // if there's a class on the stack, push the token to the class.
                    // Otherwise, push the token to self.tokens
                    self.push_func_to_stack(function_token);

                },
                _=> {
                    println!("match is unknown: {} | {:?}", self.words[2], self.words);
                }
            }
        }
    }
}

fn main() {

    // let mut words = vec!["a", "b", "c", "d", "e"];
    // tokenize(&mut words);
    // dbg!(&words);

    // if true { return; }

    let file_path = "Example.java";

    // Open the file
    let mut file = File::open(file_path).expect("Failed to open file");

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Print the contents of the file
    println!("File contents:\n{}", contents);

    let re = Regex::new(r"\s+").unwrap();

    contents = re.replace_all(&contents, " ").to_string();

    println!("After regex:\n{}", contents);

    let re = Regex::new(r"(\w+|[^\w\s])").unwrap();
    let split: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();    

    let tokens = Tokenizer::new(split).run();

    dbg!(tokens);

}
