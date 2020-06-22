use std::os::raw::c_void;
use crate::share::any_type::VarType;
use std::ptr::null;
use crate::share::obj_id::ObjId;

pub fn swap<T: Copy>(mut v1: T, mut v2: T) {
    let t: T = v1;
    v1 = v2;
    v2 = t;
}

struct StructType {
    pub type_: VarType,
    pub flag_: u16,
}

pub union AttrTypeFlag {
    pub str_type_: StructType,
    pub dummy_: u32,
}

pub union AttrValueData {
    pub value_: i32,
    pub value_f_: f32,
    pub value_ptr_: *const c_void,
}

#[derive(Debug, Copy, Clone)]
pub struct AttrValue {
    type_: AttrTypeFlag,
    value_: AttrValueData,
}

impl AttrValue {
    pub unsafe fn new(src: Self) -> Self {
        let mut attr = src;

        match src.type_.str_type_.type_ {
            VarType::VarTypeI64 => {
                attr.value_.value_ptr_ = null();

                if src.value_.value_ptr_ != null() {
                    attr.inner_set_i64(*(src.value_.value_ptr_ as *const i64))
                }
            },
            VarType::VarTypeF64 => {
                attr.value_.value_ptr_ = null();

                if src.value_.value_ptr_ != null() {
                    attr.inner_set_f64(*(src.value_.value_ptr_ as *const f64))
                }
            },
            VarType::VarTypeStr => {
                attr.value_.value_ptr_ = null();

                if src.value_.value_ptr_ != null() {
                    attr.inner_set_str(src.value_.value_ptr_ as *const String)
                }
            },
            VarType::VarTypeObj => {
                attr.value_.value_ptr_ = null();

                if src.value_.value_ptr_ != null() {
                    attr.inner_set_obj(*(src.value_.value_ptr_ as *const ObjId))
                }
            }
            _ => {}
        }

        attr
    }

    fn inner_set_i64(&mut self, value: i64) {
        self.value_.value_ptr_ = *value;
    }

    fn inner_set_f64(&mut self, value: f64) {
        self.value_.value_ptr_ = *value;
    }

    fn inner_set_str(&mut self, value: *const String) {
        self.value_.value_ptr_ = value as *const c_void;
    }

    fn inner_set_obj(&mut self, value: ObjId) {
        self.value_.value_ptr_ = *value;
    }
}