use std::collections::{HashMap, HashSet, VecDeque};

use spinel_events::{RegisteredEvent, RegisteredModule, RegisteredModuleDependency};
use spinel_network::PacketListener;

pub struct ModuleManager {
    pub independent_events: HashSet<&'static str>,
    pub active_unqualified_modules: HashSet<&'static str>,
}

impl ModuleManager {
    pub fn new() -> Self {
        // Gather all unique module names.
        let mut all_module_names: HashSet<&'static str> = HashSet::new();
        for module in inventory::iter::<&'static RegisteredModule>() {
            all_module_names.insert(module.name);
        }
        for dependency in inventory::iter::<&'static RegisteredModuleDependency>() {
            all_module_names.insert(dependency.subject_module);
            all_module_names.insert(dependency.dependent_on);
        }

        // Map of unqualified names to qualified module IDs.
        let module_resolution_map: HashMap<String, Vec<String>> = {
            let mut map: HashMap<String, Vec<String>> = HashMap::new();
            for qualified_name in all_module_names {
                let unqualified_name = qualified_name
                    .split(':')
                    .last()
                    .unwrap_or(qualified_name)
                    .to_string();
                map.entry(unqualified_name)
                    .or_default()
                    .push(qualified_name.to_string());
            }
            map
        };

        // Collect all unique raw module names requiring resolution.
        let mut modules_to_resolve: HashSet<&'static str> =
            inventory::iter::<&'static RegisteredModule>()
                .map(|module| module.name)
                .collect();
        for dependency in inventory::iter::<&'static RegisteredModuleDependency>() {
            modules_to_resolve.insert(dependency.subject_module);
            modules_to_resolve.insert(dependency.dependent_on);
        }

        // Create cache of resolved qualified names.
        let resolved_name_cache: HashMap<&'static str, String> = modules_to_resolve
            .into_iter()
            .map(|raw_name| {
                let resolved = resolve_ambiguous_module(raw_name, &module_resolution_map);
                (raw_name, resolved)
            })
            .collect();

        // Populate initial set of active modules.
        let mut starting_active_modules = HashSet::new();
        for module in inventory::iter::<&'static RegisteredModule>() {
            if let Some(resolved) = resolved_name_cache.get(module.name) {
                starting_active_modules.insert(resolved.clone());
            }
        }

        // Build the dependency graph using cached results.
        let mut dependency_graph: HashMap<String, HashSet<String>> = HashMap::new();
        for dependency in inventory::iter::<&'static RegisteredModuleDependency>() {
            let resolved_subject = resolved_name_cache
                .get(dependency.subject_module)
                .unwrap()
                .clone();
            let resolved_dependency = resolved_name_cache
                .get(dependency.dependent_on)
                .unwrap()
                .clone();

            dependency_graph
                .entry(resolved_subject)
                .or_default()
                .insert(resolved_dependency);
        }

        // Traverse dependencies to find all required active modules.
        let mut active_modules: HashSet<String> = starting_active_modules.clone();
        let mut modules_to_check: VecDeque<String> = starting_active_modules.into_iter().collect();

        while let Some(current_module) = modules_to_check.pop_front() {
            if let Some(dependencies) = dependency_graph.get(&current_module) {
                for dependency in dependencies {
                    if active_modules.insert(dependency.clone()) {
                        modules_to_check.push_back(dependency.clone());
                    }
                }
            }
        }

        // Collect names of all independent events.
        let independent_events: HashSet<&'static str> =
            inventory::iter::<&'static RegisteredEvent>()
                .filter(|event| event.is_independent)
                .map(|event| event.name)
                .collect();

        // Generate the final set of active unqualified names.
        let active_unqualified_modules: HashSet<&'static str> = inventory::iter::<
            &'static PacketListener,
        >()
        .flat_map(|listener| listener.modules)
        .copied()
        .filter(|unqualified_name| {
            let resolved_name = resolve_ambiguous_module(unqualified_name, &module_resolution_map);
            active_modules.contains(&resolved_name)
        })
        .collect();

        ModuleManager {
            independent_events,
            active_unqualified_modules,
        }
    }
}

// Does what the name implies, incase you're slow.
fn resolve_ambiguous_module(
    unqualified_name: &str,
    all_modules_map: &HashMap<String, Vec<String>>,
) -> String {
    // Resolves an unqualified module name to its unique qualified ID.
    if unqualified_name.contains(':') {
        return unqualified_name.to_string();
    }

    match all_modules_map.get(unqualified_name.to_lowercase().as_str()) {
        None => {
            format!("unknown:{}", unqualified_name)
        }
        Some(candidates) => {
            if candidates.len() == 1 {
                candidates[0].clone()
            } else {
                let minecraft_candidate = format!("minecraft:{}", unqualified_name);
                if candidates.iter().any(|c| c == &minecraft_candidate) {
                    minecraft_candidate
                } else {
                    panic!(
                        "Module name '{}' is ambiguous. Candidates found: {:?}. Please specify a namespace (e.g., 'namespace:{}').",
                        unqualified_name, candidates, unqualified_name
                    );
                }
            }
        }
    }
}
