use bevy::prelude::{Commands, In};

pub fn handle_io_errors(In(result): In<std::io::Result<()>>, mut commands: Commands) {
    if let Err(e) = result {
        eprintln!("I/O error occurred: {}", e.to_string());
        commands.spawn((/* ... */));
    }
}
