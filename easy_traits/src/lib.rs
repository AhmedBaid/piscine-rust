pub trait AppendStrExt {
    fn append_str(&mut self, str_to_append: &str) -> &mut Self;

    fn append_number(&mut self, nb_to_append: f64) -> &mut Self;

    fn remove_punctuation_marks(&mut self) -> &mut Self;
}

impl AppendStrExt for String {
    fn append_str(&mut self, str_to_append: &str) -> &mut Self {
        *self = format!("{}{}", self, str_to_append);
        self
    }

    fn append_number(&mut self, nb_to_append: f64) -> &mut Self {
        *self = format!("{}{}", self, nb_to_append);
        self
    }

    fn remove_punctuation_marks(&mut self) -> &mut Self {
        let mut new = String::new();
        for ch in self.chars(){
            if !"!?.,".contains(ch){
                new.push(ch);
            }
        }
        *self = new;
        self
    }
}
