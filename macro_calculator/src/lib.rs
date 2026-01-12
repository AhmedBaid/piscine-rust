pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;
    for obj in foods {
        let calorie = obj.calories.1[0..(obj.calories.1.len() - 4)]
            .parse::<f64>()
            .unwrap()
            * obj.nbr_of_portions;
        cals += calorie;
        let carb = obj.carbs * obj.nbr_of_portions;
        carbs += carb;
        let protein = obj.proteins * obj.nbr_of_portions;
        proteins += protein;
        let fat = obj.fats * obj.nbr_of_portions;
        fats += fat;
    }
    cals = (cals * 100.0).round() / 100.0;
    carbs = (carbs * 100.0).round() / 100.0;
    proteins = (proteins * 100.0).round() / 100.0;
    fats = (fats * 100.0).round() / 100.0;
    
    let mut json_data = json::JsonValue::new_object();
    json_data["cals"] = cals.into();
    json_data["carbs"] = carbs.into();
    json_data["proteins"] = proteins.into();
    json_data["fats"] = fats.into();
    json_data
}
