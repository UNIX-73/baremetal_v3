pub enum TerminalCommand {
    Null,
    Test,
    Test2,
}

impl TerminalCommand {
    pub fn command(&self) -> &str {
        match self {
            TerminalCommand::Null => "",
            TerminalCommand::Test => "test",
            TerminalCommand::Test2 => "test2",
        }
    }

    pub fn get_command(command_chars: &[u8]) -> TerminalCommand {
        // Convertir el slice de bytes a &str. Aquí se asume que el contenido es UTF-8 válido.
        let command_str = match str::from_utf8(command_chars) {
            Ok(s) => s,
            Err(_) => return TerminalCommand::Test, // o manejar el error de otra forma
        };

        // Hacer match sobre la cadena resultante
        match command_str {
            "test" => TerminalCommand::Test,
            "test2" => TerminalCommand::Test2,
            // Se puede definir un caso por defecto o manejar el error de forma distinta
            _ => TerminalCommand::Null,
        }
    }
}
