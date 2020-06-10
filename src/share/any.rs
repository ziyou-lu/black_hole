#[derive(Debug)]
pub(crate) struct IAny {
    }

impl IAny {
    fn get_type(&self) -> i32{}
    fn is_real(&self) -> bool{}

    fn is_number(&self) -> bool{}

    fn assign(&self, src: &Self){}
 
    fn get_value<T>(&self) -> Option<T> {
        unimplemented!()
    }

    fn set_value<T>(&self, value: T) {
        unimplemented!()
    }

}
