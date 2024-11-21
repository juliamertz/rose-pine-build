pub(crate) trait Substitutable {
    fn substitute(&mut self, replacement: Self, start: usize, end: usize);
}

impl Substitutable for String {
    fn substitute(&mut self, replacement: Self, start: usize, end: usize) {
        let (before, after) = self.split_at(start);
        let (_, after_replace) = after.split_at(end - start);
        *self = format!("{}{}{}", before, replacement, after_replace)
    }
}

