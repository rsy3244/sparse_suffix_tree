mod node;
mod pointer;

pub trait Alphabet: Sized + Clone + Copy {}

pub struct SparseSuffixTree<T: Alphabet> {
    text: Vec<T>,
    root: pointer::Pointer,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
