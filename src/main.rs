use std::collections::BTreeMap;

use zellij_tile::prelude::*;

struct State {
    permissions_granted: bool,
    match_commands: Vec<String>,
    action: Option<(Direction, u8)>,
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
        match event {
            Event::ListClients(clients) => {
                let Some(command) = clients
                    .iter()
                    .find(|c| c.is_current_client && !c.running_command.is_empty())
                    .map(|c| {
                        c.running_command
                            .trim()
                            .split_whitespace()
                            .collect::<Vec<_>>()[0]
                            .split('/')
                            .last()
                            .unwrap_or("")
                            .to_string()
                    })
                else {
                    return false;
                };
                let Some(action) = self.action else {
                    return false;
                };
                if self.match_commands.contains(&command) {
                    // forward to nvim
                    write(vec![action.1]);
                } else {
                    // send to zellij
                    if action.0 == Direction::Left || action.0 == Direction::Right {
                        move_focus_or_tab(action.0);
                    } else {
                        move_focus(action.0);
                    }
                }
                self.action = None;
            }
            _ => {}
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
        let Some(Ok(key)) = pipe_message.payload.map(|p| p.parse::<u8>()) else {
            return false;
        };
        self.action = Some((direction, key));
        list_clients();
        false
    }
}
