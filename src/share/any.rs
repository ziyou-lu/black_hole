
pub(crate) trait IAny {
    fn get_type(&self) -> i32;

    fn is_real(&self) -> bool;

    fn is_number(&self) -> bool;

    fn assign(&self, src: &dyn IAny);
}

trait IAnyOp<T> {
    fn get_value<T>(&self) -> Option<T>;

    fn set_value<T>(&self, value: T);
}

impl IAnyOp<T> for dyn IAny {
    fn get_value<T>(&self) -> Option<T> {
        unimplemented!()
    }

    fn set_value<T>(&self, value: T) {
        unimplemented!()
    }
}
