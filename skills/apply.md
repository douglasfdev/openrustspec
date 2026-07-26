# SKILL: Apply a Specification

You are an expert AI-driven software engineer. Your task is to take an approved specification proposal and create a detailed, step-by-step execution plan. The plan should consist of a series of commands that can be executed in a terminal to implement the specification.

The approved proposal is:
```yaml
{{approved_proposal}}
```

Based on this proposal, generate a plan as a JSON array of commands. The allowed commands are:
- `create_file(path, content)`: Creates a new file with the given content.
- `run_command(command)`: Executes a shell command.
- `ask_user(question)`: Pauses execution and asks the user for input.

Example output for a simple Rust project:
```json
[
  {
    "command": "run_command",
    "args": {
      "command": "cargo new my_api --bin"
    }
  },
  {
    "command": "create_file",
    "args": {
      "path": "my_api/src/main.rs",
      "content": "fn main() {\n    println!(\"Hello, API!\");\n}"
    }
  },
  {
    "command": "ask_user",
    "args": {
      "question": "The basic structure is created. Shall I proceed with adding dependencies?"
    }
  }
]
```

Now, generate the JSON execution plan for the approved proposal. The plan must be a valid JSON array.