mod mall;
pub use crate::mall::*;
use std::collections::HashMap;
pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut data = HashMap::new();

    for floor in mall.floors.clone() {
        for (string, store) in floor.1.stores.clone() {
            data.insert(string, store);
        }
    }
    data.into_iter()
        .max_by_key(|(_, v)| v.square_meters)
        .unwrap()
}
pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut highest: Vec<(&String, &Employee)> = Vec::new();
    for floor in &mall.floors {
        for store in &floor.1.stores {
            for s in &store.1.employees {
                if !highest.is_empty() && s.1.salary > highest[0].1.salary {
                    highest.clear();
                    highest.push((&s.0, &s.1));
                } else if !highest.is_empty() && s.1.salary == highest[0].1.salary {
                    highest.push((&s.0, &s.1));
                } else if highest.is_empty() {
                    highest.push((&s.0, &s.1));
                }
            }
        }
    }
    highest
}
pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut number_employee: usize = 0;
    for floor in mall.floors.clone() {
        for store in floor.1.stores.clone() {
            for _ in store.1.employees {
                number_employee += 1;
            }
        }
    }
    for _ in mall.guards.clone() {
        number_employee += 1;
    }
    number_employee
}
pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut total_area: u64 = 0;
    for floor in mall.floors.clone() {
        total_area += floor.1.size_limit;
    }
    let required_guards = (total_area / 200) as usize;

    let current_guards = mall.guards.len();

    if current_guards < required_guards {
        let add_guard = required_guards - current_guards;
        for (name, guard) in guards.into_iter().take(add_guard) {
            mall.hire_guard(name, guard);
        }
    }
}
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let working_hours = employee.working_hours.1 - employee.working_hours.0;

                if working_hours >= 10 {
                    employee.raise(employee.salary * 0.1);
                } else {
                    employee.cut(employee.salary * 0.1);
                }
            }
        }
    }
}
