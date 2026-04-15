# 📋 Soroban Task Manager

A **priority-aware task management** smart contract deployed on the **Stellar testnet** using [Soroban SDK](https://docs.stellar.org/docs/smart-contracts). Unlike a simple notes app, this contract tracks tasks through a complete lifecycle with priority levels and status filtering.

---

## 🚀 Live Contract on Testnet

| Field | Value |
|---|---|
| **Network** | Stellar Testnet |
| **Contract ID** | `CBXXX...` *(replace after deploying — see Deploy section)* |
| **Explorer** | [stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet) |

> **Note:** Build and deploy the contract using the steps in the [Deploy](#-deploy-to-testnet) section, then replace the Contract ID above with the one printed to your terminal.

---

## ✨ Features

This contract is **different from a basic notes app** in the following ways:

| Feature | Notes App (reference) | Task Manager (this contract) |
|---|---|---|
| Data model | `id, title, content` | `id, title, description, **priority**, **status**, created_at` |
| Priority system | ❌ | ✅ Low / Medium / High |
| Status lifecycle | ❌ | ✅ Pending → InProgress → Completed |
| Filter by status | ❌ | ✅ `get_tasks_by_status` |
| Filter by priority | ❌ | ✅ `get_tasks_by_priority` |
| Bulk cleanup | ❌ | ✅ `clear_completed` |
| Mutable fields | ❌ | ✅ Update status & priority separately |

### Contract Functions

| Function | Arguments | Returns | Description |
|---|---|---|---|
| `get_tasks` | — | `Vec<Task>` | Returns all tasks |
| `get_tasks_by_status` | `status: Status` | `Vec<Task>` | Filter tasks by lifecycle status |
| `get_tasks_by_priority` | `priority: Priority` | `Vec<Task>` | Filter tasks by priority level |
| `create_task` | `title, description, priority` | `u64` (task id) | Create a new task (starts as Pending) |
| `update_status` | `id, new_status` | `String` | Advance or change a task's status |
| `update_priority` | `id, new_priority` | `String` | Change a task's priority |
| `delete_task` | `id` | `String` | Permanently delete one task |
| `clear_completed` | — | `u32` | Remove all Completed tasks, returns count |

---

## 🗂️ Data Types

```rust
pub enum Priority { Low, Medium, High }

pub enum Status { Pending, InProgress, Completed }

pub struct Task {
    pub id:          u64,      // PRNG-generated unique ID
    pub title:       String,   // short task name
    pub description: String,   // detail / acceptance criteria
    pub priority:    Priority,
    pub status:      Status,
    pub created_at:  u64,      // ledger timestamp (seconds since Unix epoch)
}
```

---

## 🏗️ Project Structure

```
soroban-task-manager/
├── Cargo.toml          # dependencies & build profiles
└── src/
    ├── lib.rs           # contract logic
    └── test.rs          # unit tests (11 test cases)
```

---

## 🔧 Prerequisites

- [Rust](https://rustup.rs/) + `wasm32-unknown-unknown` target  
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli/install-cli)

```bash
# Install Rust toolchain target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

---

## 🧪 Run Tests

```bash
cd soroban-task-manager
cargo test
```

Expected output (11 tests):

```
running 11 tests
test test::test_get_tasks_initially_empty         ... ok
test test::test_create_task_returns_id            ... ok
test test::test_create_multiple_tasks             ... ok
test test::test_update_status_pending_to_in_progress ... ok
test test::test_update_status_nonexistent_id      ... ok
test test::test_update_priority                   ... ok
test test::test_delete_task                       ... ok
test test::test_delete_nonexistent_task           ... ok
test test::test_filter_by_status                  ... ok
test test::test_filter_by_priority                ... ok
test test::test_clear_completed                   ... ok
test result: ok. 11 passed; 0 failed
```

---

## 🚀 Deploy to Testnet

```bash
# 1. Build the optimised WASM binary
stellar contract build

# 2. Generate or import a Stellar keypair
stellar keys generate --global alice --network testnet

# 3. Fund the account on testnet faucet
stellar keys fund alice --network testnet

# 4. Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_task_manager.wasm \
  --network testnet \
  --source alice
# → prints CONTRACT_ID — paste it in README above
```

---

## 💻 Invoke via CLI

```bash
# Set env shorthand
export CONT=<YOUR_CONTRACT_ID>
export NET="--network testnet --source alice"

# Create a High-priority task
stellar contract invoke $CONT $NET -- create_task \
  --title "Finish assignment" \
  --description "Submit before midnight" \
  --priority '{"High":{}}'

# Move task to InProgress (replace <TASK_ID> with the returned u64)
stellar contract invoke $CONT $NET -- update_status \
  --id <TASK_ID> \
  --new_status '{"InProgress":{}}'

# List all pending tasks
stellar contract invoke $CONT $NET -- get_tasks_by_status \
  --status '{"Pending":{}}'

# Get all High priority tasks
stellar contract invoke $CONT $NET -- get_tasks_by_priority \
  --priority '{"High":{}}'

# Delete a single task
stellar contract invoke $CONT $NET -- delete_task --id <TASK_ID>

# Remove every Completed task at once
stellar contract invoke $CONT $NET -- clear_completed
```

---

## 📄 License

MIT — see [LICENSE](./LICENSE) for details.
