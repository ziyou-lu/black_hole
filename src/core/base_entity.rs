/*************************************************
//  Copyright (C), 2020-2020, luangada.
//  File name:       i_base_entity
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base entity interface define
//  Others:
//  History:
*************************************************/
use entity_info;
use core;
use entity_script;
use share::any_list;

pub(crate) trait IBaseEntity {
    fn init();
}

struct BaseEntity {
    core_: *core,
    entity_info_: *entity_info,
}
