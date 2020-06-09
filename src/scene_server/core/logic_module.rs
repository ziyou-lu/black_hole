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
    fn init<T: IGravity>(&self, gravity: &T) -> bool;

    fn shut<T: IGravity>(&self, gravity: &T) -> bool;

    fn before_launch<T: IGravity>(&self, gravity: &T) -> bool {
        true
    }
}
