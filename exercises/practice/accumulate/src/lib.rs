/// What should the type of f be?
pub fn map<I, O, F>(input: Vec<I>, mut f: F) -> Vec<O> 
where 
    I: Sized,
    O: Sized,
    F: FnMut(I) -> O {
    input.into_iter().map(|i| f(i)).collect()
}
