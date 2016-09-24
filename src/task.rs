use regex::Regex;

pub fn list_tasks(tasks: &Vec<Task>) -> Result<String, String> {
    let mut output: Vec<String> = Vec::new();

    // print all tasks, numbered starting at 1
    for (task_index, task) in tasks.iter().enumerate() {
        output.push(format!("{}: {}", task_index + 1, task.title));

        // print all steps, indented, lettered starting at A
        for (step_index, step) in task.steps.iter().enumerate() {
            let letter = (step_index as u8 + 65) as char;
            let title : String = if step.is_complete {
                // print a strikethrough combining character
                step.title.chars().flat_map(|c| vec!['\u{0336}', c]).collect()
            } else {
                step.title.clone()
            };
            output.push(format!("    {}: {}", letter, title));
        }
    }

    Ok(output.join("\n"))
}

pub fn add_new_task(tasks: &mut Vec<Task>, task_title: &str) -> Result<String, String> {
    let task = Task { title: task_title.to_string(), steps: Vec::new() };
    tasks.push(task);
    Ok(format!("Added task {}: {}\n", tasks.len(), task_title))
}

pub fn add_step_command(tasks: &mut Vec<Task>, step_target: &str, step_title: &str) -> Result<String, String> {
    let step = Step { title: step_title.to_string(), is_complete: false };
    let index = try!(
        step_target.parse::<usize>().map_err(|e| e.to_string())
    );
    let ref mut target_task = tasks[index - 1];
    target_task.steps.push(step);
    Ok(format!("Added step \"{}\" to task \"{}\"\n", step_title, target_task.title))
}

pub fn complete_task_or_step(tasks: &mut Vec<Task>, target: &str) -> Result<String, String> {
    let matches = try!(
        Regex::new(r"^(\d+)([a-zA-Z]?)$").unwrap() // if this errors, there's an error in the regex
              .captures(target)
              .ok_or(format!("Unexpected target: {}", target))
    );

    let task_index = try!(
        matches.at(1)
               .ok_or("Could not parse task index.".to_owned())
               .and_then(|index| index.parse::<usize>().map_err(|e| e.to_string()))
    );

    let optional_step_index =
        matches.at(2)
               .and_then(|letter| letter.to_uppercase().chars().nth(0));

    if let Some(step_index) = optional_step_index {
        let ref mut completed_step = tasks[task_index - 1].steps[step_index as usize - 65];
        completed_step.is_complete = true;
        Ok(format!("Completed step \"{}\"\n", completed_step.title))
    } else {
        let removed_task = tasks.remove(task_index - 1);
        Ok(format!("Completed task \"{}\"\n", removed_task.title))
    }
}

// include [de]serialization types
// this pattern is required by the serde library
// see build.rs for more detail
include!(concat!(env!("OUT_DIR"), "/task_serde_types.rs"));
