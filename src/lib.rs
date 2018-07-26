pub enum Key {
    SearchText,
    DisplayText,
    IconPath,
}

#[derive(Clone)]
pub struct SearchCandidate {
    searched_text: String,
    display_text: String,
    icon_path: String,
}

impl SearchCandidate {

    pub fn new(searched_text: String, display_text: String, icon_path: String) -> Self {
        SearchCandidate {
            searched_text,
            display_text,
            icon_path,
        }
    }

    pub fn get_value(&self, key: Key) -> String {
        match key {
            Key::SearchText => self.searched_text.clone(),
            Key::DisplayText => self.display_text.clone(),
            Key::IconPath => self.icon_path.clone(),
        }
    }

    pub fn set_value(&mut self, key: Key, value: String) {
        match key {
            Key::SearchText => self.searched_text = value,
            Key::DisplayText => self.display_text = value,
            Key::IconPath => self.icon_path = value,
        }
    }

}

#[cfg(test)]
mod tests {

    use SearchCandidate;
    use Key;

    #[test]
    fn get_value_test() {
        let search_candidate = SearchCandidate::new(String::from("torch"),
                                                    String::from("Torch"),
                                                    String::from(""));
        assert_eq!(search_candidate.get_value(Key::SearchText), "torch");
        assert_eq!(search_candidate.get_value(Key::DisplayText), "Torch");
        assert_eq!(search_candidate.get_value(Key::IconPath), "");
    }

    #[test]
    fn set_value_test() {
        let mut search_candidate = SearchCandidate::new(String::from("torch"),
                                                        String::from("Torch"),
                                                    String::from(""));

        // Check the state before setting value
        assert_eq!(search_candidate.get_value(Key::SearchText), "torch");
        assert_eq!(search_candidate.get_value(Key::DisplayText), "Torch");
        assert_eq!(search_candidate.get_value(Key::IconPath), "");

        // Set the value
        search_candidate.set_value(Key::SearchText, String::from("torch v2"));

        // Check the state after setting value
        assert_eq!(search_candidate.get_value(Key::SearchText), "torch v2");
        assert_eq!(search_candidate.get_value(Key::DisplayText), "Torch");
        assert_eq!(search_candidate.get_value(Key::IconPath), "");
    }

}
