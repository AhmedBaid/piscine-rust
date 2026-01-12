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
pub fn highest_paid_employee(mall: &Mall) -> Vec<Employee> {
    let mut highest: Vec<Employee> = Vec::new();
    for floor in mall.floors.clone() {
        for store in floor.1.stores.clone() {
            for s in store.1.employees {
                if !highest.is_empty() && s.1.salary > highest[0].salary {
                    highest.clear();
                    highest.push(s.1);
                } else if !highest.is_empty() && s.1.salary == highest[0].salary {
                    highest.clear();
                    highest.push(s.1);
                } else if highest.is_empty() {
                    highest.push(s.1);
                }
            }
        }
    }
    highest
}
pub fn nbr_of_employees(mall: &Mall) -> i32 {
    let mut number_employee: i32 = 0;
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
        for (_, store) in floor.1.stores.clone() {
            total_area += store.square_meters;
        }
    }
    let required_guards = (total_area / 200) as usize;

    let current_guards = mall.guards.len();

    if current_guards < required_guards {
        for (name, guard) in guards.into_iter() {
            if mall.guards.len() >= required_guards {
                break;
            }
            mall.hire_guard(name, guard);
        }
    }
}
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                // Calculate working hours
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

/*
$ cargo run
Biggest store: (
    "Pretail",
    Store {
        employees: {
            "Mohsin Mcgee": Employee {
                age: 30,
                working_hours: (
                    19,
                    24,
                ),
                salary: 703.83,
            },
            "Jadine Page": Employee {
                age: 48,
                working_hours: (
                    13,
                    20,
                ),
                salary: 743.21,
            },
            "Yara Wickens": Employee {
                age: 39,
                working_hours: (
                    9,
                    14,
                ),
                salary: 853.42,
            },
            "Antoine Goulding": Employee {
                age: 45,
                working_hours: (
                    19,
                    24,
                ),
                salary: 697.12,
            },
            "Indiana Baxter": Employee {
                age: 33,
                working_hours: (
                    13,
                    20,
                ),
                salary: 991.71,
            },
            "Mark Barnard": Employee {
                age: 53,
                working_hours: (
                    19,
                    24,
                ),
                salary: 788.81,
            },
            "Tyler Hunt": Employee {
                age: 63,
                working_hours: (
                    13,
                    20,
                ),
                salary: 668.25,
            },
        },
        square_meters: 950,
    },
)
Highest paid employee: [
    (
        "Abdallah Stafford",
        Employee {
            age: 54,
            working_hours: (
                8,
                22,
            ),
            salary: 1234.21,
        },
    ),
]
Number of employees: 13
Mall {
    name: "La Vie Funchal",
    guards: {
        "Ray Storey": Guard {
            age: 37,
            years_experience: 12,
        },
        "Jason Mackie": Guard {
            age: 26,
            years_experience: 2,
        },
        "John Oliver": Guard {
            age: 34,
            years_experience: 7,
        },
        "Christopher Smith": Guard {
            age: 35,
            years_experience: 9,
        },
        "Peter Solomons": Guard {
            age: 45,
            years_experience: 20,
        },
        "Bob Schumacher": Guard {
            age: 53,
            years_experience: 15,
        },
    },
    floors: {
        "Supermarket": Floor {
            stores: {
                "Pretail": Store {
                    employees: {
                        "Mohsin Mcgee": Employee {
                            age: 30,
                            working_hours: (
                                19,
                                24,
                            ),
                            salary: 633.447,
                        },
                        "Jadine Page": Employee {
                            age: 48,
                            working_hours: (
                                13,
                                20,
                            ),
                            salary: 668.889,
                        },
                        "Yara Wickens": Employee {
                            age: 39,
                            working_hours: (
                                9,
                                14,
                            ),
                            salary: 768.078,
                        },
                        "Antoine Goulding": Employee {
                            age: 45,
                            working_hours: (
                                19,
                                24,
                            ),
                            salary: 627.408,
                        },
                        "Indiana Baxter": Employee {
                            age: 33,
                            working_hours: (
                                13,
                                20,
                            ),
                            salary: 892.539,
                        },
                        "Mark Barnard": Employee {
                            age: 53,
                            working_hours: (
                                19,
                                24,
                            ),
                            salary: 709.929,
                        },
                        "Tyler Hunt": Employee {
                            age: 63,
                            working_hours: (
                                13,
                                20,
                            ),
                            salary: 601.425,
                        },
                    },
                    square_meters: 950,
                },
            },
            size_limit: 1000,
        },
        "Ground Floor": Floor {
            stores: {
                "Swashion": Store {
                    employees: {
                        "Abdallah Stafford": Employee {
                            age: 54,
                            working_hours: (
                                8,
                                22,
                            ),
                            salary: 1357.631,
                        },
                        "Marian Snyder": Employee {
                            age: 21,
                            working_hours: (
                                8,
                                14,
                            ),
                            salary: 748.71,
                        },
                    },
                    square_meters: 43,
                },
                "Footzo": Store {
                    employees: {
                        "Finbar Haines": Employee {
                            age: 36,
                            working_hours: (
                                9,
                                14,
                            ),
                            salary: 585.792,
                        },
                        "Sienna-Rose Penn": Employee {
                            age: 26,
                            working_hours: (
                                9,
                                22,
                            ),
                            salary: 1100.473,
                        },
                    },
                    square_meters: 50,
                },
            },
            size_limit: 300,
        },
    },
}
$
*/
