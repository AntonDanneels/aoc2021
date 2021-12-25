use std::iter::Peekable;

#[derive(Debug)]
enum InstructionType {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug)]
enum Operand {
    Register { index: usize },
    Value { val: i64 },
}

#[derive(Debug)]
struct Instruction {
    instr_type: InstructionType,
    left: Operand,
    right: Operand,
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    registers: [i64; 4],
}

fn parse_program<'a, I: Iterator<Item = &'a str>>(data: &mut Peekable<I>) -> Option<Program> {
    let input = data.next()?;
    let mut result = Program {
        instructions: Vec::new(),
        registers: [0; 4],
    };

    if !input.starts_with("inp") {
        return None;
    }

    while let Some(x) = data.peek() {
        if x.starts_with("inp") {
            break;
        }
        let instr = data.next()?.split(" ").collect::<Vec<&str>>();
        let instr_type = match instr[0] {
            "add" => InstructionType::Add,
            "mul" => InstructionType::Mul,
            "div" => InstructionType::Div,
            "mod" => InstructionType::Mod,
            "eql" => InstructionType::Eql,
            _ => return None,
        };
        let left = match instr[1] {
            "x" => Operand::Register { index: 0 },
            "y" => Operand::Register { index: 1 },
            "z" => Operand::Register { index: 2 },
            "w" => Operand::Register { index: 3 },
            i => Operand::Value {
                val: i.parse().unwrap(),
            },
        };
        let right = match instr[2] {
            "x" => Operand::Register { index: 0 },
            "y" => Operand::Register { index: 1 },
            "z" => Operand::Register { index: 2 },
            "w" => Operand::Register { index: 3 },
            i => Operand::Value {
                val: i.parse().unwrap(),
            },
        };

        result.instructions.push(Instruction {
            instr_type,
            left,
            right,
        });
    }

    Some(result)
}

fn get_register(op: &Operand) -> usize {
    match op {
        Operand::Register { index } => *index,
        Operand::Value { val: _ } => panic!(),
    }
}

fn get_value(op: &Operand, program: &Program) -> i64 {
    match op {
        Operand::Register { index } => program.registers[*index],
        Operand::Value { val } => *val,
    }
}

fn execute(program: &mut Program) {
    for instr in program.instructions.iter() {
        //println!("{:?}", instr);
        match instr.instr_type {
            InstructionType::Add => {
                program.registers[get_register(&instr.left)] += get_value(&instr.right, program);
            }
            InstructionType::Mul => {
                program.registers[get_register(&instr.left)] *= get_value(&instr.right, program);
            }
            InstructionType::Div => {
                program.registers[get_register(&instr.left)] /= get_value(&instr.right, program);
            }
            InstructionType::Mod => {
                program.registers[get_register(&instr.left)] %= get_value(&instr.right, program);
            }
            InstructionType::Eql => {
                program.registers[get_register(&instr.left)] =
                    match get_value(&instr.right, program)
                        == program.registers[get_register(&instr.left)]
                    {
                        true => 1,
                        false => 0,
                    };
            }
        };
    }
}

fn main() {
    let mut data = include_str!("input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .peekable();

    let mut programs = Vec::new();

    while let Some(program) = parse_program(&mut data) {
        programs.push(program);
    }

    let mut registers = [0; 4];

    let mut registers = [0; 4];
    for (j, mut program) in programs.iter_mut().enumerate() {
        registers[3] = 9;
        for (i, val) in registers.iter().enumerate() {
            program.registers[i] = *val;
        }
        execute(&mut program);
        for (i, val) in program.registers.iter().enumerate() {
            registers[i] = *val;
        }
    }
    println!("End: {:?}", registers);

    /*
    for i in 1..10 {
        programs[0].registers = [0; 4];
        programs[0].registers[3] = i;
        //println!("{:?}", programs[0].registers);
        execute(&mut programs[0]);
        //println!("P1: {:?}", programs[0].registers);
        for j in 1..10 {
            programs[1].registers = programs[0].registers;
            programs[1].registers[3] = j;
            execute(&mut programs[1]);
            //println!("P2: {:?}", programs[1].registers);
            for k in 1..10 {
                programs[2].registers = programs[1].registers;
                programs[2].registers[3] = j;
                execute(&mut programs[2]);
                println!("P2: {:?}", programs[1].registers);
            }
        }
    }
    */
}
