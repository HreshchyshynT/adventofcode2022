// use std::{
//     collections::{HashMap, VecDeque},
//     io::{BufRead, Result},
// };

// use crate::utils;

// #[derive(Debug, Clone)]
// struct Directory {
//     name: String,
//     files: Vec<FsEntity>,
// }
// #[derive(Debug, Clone)]
// struct File {
//     name: String,
//     size: u32,
// }

// impl FsEntity {
//     fn is_dir(&self) -> bool {
//         match self {
//             Directory => true,
//             File => false,
//         }
//     }

//     fn size(&self) -> u32 {
//         match self {
//             FsEntity::Directory { name: _, files } => {
//                 let mut total: u32 = 0;
//                 for entity in files.iter() {
//                     total += entity.size();
//                 }
//                 total
//             }
//             FsEntity::File { name: _, size } => size.clone(),
//         }
//     }
// }

// #[derive(Debug)]
// enum Command {
//     Cd { path: String },
//     Ls,
// }

// impl From<String> for Command {
//     fn from(line: String) -> Self {
//         let split: Vec<&str> = line.split(' ').collect();
//         match split[1] {
//             "cd" => Command::Cd {
//                 path: split[2].to_string(),
//             },
//             "ls" => Command::Ls,
//             _ => panic!("unknown comamnd: {}", split[1]),
//         }
//     }
// }

// impl From<String> for FsEntity {
//     fn from(line: String) -> Self {
//         if line.starts_with("dir") {
//             let split: Vec<&str> = line.split(' ').collect();
//             FsEntity::Directory {
//                 name: split[1].to_string(),
//                 files: vec![],
//             }
//         } else {
//             let split: Vec<&str> = line.split(' ').collect();
//             FsEntity::File {
//                 name: split[1].to_string(),
//                 size: split[0].parse().unwrap(),
//             }
//         }
//     }
// }

// #[derive(Debug)]
// enum Input {
//     Cmd(Command),
//     Entity(FsEntity),
// }

// impl From<String> for Input {
//     fn from(line: String) -> Self {
//         if line.starts_with('$') {
//             Input::Cmd(line.into())
//         } else {
//             Input::Entity(line.into())
//         }
//     }
// }

// pub fn run() -> Result<()> {
//     let reader = utils::read_file("./inputs/test7.txt")?;
//     let mut inputs: Vec<Input> = vec![];
//     let mut dirs: HashMap<String, FsEntity> = HashMap::new();
//     let mut cds: VecDeque<Command> = VecDeque::new();
//     for line in reader.lines().map(|l| l.unwrap()) {
//         let input: Input = line.into();
//         match input {
//             Input::Cmd(command) => match command {
//                 Command::Cd { path: _ } => cds.push_front(command),
//                 Command::Ls => {}
//             },
//             Input::Entity(entity) => match entity {
//                 FsEntity::Directory { name, files } => {
//                     // how to pass entity here?
//                     dirs.insert(name.clone(), FsEntity::Directory { name, files });
//                 }
//                 FsEntity::File { name: _, size: _ } => if let Some(cmd) = cds.front() {},
//             },
//         }
//     }
//     for i in inputs {
//         println!("{:?}", i);
//     }
//     Ok(())
// }

// fn handle_cd(cds: &mut VecDeque<Command>, cd: Command) {
//     if let Command::Cd { path } = cd {
//         match path.as_str() {
//             ".." => {
//                 cds.pop_front();
//             }
//             _ => {
//                 cds.push_front(Command::Cd { path });
//             }
//         };
//     }
// }

// fn add_file_to_dir(dirs: &HashMap<String, FsEntity>, file: FsEntity, top_cd: Command) {
//     let cd = match top_cd {
//         Command::Cd { path: _ } => top_cd,
//         _ => return,
//     };
// }
