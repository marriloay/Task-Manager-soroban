#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

// ─────────────────────────────────────────────────────────────────────────────
//  Data Types
// ─────────────────────────────────────────────────────────────────────────────

/// Priority level for a task.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

/// Lifecycle status of a task.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

/// A single task entry stored on-chain.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub status: Status,
    pub created_at: u64, // ledger timestamp (seconds)
}

// ─────────────────────────────────────────────────────────────────────────────
//  Storage Keys
// ─────────────────────────────────────────────────────────────────────────────

const TASK_DATA: Symbol = symbol_short!("TASK_DATA");

// ─────────────────────────────────────────────────────────────────────────────
//  Contract
// ─────────────────────────────────────────────────────────────────────────────

#[contract]
pub struct TaskManagerContract;

#[contractimpl]
impl TaskManagerContract {
    // ── Read ──────────────────────────────────────────────────────────────────

    /// Return all tasks stored on-chain.
    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env))
    }

    /// Return only tasks that match the given status.
    pub fn get_tasks_by_status(env: Env, status: Status) -> Vec<Task> {
        let all: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        let mut filtered: Vec<Task> = Vec::new(&env);
        for i in 0..all.len() {
            let task = all.get(i).unwrap();
            if task.status == status {
                filtered.push_back(task);
            }
        }
        filtered
    }

    /// Return only tasks that match the given priority.
    pub fn get_tasks_by_priority(env: Env, priority: Priority) -> Vec<Task> {
        let all: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        let mut filtered: Vec<Task> = Vec::new(&env);
        for i in 0..all.len() {
            let task = all.get(i).unwrap();
            if task.priority == priority {
                filtered.push_back(task);
            }
        }
        filtered
    }

    // ── Write ─────────────────────────────────────────────────────────────────

    /// Create and store a new task.
    ///
    /// Returns the generated task ID so callers can reference it later.
    pub fn create_task(
        env: Env,
        title: String,
        description: String,
        priority: Priority,
    ) -> u64 {
        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        let id: u64 = env.prng().gen::<u64>();

        let task = Task {
            id,
            title,
            description,
            priority,
            status: Status::Pending,          // every new task starts as Pending
            created_at: env.ledger().timestamp(),
        };

        tasks.push_back(task);
        env.storage().instance().set(&TASK_DATA, &tasks);

        id
    }

    /// Update the status of an existing task identified by `id`.
    pub fn update_status(env: Env, id: u64, new_status: Status) -> String {
        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();
            if task.id == id {
                task.status = new_status;
                tasks.set(i, task);
                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Task status updated successfully");
            }
        }

        String::from_str(&env, "Task not found")
    }

    /// Update the priority of an existing task identified by `id`.
    pub fn update_priority(env: Env, id: u64, new_priority: Priority) -> String {
        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();
            if task.id == id {
                task.priority = new_priority;
                tasks.set(i, task);
                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Task priority updated successfully");
            }
        }

        String::from_str(&env, "Task not found")
    }

    /// Permanently remove a task identified by `id`.
    pub fn delete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i);
                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Task deleted successfully");
            }
        }

        String::from_str(&env, "Task not found")
    }

    /// Remove every task whose status is `Completed` in one call.
    pub fn clear_completed(env: Env) -> u32 {
        let tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        let mut remaining: Vec<Task> = Vec::new(&env);
        let mut removed: u32 = 0;

        for i in 0..tasks.len() {
            let task = tasks.get(i).unwrap();
            if task.status == Status::Completed {
                removed += 1;
            } else {
                remaining.push_back(task);
            }
        }

        env.storage().instance().set(&TASK_DATA, &remaining);
        removed // return count of removed tasks
    }
}


mod test;
