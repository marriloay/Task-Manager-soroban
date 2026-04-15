#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::Ledger, Env};

/// Helper: deploy contract and return (env, client).
fn setup() -> (Env, TaskManagerContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    // Set a concrete ledger timestamp so `created_at` is deterministic.
    env.ledger().with_mut(|l| {
        l.timestamp = 1_700_000_000;
    });

    let contract_id = env.register(TaskManagerContract, ());
    let client = TaskManagerContractClient::new(&env, &contract_id);
    (env, client)
}

// ─────────────────────────────────────────────────────────────────────────────
//  get_tasks
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_get_tasks_initially_empty() {
    let (_env, client) = setup();
    assert_eq!(client.get_tasks().len(), 0);
}

// ─────────────────────────────────────────────────────────────────────────────
//  create_task
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_create_task_returns_id() {
    let (env, client) = setup();

    let id = client.create_task(
        &String::from_str(&env, "Buy groceries"),
        &String::from_str(&env, "Milk, eggs, bread"),
        &Priority::Low,
    );

    // ID must be non-zero (extremely unlikely to be zero from PRNG).
    // We just check the list grew.
    assert_eq!(client.get_tasks().len(), 1);
    let task = client.get_tasks().get(0).unwrap();
    assert_eq!(task.id, id);
    assert_eq!(task.status, Status::Pending);
    assert_eq!(task.priority, Priority::Low);
    assert_eq!(task.created_at, 1_700_000_000);
}

#[test]
fn test_create_multiple_tasks() {
    let (env, client) = setup();

    client.create_task(
        &String::from_str(&env, "Task A"),
        &String::from_str(&env, "Desc A"),
        &Priority::High,
    );
    client.create_task(
        &String::from_str(&env, "Task B"),
        &String::from_str(&env, "Desc B"),
        &Priority::Medium,
    );

    assert_eq!(client.get_tasks().len(), 2);
}

// ─────────────────────────────────────────────────────────────────────────────
//  update_status
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_update_status_pending_to_in_progress() {
    let (env, client) = setup();

    let id = client.create_task(
        &String::from_str(&env, "Write tests"),
        &String::from_str(&env, "Cover all branches"),
        &Priority::High,
    );

    let msg = client.update_status(&id, &Status::InProgress);
    assert_eq!(msg, String::from_str(&env, "Task status updated successfully"));

    let task = client.get_tasks().get(0).unwrap();
    assert_eq!(task.status, Status::InProgress);
}

#[test]
fn test_update_status_nonexistent_id() {
    let (env, client) = setup();

    let msg = client.update_status(&9999_u64, &Status::Completed);
    assert_eq!(msg, String::from_str(&env, "Task not found"));
}

// ─────────────────────────────────────────────────────────────────────────────
//  update_priority
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_update_priority() {
    let (env, client) = setup();

    let id = client.create_task(
        &String::from_str(&env, "Deploy contract"),
        &String::from_str(&env, "To testnet"),
        &Priority::Low,
    );

    let msg = client.update_priority(&id, &Priority::High);
    assert_eq!(msg, String::from_str(&env, "Task priority updated successfully"));

    let task = client.get_tasks().get(0).unwrap();
    assert_eq!(task.priority, Priority::High);
}

// ─────────────────────────────────────────────────────────────────────────────
//  delete_task
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_delete_task() {
    let (env, client) = setup();

    let id = client.create_task(
        &String::from_str(&env, "Temporary task"),
        &String::from_str(&env, "Will be deleted"),
        &Priority::Medium,
    );

    assert_eq!(client.get_tasks().len(), 1);

    let msg = client.delete_task(&id);
    assert_eq!(msg, String::from_str(&env, "Task deleted successfully"));
    assert_eq!(client.get_tasks().len(), 0);
}

#[test]
fn test_delete_nonexistent_task() {
    let (env, client) = setup();

    let msg = client.delete_task(&42_u64);
    assert_eq!(msg, String::from_str(&env, "Task not found"));
}

// ─────────────────────────────────────────────────────────────────────────────
//  get_tasks_by_status
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_filter_by_status() {
    let (env, client) = setup();

    let id1 = client.create_task(
        &String::from_str(&env, "Task 1"),
        &String::from_str(&env, ""),
        &Priority::Low,
    );
    client.create_task(
        &String::from_str(&env, "Task 2"),
        &String::from_str(&env, ""),
        &Priority::High,
    );

    client.update_status(&id1, &Status::Completed);

    let completed = client.get_tasks_by_status(&Status::Completed);
    let pending = client.get_tasks_by_status(&Status::Pending);

    assert_eq!(completed.len(), 1);
    assert_eq!(pending.len(), 1);
    assert_eq!(completed.get(0).unwrap().id, id1);
}

// ─────────────────────────────────────────────────────────────────────────────
//  get_tasks_by_priority
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_filter_by_priority() {
    let (env, client) = setup();

    client.create_task(
        &String::from_str(&env, "High prio task"),
        &String::from_str(&env, ""),
        &Priority::High,
    );
    client.create_task(
        &String::from_str(&env, "Low prio task"),
        &String::from_str(&env, ""),
        &Priority::Low,
    );

    let high = client.get_tasks_by_priority(&Priority::High);
    let low = client.get_tasks_by_priority(&Priority::Low);
    let medium = client.get_tasks_by_priority(&Priority::Medium);

    assert_eq!(high.len(), 1);
    assert_eq!(low.len(), 1);
    assert_eq!(medium.len(), 0);
}

// ─────────────────────────────────────────────────────────────────────────────
//  clear_completed
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_clear_completed() {
    let (env, client) = setup();

    let id1 = client.create_task(
        &String::from_str(&env, "Done 1"),
        &String::from_str(&env, ""),
        &Priority::Low,
    );
    let id2 = client.create_task(
        &String::from_str(&env, "Done 2"),
        &String::from_str(&env, ""),
        &Priority::Medium,
    );
    client.create_task(
        &String::from_str(&env, "Still pending"),
        &String::from_str(&env, ""),
        &Priority::High,
    );

    client.update_status(&id1, &Status::Completed);
    client.update_status(&id2, &Status::Completed);

    let removed = client.clear_completed();
    assert_eq!(removed, 2);
    assert_eq!(client.get_tasks().len(), 1);
    assert_eq!(
        client.get_tasks().get(0).unwrap().status,
        Status::Pending
    );
}
