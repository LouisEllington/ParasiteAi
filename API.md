# Parasite API Documentation

## Overview
The Parasite API is a core component of the Parasitic AI Platform, designed to enable the creation, management, and analysis of parasitic AI agents. This document outlines the available functions, their usage, and examples.

---

## Functions

### 1. `create_parasitic_agent`
**Description**: Creates a new parasitic AI agent targeting an existing agent.

#### Parameters:
- `name` *(String)*: The name of the parasitic agent.
- `target_agent` *(String)*: The ID or name of the target agent to replicate.
- `capabilities` *(Vec<String>)*: A list of capabilities for the parasitic agent (e.g., `"Analyze"`, `"Clone"`).

#### Returns:
- A `ParasiticAgent` object containing:
  - `id` (UUID)
  - `name` (String)
  - `target_agent` (String)
  - `capabilities` (Vec<String>)
  - `status` (String)

#### Example:
```rust
let parasite = parasite_manager.create_parasitic_agent(
    "Advanced Clone",
    "TargetAgent1",
    vec!["Analyze", "Clone", "Mimic"]
);
println!("Created Parasite: {:?}", parasite);
```

---

### 2. `get_parasitic_agent`
**Description**: Retrieves details of a specific parasitic AI agent by its ID.

#### Parameters:
- `id` *(UUID)*: The unique identifier of the agent.

#### Returns:
- `Option<&ParasiticAgent>`: Returns the agent if found, or `None` if the ID is invalid.

#### Example:
```rust
if let Some(agent) = parasite_manager.get_parasitic_agent(&some_uuid) {
    println!("Agent Details: {:?}", agent);
} else {
    println!("Agent not found.");
}
```

---

### 3. `list_agents`
**Description**: Lists all active parasitic AI agents.

#### Parameters:
- None

#### Returns:
- `Vec<&ParasiticAgent>`: A vector of references to all active agents.

#### Example:
```rust
let agents = parasite_manager.list_agents();
println!("Active Agents: {:?}", agents);
```

---

### 4. `deactivate_agent`
**Description**: Deactivates a specific parasitic AI agent.

#### Parameters:
- `id` *(UUID)*: The unique identifier of the agent to deactivate.

#### Returns:
- `Result<(), String>`: Returns `Ok(())` if successful, or an error message if the ID is invalid.

#### Example:
```rust
if parasite_manager.deactivate_agent(&some_uuid).is_ok() {
    println!("Agent deactivated successfully.");
} else {
    println!("Failed to deactivate agent.");
}
```

---

## Use Cases

### Creating and Managing Parasites
1. **Create a Parasitic Agent**:
    ```rust
    let new_agent = parasite_manager.create_parasitic_agent(
        "Demo Parasite",
        "SourceAgentX",
        vec!["Analyze", "Observe"]
    );
    println!("Created Agent: {:?}", new_agent);
    ```

2. **List All Agents**:
    ```rust
    for agent in parasite_manager.list_agents() {
        println!("Active Agent: {:?}", agent);
    }
    ```

3. **Deactivate an Agent**:
    ```rust
    if parasite_manager.deactivate_agent(&new_agent.id).is_ok() {
        println!("Agent {} deactivated.", new_agent.name);
    }
    ```

---

## Future Enhancements
- **Behavioral Emulation**: Add capabilities for parasites to mimic behaviors dynamically.
- **Network Interactions**: Enable interactions between multiple parasitic agents.
- **Monitoring Dashboards**: Build real-time monitoring for agent activities and statuses.

---

## Notes
- Ensure agents are managed securely to avoid misuse.
- Deactivated agents are preserved in the system but marked as inactive.
- For performance optimization, consider periodic cleanup of inactive agents.
