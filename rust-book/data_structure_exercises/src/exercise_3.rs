use std::collections::HashMap;

pub struct Company {
    pub departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn add(&mut self, name: &str, department_name: &str) {
        let department = self
            .departments
            .entry(String::from(department_name))
            .or_insert(vec![]);
        department.push(String::from(name))
    }

    pub fn get_all(&self) -> Vec<String> {
        let mut departments = vec![];
        for (_, value) in &self.departments {
            departments.extend_from_slice(&value);
        }
        departments
    }

    pub fn get_by_department(&self, name: &str) -> Option<&Vec<String>> {
        let department = self.departments.get(&*name);
        match department {
            Some(v) => Some(v),
            None => None,
        }
    }
}
