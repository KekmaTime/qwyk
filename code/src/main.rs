// main.rs

// Import the modules you'll be using in the main program.
mod task;
mod team;

use task::{allocate_tasks, Task};
use team::TeamMember;

fn main() {
    // Your main program logic remains unchanged.
    let mut team_members = vec![
        TeamMember {
            name: "Alice".to_string(),
            skills: vec!["Rust".to_string(), "Python".to_string()],
            availability: true,
            workload: 0,
        },
        TeamMember {
            name: "Bob".to_string(),
            skills: vec!["Java".to_string(), "C++".to_string()],
            availability: true,
            workload: 0,
        },
        TeamMember {
            name: "Charlie".to_string(),
            skills: vec!["Python".to_string(), "JavaScript".to_string()],
            availability: true,
            workload: 0,
        },
    ];

    let mut tasks = vec![
        Task {
            name: "Implement feature X".to_string(),
            required_skills: vec!["Rust".to_string(), "Python".to_string()],
            priority_level: 2,
            assigned_to: None,
            status: "Unassigned".to_string(),
            dependencies: vec![],
            deadline: None,
        },
        Task {
            name: "Fix bug Y".to_string(),
            required_skills: vec!["Java".to_string(), "C++".to_string()],
            priority_level: 1,
            assigned_to: None,
            status: "Unassigned".to_string(),
            dependencies: vec![],
            deadline: Some(5),
        },
        Task {
            name: "Write tests for module Z".to_string(),
            required_skills: vec!["Python".to_string(), "JavaScript".to_string()],
            priority_level: 3,
            assigned_to: None,
            status: "Unassigned".to_string(),
            dependencies: vec!["Implement feature X".to_string()],
            deadline: None,
        },
    ];

    allocate_tasks(&mut tasks, &mut team_members);

    for task in tasks.iter() {
        println!("Task: {}", task.name);
        match &task.assigned_to {
            Some(name) => println!("Assigned to: {}", name),
            None => println!("Assigned to: None"),
        }
        println!("Status: {}", task.status);
        println!();
    }
}
