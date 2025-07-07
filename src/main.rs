use std::collections::BTreeMap;

use zellij_tile::prelude::*;

struct State {
    permissions_granted: bool,
    match_commands: Vec<String>,
    action: Option<(Direction, Vec<u8>)>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            permissions_granted: false,
            match_commands: vec!["vim".to_string(), "nvim".to_string()],
            action: None,
        }
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::WriteToStdin,
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::ListClients]);
        if let Some(match_commands) = configuration.get("match_commands") {
            self.match_commands = match_commands
                .split(",")
                .map(|s| s.trim().to_string())
                .collect();
        }
        if self.permissions_granted {
            hide_self();
        }
    }

    fn update(&mut self, event: Event) -> bool {
        if let Event::ListClients(clients) = event {
            let Some(command) = clients
                .iter()
                .find(|c| c.is_current_client && !c.running_command.is_empty())
                .map(|c| {
                    c.running_command.split_whitespace().collect::<Vec<_>>()[0]
                        .split('/')
                        .next_back()
                        .unwrap_or("")
                        .to_string()
                })
            else {
                return false;
            };
            let Some(action) = self.action.take() else {
                return false;
            };
            if self.match_commands.contains(&command) {
                // forward to nvim
                write(action.1);
            } else {
                // send to zellij
                if action.0 == Direction::Left || action.0 == Direction::Right {
                    move_focus_or_tab(action.0);
                } else {
                    move_focus(action.0);
                }
            }
        }
        false
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        let direction = match &*pipe_message.name {
            "nvim_nav_left" => Direction::Left,
            "nvim_nav_right" => Direction::Right,
            "nvim_nav_up" => Direction::Up,
            "nvim_nav_down" => Direction::Down,
            _ => {
                return false;
            }
        };
        let Some(payload) = pipe_message.payload else {
            return false;
        };

        let bytes: Result<Vec<u8>, _> =
            payload.split(',').map(|s| s.trim().parse::<u8>()).collect();

        let Ok(key_sequence) = bytes else {
            return false;
        };

        if key_sequence.is_empty() {
            return false;
        }

        self.action = Some((direction, key_sequence));
        list_clients();
        false
    }
}
