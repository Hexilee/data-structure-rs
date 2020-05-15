pub struct DoubleArrayTrie {
    base: Vec<usize>,
    check: Vec<usize>,
}

impl DoubleArrayTrie {
    pub fn new() -> Self {
        Self {
            base: vec![0],
            check: Vec::new(),
        }
    }

    fn state(&self, s: &str) -> Option<usize> {
        if s == "" {
            Some(0)
        } else {
            let last_state = self.state(&s[..s.len() - 1])?;
            Some(self.base.get(last_state)? + s.as_bytes()[s.len() - 1] as usize)
        }
    }

    fn get(&self, s: &str) -> Option<()> {
        for i in 1..s.len() {
            let state = self.state(&s[..i])?;
            let check = *self.check.get(state)?;
            if check == *self.base.get(self.state(&s[..i - 1])?)? {
                continue;
            }
            return None;
        }
        Some(())
    }

    pub fn contains(&self, s: &str) -> bool {
        self.get(s).is_some()
    }

    pub fn insert(&mut self, _s: &str) {
        unimplemented!()
    }
}
