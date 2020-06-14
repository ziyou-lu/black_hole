use super::obj_id::ObjId;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VarType{
	VarTypeUnKnow,		    // 未知
	VarTypeBool,			// 布尔
	VarTypeI8,			    // 1字节
	VarTypeI16,			// 2字节
	VarTypeI32,			// 32位整数
    VarTypeI64,			// 64位整数
    VarTypeI128,			// 128位整数
	VarTypeF32,			// 单精度浮点数
	VarTypeF64,			// 双精度浮点数
	VarTypeStr,			// 字符串
	VarTypeObj,			// 对象号
	VarTypeMax,
}

#[derive(Copy, Clone)]
pub union AnyType {
    pub v_i8: i8,
    pub v_i16: i16,
    pub v_i32: i32,
    pub v_i64: i64,
    pub v_f32: f32,
    pub v_f64: f64,
    pub v_bool: bool,
    pub v_obj: ObjId,
    pub v_str: &'static str,
}

#[derive(Copy, Clone)]
pub struct VarData {
    pub type_: VarType,
    pub data_: AnyType,
}