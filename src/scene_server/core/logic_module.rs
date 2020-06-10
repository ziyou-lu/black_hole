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

use super::gravity::IGravity;
#[derive(Debug)]
pub(crate) struct ILogicModule {}

impl ILogicModule {
    fn init(&self, gravity: Box<IGravity>) -> bool {}

    fn shut(&self, gravity: Box<IGravity>) -> bool {}

    fn before_launch(&self, gravity: Box<IGravity>) -> bool {
        true
    }
}
