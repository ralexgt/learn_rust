// NOTE using trait objects
trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

struct CreateTable;
impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create the table"
    }
    fn rollback(&self) -> &str {
        "drop the table"
    }
}

struct AddField;
impl Migration for AddField {
    fn execute(&self) -> &str {
        "add a field"
    }
    fn rollback(&self) -> &str {
        "drop a field"
    }
}

#[derive(Default)]
struct Schema {
    commands: Vec<Box<dyn Migration>>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev() // reverse iterator's direction
            .map(|cmd| cmd.rollback())
            .collect()
    }
}

// NOTE using function pointers
type FnPtr = fn() -> String;
struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

struct SchemaFncs {
    commands: Vec<Command>,
}

impl SchemaFncs {
    fn new() -> Self {
        Self { commands: vec![] }
    }
    fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command { execute, rollback });
    }
    fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }
    fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

fn add_field() -> String {
    String::from("Add a field")
}

fn create_table() -> String {
    String::from("Create a table")
}

// NOTE implementing Fn trait objects

type Migration2<'a> = Box<dyn Fn() -> &'a str>;

struct Schema2<'a> {
    executes: Vec<Migration2<'a>>,
    rollbacks: Vec<Migration2<'a>>,
}

impl<'a> Schema2<'a> {
    fn new() -> Self {
        Self {
            executes: vec![],
            rollbacks: vec![],
        }
    }
    fn add_migration<E, R>(&mut self, execute: E, rollback: R)
    where
        E: Fn() -> &'a str + 'static,
        R: Fn() -> &'a str + 'static,
    {
        self.executes.push(Box::new(execute));
        self.rollbacks.push(Box::new(rollback));
    }
    fn execute(&self) -> Vec<&str> {
        self.executes.iter().map(|cmd| cmd()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
    }
}

fn add_field_2() -> &'static str {
    "Add a field"
}

fn create_table_2() -> &'static str {
    "Create a table"
}
fn main() {
    let mut schema = Schema::new();

    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);

    assert_eq!(vec!["create the table", "add a field"], schema.execute());
    assert_eq!(vec!["drop a field", "drop the table"], schema.rollback());

    let mut schema = SchemaFncs::new();
    schema.add_migration(create_table, || "Drop table".to_string());
    schema.add_migration(add_field, || "Drop field".to_string());

    assert_eq!(vec!["Create a table", "Add a field"], schema.execute());
    assert_eq!(vec!["Drop field", "Drop table"], schema.rollback());

    let mut schema = Schema2::new();
    schema.add_migration(create_table_2, || "Drop table");
    schema.add_migration(add_field_2, || "Drop field");

    assert_eq!(vec!["Create a table", "Add a field"], schema.execute());
    assert_eq!(vec!["Drop field", "Drop table"], schema.rollback());
}
