
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Represents a Parasitic AI Agent
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParasiticAgent {
    pub id: Uuid,
    pub name: String,
    pub target_agent: String,
    pub capabilities: Vec<String>,
    pub status: String,
}

/// Manages Parasitic AI Agents
pub struct ParasiteManager {
    agents: HashMap<Uuid, ParasiticAgent>,
}

impl ParasiteManager {
    /// Creates a new ParasiteManager
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Creates a new Parasitic AI Agent
    pub fn create_parasitic_agent(&mut self, name: &str, target_agent: &str, capabilities: Vec<String>) -> ParasiticAgent {
        let agent = ParasiticAgent {
            id: Uuid::new_v4(),
            name: name.to_string(),
            target_agent: target_agent.to_string(),
            capabilities,
            status: "Active".to_string(),
        };
        self.agents.insert(agent.id, agent.clone());
        agent
    }

    /// Retrieves a Parasitic AI Agent by ID
    pub fn get_parasitic_agent(&self, id: &Uuid) -> Option<&ParasiticAgent> {
        self.agents.get(id)
    }

    /// Lists all Parasitic AI Agents
    pub fn list_agents(&self) -> Vec<&ParasiticAgent> {
        self.agents.values().collect()
    }

    /// Deactivates a Parasitic AI Agent
    pub fn deactivate_agent(&mut self, id: &Uuid) -> Result<(), String> {
        if let Some(agent) = self.agents.get_mut(id) {
            agent.status = "Deactivated".to_string();
            Ok(())
        } else {
            Err("Agent not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parasitic_agent_management() {
        let mut manager = ParasiteManager::new();

        let agent = manager.create_parasitic_agent("Test Parasite", "TargetAgent1", vec!["Clone", "Analyze"]);
        assert_eq!(agent.name, "Test Parasite");

        let fetched_agent = manager.get_parasitic_agent(&agent.id).unwrap();
        assert_eq!(fetched_agent.name, "Test Parasite");

        assert_eq!(manager.list_agents().len(), 1);

        manager.deactivate_agent(&agent.id).unwrap();
        let deactivated_agent = manager.get_parasitic_agent(&agent.id).unwrap();
        assert_eq!(deactivated_agent.status, "Deactivated");
    }
}
