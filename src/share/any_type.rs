pub enum VarType{
	VarType_UnKnow,		    // 未知
	VarType_Bool,			// 布尔
	VarType_I8,			    // 1字节
	VarType_I16,			// 2字节
	VarType_I32,			// 32位整数
    VarType_I64,			// 64位整数
    VarType_I128,			// 128位整数
	VarType_F32,			// 单精度浮点数
	VarType_F64,			// 双精度浮点数
	VarType_Str,			// 字符串
	VarType_Obj,			// 对象号
	VarType_Max,
};