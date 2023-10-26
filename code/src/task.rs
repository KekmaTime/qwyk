// task.rs

use crate::TeamMember;

// Define the Task struct and its associated functions in a module.
#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub required_skills: Vec<String>,
    pub priority_level: u32,
    pub assigned_to: Option<String>,
    pub status: String,
    pub dependencies: Vec<String>,
    pub deadline: Option<u32>,
}

// Define the assign_task and allocate_tasks functions.
pub fn assign_task(task: &mut Task, team_members: &mut Vec<TeamMember>) {
    let mut suitable_members: Vec<&mut TeamMember> = Vec::new();
    for member in team_members.iter_mut() {
        if task
            .required_skills
            .iter()
            .all(|skill| member.skills.contains(skill))
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

pub fn allocate_tasks(tasks: &mut Vec<Task>, team_members: &mut Vec<TeamMember>) {
    let mut completed_tasks: Vec<String> = Vec::new();
    while completed_tasks.len() < tasks.len() {
        for task in tasks.iter_mut() {
            if task.status == "Unassigned"
                && task
                    .dependencies
                    .iter()
                    .all(|dep| completed_tasks.contains(&dep))
            {
                assign_task(task, team_members);
                if task.status == "Assigned" {
                    if let Some(deadline) = task.deadline {
                        if let Some(assigned_to) = &task.assigned_to {
                            let assigned_member = team_members
                                .iter_mut()
                                .find(|member| &member.name == assigned_to)
                                .unwrap();
                            let deadline_task = task.clone();
                            let mut deadline_member = assigned_member.clone();
                            std::thread::spawn(move || {
                                std::thread::sleep(std::time::Duration::from_secs(u64::from(
                                    deadline,
                                )));
                                println!("Deadline for task {} has passed", deadline_task.name);
                                deadline_member.availability = true;
                            });
                        }
                    }
                }
            }
            if task.status == "Assigned" {
                if let Some(assigned_to) = &task.assigned_to {
                    let assigned_member = team_members
                        .iter_mut()
                        .find(|member| &member.name == assigned_to)
                        .unwrap();
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
