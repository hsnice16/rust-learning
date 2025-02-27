// fn borrow<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
//     i
// }

// fn main() {
//     let n = 12;
//     let borrowed_n = &n;
//     borrow(&n, borrowed_n);
// }

struct Cat(String);

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purrring)", self.0);
    }
}

struct CatFeeder<'a> {
    cat: &'a mut Cat,
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    let mut cats = vec![Cat("Froda".to_string()), Cat("Bilbo".to_string())];

    let mut feeders = Vec::new();
    for cat in cats.iter_mut() {
        feeders.push(CatFeeder { cat });
    }

    feeders.iter_mut().for_each(|f| f.feed());
}
