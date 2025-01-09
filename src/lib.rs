use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use zed::{Extension, SlashCommand, SlashCommandOutput, Worktree};
use zed_extension_api as zed;

struct MakefileTaskGenerator;

impl Extension for MakefileTaskGenerator {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        if command.name == "generate-tasks" {
            if let Some(worktree) = worktree {
                let root_path = PathBuf::from(worktree.root_path());
                let makefile_path = root_path.join("Makefile");

                if makefile_path.exists() {
                    match worktree.read_text_file("Makefile") {
                        Ok(makefile_content) => {
                            let tasks = parse_makefile(&makefile_content);
                            match serde_json::to_string_pretty(&tasks) {
                                Ok(tasks_json) => {
                                    let tasks_path = root_path.join(".tasks.json");
                                    if let Err(e) = fs::write(tasks_path, tasks_json) {
                                        return Err(format!("Failed to write .tasks.json: {}", e));
                                    }
                                    return Ok(SlashCommandOutput {
                                        text: ".tasks.json generated successfully.".to_string(),
                                        sections: vec![],
                                    });
                                }
                                Err(e) => {
                                    return Err(format!(
                                        "Failed to serialize tasks to JSON: {}",
                                        e
                                    ));
                                }
                            }
                        }
                        Err(e) => {
                            return Err(format!("Failed to read Makefile: {}", e));
                        }
                    }
                } else {
                    return Err("Makefile not found in the current project.".to_string());
                }
            } else {
                return Err("No worktree available.".to_string());
            }
        }
        Err("Unknown command.".to_string())
    }
}

zed::register_extension!(MakefileTaskGenerator);

#[derive(Serialize)]
struct Task {
    label: String,
    command: String,
}

fn parse_makefile(content: &str) -> Vec<Task> {
    content
        .lines()
        .filter_map(|line| {
            if let Some((target, _)) = line.split_once(':') {
                let target = target.trim();
                if !target.is_empty() && !target.contains(' ') && !target.starts_with('.') {
                    return Some(Task {
                        label: target.to_string(),
                        command: format!("make {}", target),
                    });
                }
            }
            None
        })
        .collect()
}
