#[derive(Clone)]
struct TeamMember {
    name: String,
    skills: Vec<String>,
    availability: bool,
    workload: u32,
}

#[derive(Clone)]
struct Task {
    name: String,
    required_skills: Vec<String>,
    priority_level: u32,
    assigned_to: Option<String>,
    status: String,
    dependencies: Vec<String>,
    deadline: Option<u32>,
}

fn assign_task(task: &mut Task, team_members: &mut Vec<TeamMember>) {
    let mut suitable_members: Vec<&mut TeamMember> = Vec::new();
    for member in team_members.iter_mut() {
        if task.required_skills.iter().all(|skill| member.skills.contains(skill))
            && member.availability
        {
            suitable_members.push(member);
        }
    }
    if !suitable_members.is_empty() {
        suitable_members.sort_by_key(|member| member.workload);
        if let Some(assigned_member) = suitable_members.first_mut() {
            assigned_member.workload += task.priority_level;
            task.assigned_to = Some(assigned_member.name.clone());
            task.status = "Assigned".to_string();
            assigned_member.availability = false;
        }
    } else {
        task.status = "Unassigned".to_string();
    }
}

fn allocate_tasks(tasks: &mut Vec<Task>, team_members: &mut Vec<TeamMember>) {
    let mut completed_tasks: Vec<String> = Vec::new();
    while completed_tasks.len() < tasks.len() {
        for task in tasks.iter_mut() {
            if task.status == "Unassigned" && task.dependencies.iter().all(|dep| completed_tasks.contains(&dep)) {
                assign_task(task, team_members);
                if task.status == "Assigned" {
                    if let Some(deadline) = task.deadline {
                        if let Some(assigned_to) = &task.assigned_to {
                            let assigned_member = team_members.iter_mut().find(|member| &member.name == assigned_to).unwrap();
                            let deadline_task = task.clone();
                            let mut deadline_member = assigned_member.clone();
                            std::thread::spawn(move || {
                                std::thread::sleep(std::time::Duration::from_secs(u64::from(deadline)));
                                println!("Deadline for task {} has passed", deadline_task.name);
                                deadline_member.availability = true;
                            });
                        }
                    }
                }
            }
            if task.status == "Assigned" {
                if let Some(assigned_to) = &task.assigned_to {
                    let assigned_member = team_members.iter_mut().find(|member| &member.name == assigned_to).unwrap();
                    if assigned_member.workload >= task.priority_level {
                        completed_tasks.push(task.name.clone());
                        assigned_member.workload -= task.priority_level;
                        assigned_member.availability = true;
                    }
                }
            }
        }
    }
}

fn main() {
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
