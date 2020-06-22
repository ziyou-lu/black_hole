use std::os::raw::c_void;
use super::super::core::object::IObject;

pub enum EClassType
{
    EClassNull =	    0x0,		/**< null */
    EClassScene =	    0x1,		/**< scene */
    EClassRole =	    0x2,		/**< player */
    EClassNpc =	        0x4,		/**< npc */
    EClassItem =	    0x8,		/**< item */
    EClassAide =	    0x10,		/**< helper */
    EClassContainer =	0x20,		/**< weak box */
}

/* 函数指针 */
#[derive(Debug)]
pub struct RegistrCbT {
    func_ptr_: *const c_void,
    func_name_: String,
}

/*心跳信息*/
#[derive(Debug)]
pub struct HBInfoT {
    pre_: Option<Box<HBInfoT>>,
    next_: Option<Box<HBInfoT>>,
    hb_name_: String,
}

// 每次最大刷新数
const MAX_VISUAL_NUM            : u32 = 256;
// 预留玩家
const MAX_VISUAL_RESERVE_NUM    : u32 = 50;
const DEF_LOOKER_NUM            : u32 = 128;
const MAX_LOOKER_NUM            : u32 = 1024;
const DEF_VISUAL_RANGE          : u32 = 50;

pub enum EPropType
{
    PfPublic =	        0x1,	// 公有
    PfPrivate =	        0x2,	// 私有
    PfRealtime =	    0x4,	// 即时刷新
    PfMember =	        0x8,	// 内部
    PfSaving =	        0x10,	// 保存
    PfNotSaveEmpty =	0x20,	// 为空时不保存 散表保存，暂时去掉这个标记
    PfTransfer =	    0x40,	// 跨服需要带的标记
    PfSwitchCarry =	    0x80,	// 切场景需要带着的标记(对于属性和表格，如果不带这个标记，则一定是不存档的，否则会出错)
}

pub enum EPropTempFlag
{
    TfCritical =	0x01,	// 属性改变回调
    TfRecHook =	    0x02,	// 表数据改变回调
    TfHiding =	    0x04,	// 隐藏可视属性
    TfRunning =	    0x08,	// 运行回调标志
}

// 内部属性
pub struct InternalT {
    prop_name_: String,
    type_:      i32,
}

// 临时属性
pub struct VolatileT {
    vol_name_:  String,
    vol_flg_:   u32,
    
}

pub struct BaseObject {
    parent_: IObject,

}



