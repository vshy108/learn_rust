//! # Rustfinity: Plugin System
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/plugin-system
//!
//! We now have an overview of traits, how to define them and how they work, now it's time to put that knowledge to use and build a plugin system using traits.
//!
//! A plugin in this system is any type that implements a specific trait. Each plugin will perform a specific task, and the system should manage a collection of these plugins, executing them in sequence. You’ll also address advanced issues like object safety and resolving potential conflicts between overlapping trait implementations.
//!
//! ## Your Task
//!
//! Design and implement a plugin system using trait objects. You will:
//!
//! 1. Define a `Plugin` trait that includes methods for initialization and execution.
//! 2. Create a `PluginManager` struct to manage plugins. It should:
//!    - Dynamically load plugins implementing the `Plugin` trait.
//!    - Allow adding and removing plugins.
//!    - Execute all registered plugins in sequence.
//!
//! ### Requirements
//!
//! - The `Plugin` trait should include the following methods:
//!   - `fn name(&self) -> &str;` - Returns the name of the plugin.
//!   - `fn execute(&self);` - Executes the plugin's functionality.
//! - The `PluginManager` should:
//!   - Have the following methods and associated functions:
//!     - `new() -> Self` - Creates a new `PluginManager` instance.
//!     - `add_plugin` - Adds a plugin to the list.
//!     - `remove_plugin` - Removes a plugin from the list and returns the removed plugin if found.
//!     - `execute_all` - Executes all registered plugins.
//! - If a duplicate plugin is added (with the same name), it should **panic** with the message "Plugin with name [name] already exists".
//!
//! Make sure you make all relevant items public.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - **Dynamic Dispatch**: Store plugins in a `Vec<Box<dyn Plugin>>` for dynamic dispatch.
//! - **Plugin Uniqueness**: Use the `name` method to identify and ensure uniqueness among plugins.
//!
//! </details>
//!
//! Run: `cargo run --example rf_plugin_system`
//! Test: `cargo test --example rf_plugin_system`
//! Filter: `cargo test --example rf_plugin_system <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        todo!("create a plugin manager")
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        todo!("add plugin {}", plugin.name())
    }

    pub fn remove_plugin(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        todo!("remove plugin {name}")
    }

    pub fn execute_all(&self) {
        todo!("execute all plugins")
    }
}

// Example usage
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyPlugin"
    }
    fn execute(&self) {
        println!("Executing MyPlugin");
    }
}

impl MyPlugin {
    fn new() -> Self {
        Self
    }
}

pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    struct TestPlugin {
        name: String,
    }

    impl TestPlugin {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
            }
        }
    }

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute(&self) {
            println!("Executing plugin: {}", self.name);
        }
    }

    #[test]
    fn test_add_and_execute_plugins() {
        let mut manager = PluginManager::new();

        let plugin1 = Box::new(TestPlugin::new("Plugin1"));
        let plugin2 = Box::new(TestPlugin::new("Plugin2"));

        manager.add_plugin(plugin1);
        manager.add_plugin(plugin2);

        manager.execute_all();
    }

    #[test]
    #[should_panic(expected = "Plugin with name 'Plugin1' already exists")]
    fn test_add_duplicate_plugin() {
        let mut manager = PluginManager::new();

        let plugin1 = Box::new(TestPlugin::new("Plugin1"));
        let duplicate_plugin = Box::new(TestPlugin::new("Plugin1"));

        manager.add_plugin(plugin1);
        manager.add_plugin(duplicate_plugin); // Should panic
    }

    #[test]
    fn test_remove_plugin() {
        let mut manager = PluginManager::new();

        let plugin1 = Box::new(TestPlugin::new("Plugin1"));
        manager.add_plugin(plugin1);

        let removed = manager.remove_plugin("Plugin1");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().name(), "Plugin1");

        assert!(manager.remove_plugin("Plugin1").is_none()); // Already removed
    }

    #[test]
    fn test_execute_with_no_plugins() {
        let manager = PluginManager::new();
        manager.execute_all(); // Should not panic and should do nothing
    }

    #[test]
    fn test_remove_non_existent_plugin() {
        let mut manager = PluginManager::new();
        assert!(manager.remove_plugin("NonExistent").is_none());
    }

    #[test]
    fn test_add_multiple_plugins() {
        let mut manager = PluginManager::new();

        for i in 1..=5 {
            let plugin = Box::new(TestPlugin::new(&format!("Plugin{}", i)));
            manager.add_plugin(plugin);
        }

        assert_eq!(manager.plugins.len(), 5);
    }
}
