/*************************************************
//  Copyright (C), 2017-2018, luwangda
//  File name:       api_logic_module.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2029/06/03
//  Description:     逻辑接口
//  Others:
//  History:
*************************************************/

use crate::core::gravity::IGravity;

pub(crate) trait ILogicModule {
    fn init(&self, gravity: *const dyn IGravity) -> bool;

    fn shut(&self, gravity: *const dyn IGravity) -> bool;

    fn before_launch(&self, gravity: *const dyn IGravity) -> bool {
        true
    }
}
