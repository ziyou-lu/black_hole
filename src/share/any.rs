use super::any_type::*;
use super::obj_id::ObjId;

pub(crate) struct IAny {
    data_: VarData,
}

impl IAny {
    pub fn get_type(&self) -> VarType{
        self.data_.type_
    }

    pub fn is_integer(&self) -> bool {
        self.data_.type_ == VarType::VarTypeI8 || self.data_.type_ == VarType::VarTypeI16
            || self.data_.type_ == VarType::VarTypeI32 || self.data_.type_ == VarType::VarTypeI64
    }

    pub fn is_real(&self) -> bool{
       self.data_.type_ == VarType::VarTypeF32 || self.data_.type_ == VarType::VarTypeF64 
    }

    pub fn is_number(&self) -> bool{
        self.is_integer() || self.is_real()
    }

    pub fn get_bool(&self) -> bool {
        self.data_.data_.v_bool
    }

    pub fn get_i8(&self) -> i8 {
        self.data_.data_.v_i8
    }

    pub fn get_i16(&self) -> i16 {
        self.data_.data_.v_i16
    }

    pub fn get_i32(&self) -> i32 {
        self.data_.data_.v_i32
    }

    pub fn get_i64(&self) -> i64 {
        self.data_.data_.v_i64
    }

    pub fn get_f32(&self) -> f32 {
        self.data_.data_.v_f32
    }

    pub fn get_f64(&self) -> f64 {
        self.data_.data_.v_f64
    }

    pub fn get_str(&self) -> &'static str {
        self.data_.data_.v_str
    }
 
    pub fn get_obj(&self) -> ObjId {
        self.data_.data_.v_obj
    }

    pub fn set_bool(&self, value: bool) {
        self.data_.data_ = AnyType{v_bool: value}
    }

    pub fn set_i8(&self, value: i8) {
        self.data_.data_ = AnyType{v_i8: value}
    }

    pub fn set_i16(&self, value: i16) {
        self.data_.data_ = AnyType{v_i16: value}
    }

    pub fn set_i32(&self, value: i32) {
        self.data_.data_ = AnyType{v_i32: value}
    }

    pub fn set_f32(&self, value: f32) {
        self.data_.data_ = AnyType{v_f32: value}
    }

    pub fn set_f64(&self, value: f64) {
        self.data_.data_ = AnyType{v_f64: value}
    }

    pub fn set_str(&self, value: &'static str) {
        self.data_.data_ = AnyType{v_str: value}
    }

    pub fn set_obj(&self, value: ObjId) {
        self.data_.data_ = AnyType{v_obj: value}
    }

}
