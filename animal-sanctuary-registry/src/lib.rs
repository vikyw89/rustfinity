use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    // TODO: implement this function
    if !registry.contains_key(section) {
        registry.insert(section.to_string(), vec![]);
    }

    // add the animal to the section if it's not already there
    let registry_content = registry.get_mut(section).unwrap();

    if !registry_content.contains(&animal.to_string()) {
        registry_content.push(animal.to_string());
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    // TODO: implement this function

    registry
        .get(section)
        .map(|animals| {
            let mut sorted = animals.to_vec();
            sorted.sort();
            sorted
        })
        .unwrap_or_default()
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    let mut animals: Vec<String> = registry
        .values()
        .flat_map(|value| value.iter().cloned())
        .collect();

    animals.sort();

    animals
}
