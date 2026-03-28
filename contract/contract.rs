#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, String, Symbol, symbol_short};

// Mapping Task to its unique ID
#[contracttype] 
pub enum DataKey { 
    Task(u64)
}

// For creating unique IDs for newly created tasks.
const TASK_COUNT: Symbol = symbol_short!("T_COUNT"); 

// Structure representing a single Task on the chain
#[contracttype]
#[derive(Clone)] 
pub struct Task {
    pub id: u64,
    pub description: String,
    pub is_completed: bool,   
}  

#[contract]
pub struct TaskManagerChain;

#[contractimpl]
impl TaskManagerChain {

    // 1. Function to create a new task
    pub fn create_task(env: Env, description: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&TASK_COUNT).unwrap_or(0);
        count += 1;
        
        let new_task = Task {
            id: count.clone(),
            description: description,
            is_completed: false,
        };
        
        // Store the new task on the ledger
        env.storage().instance().set(&DataKey::Task(count.clone()), &new_task);
        // Update the task counter
        env.storage().instance().set(&TASK_COUNT, &count);
        
        // Extend Time-to-Live to prevent early archival
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Task Created on Chain with ID: {}", count);
        return count; 
    }

    // 2. Function to mark a task as completed
    pub fn complete_task(env: Env, task_id: u64) {
        let key = DataKey::Task(task_id.clone());
        
        // Retrieve the task, panics if it doesn't exist
        let mut task: Task = env.storage().instance().get(&key).expect("Task does not exist on chain!");
        
        if task.is_completed == false {
            task.is_completed = true;
            
            // Save the updated task back to the ledger
            env.storage().instance().set(&key, &task);
            env.storage().instance().extend_ttl(5000, 5000);
            
            log!(&env, "Task ID: {} is now marked as Completed", task_id);
        } else {
            log!(&env, "Task is already completed!");
            panic!("Task is already completed!");
        }
    }

    // 3. Function to view a task's details from the chain
    pub fn get_task(env: Env, task_id: u64) -> Task {
        let key = DataKey::Task(task_id.clone()); 
        
        env.storage().instance().get(&key).unwrap_or(Task {
            id: 0,
            description: String::from_str(&env, "Not_Found"),
            is_completed: false, 
        })
    }
}