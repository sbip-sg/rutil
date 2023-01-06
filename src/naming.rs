//! Module to provide naming utilities.

use std::collections::HashMap;

/// Data structure capturing a naming environment, which maps names in String to
/// their indices.
#[derive(Clone)]
pub struct NamingEnv {
    /// Mapping a name to its index in the current scope.
    current_naming_index: HashMap<String, usize>,

    /// Mapping a name to its index counter (the maximum index of this name).
    naming_index_counter: HashMap<String, usize>,
}

impl NamingEnv {
    /// Constructor.
    pub fn new() -> Self {
        NamingEnv {
            current_naming_index: HashMap::new(),
            naming_index_counter: HashMap::new(),
        }
    }

    /// Find the current index of a name.
    pub fn get_current_index(&self, name: &str) -> Option<usize> {
        match self.current_naming_index.get(name) {
            None => None,
            Some(0) => None, // by default, consider 0 index as None
            Some(idx) => Some(*idx),
        }
    }

    /// Create a new index for a name.
    pub fn create_new_name_index(
        &self,
        name: &String,
    ) -> (Option<usize>, NamingEnv) {
        // Create the new environment
        let mut new_env = self.to_owned();

        // New index
        let new_idx = match self.naming_index_counter.get(name) {
            None => 0,
            Some(idx) => *idx + 1,
        };

        // Update current index
        new_env
            .current_naming_index
            .insert(name.to_owned(), new_idx);

        // Update index counter
        new_env
            .naming_index_counter
            .insert(name.to_owned(), new_idx);

        // Fine-tune and return result
        let final_idx = ite!(new_idx == 0, None, Some(new_idx));
        (final_idx, new_env)
    }
}
