// 'file'
trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

trait MyDebug: std::fmt::Debug {
    fn my_subtrait_function(&self);
}
