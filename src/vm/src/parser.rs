use crate::instruction::*;

#[derive(Debug)]
pub struct Command<'a> {
    pub inst: Instruction,
    pub raw: String,
    pub module_name: &'a str,
}

#[derive(Debug)]
pub struct ParseResult<'a> {
    pub commands: Vec<Command<'a>>,
    pub filename: String,
}

pub struct Parser<'a> {
    input: &'a str,
    filename: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, filename: &'a str) -> Self {
        Parser { input, filename }
    }

    pub fn parse(&self) -> ParseResult {
        let mut commands = Vec::<Command>::new();
        for line in self.input.lines() {
            if let Some(inst) = Self::parse_line(line) {
                commands.push(Command {
                    inst,
                    raw: line.into(),
                    module_name: self.get_module_name(),
                })
            }
        }
        ParseResult { commands: commands, filename: self.filename.into() }
    }

    fn parse_line(line: &str) -> Option<Instruction> {
        let cleaned = line.split("//").nth(0).unwrap_or_default().trim();
        let cmds: Vec<&str> = cleaned.split_whitespace().collect();
        let result = match cmds[..] {
            ["function", label, n_args] => Some(Instruction::Function(label.into(), str::parse::<usize>(n_args).ok()?)),
            ["return"] => Some(Instruction::Return()),
            ["label", label] => Some(Instruction::Label(label.into())),
            ["goto", label] => Some(Instruction::Goto(label.into())),
            ["if-goto", label] => Some(Instruction::IfGoto(label.into())),
            [arith] => Some(Instruction::Arithmetic(arith.into())),
            [cmd1, cmd2, cmd3] => match cmd1 {
                "push" | "pop" => {
                    let inst_type = match cmd1 {
                        "push" => PushPop::Push,
                        "pop" => PushPop::Pop,
                        _ => unreachable!(),
                    };
                    Some(Instruction::PushPop(PushPopInstruction {
                        segment: cmd2.into(),
                        addr: str::parse::<u16>(&cmd3).ok()?,
                        inst_type,
                    }))
                }
                _ => None,
            },
            _ => None,
        };
        if result.is_none() && !cleaned.is_empty() {
            eprintln!("Unable to parse line: {}", line);
        }
        result
    }

    fn get_module_name(&self) -> &str {
        self.filename
    }
}

pub fn create<'a>(content: &'a str, filename: &'a str) -> Parser<'a> {
    let parser = Parser::new(content, filename);
    parser
}
