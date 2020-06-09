/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       IBaseLogicInfo.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base logic info interface define
//  Others:
//  History:
*************************************************/
use super::base_logic_creator::IBaseLogicCreator;
use crate::core::base_logic_creator::BaseLogicCallBack;
use crate::runtime::inlines;
use crate::runtime::inlines::get_hash_value_case;
use crate::share::any_list::IArrayList;
use std::borrow::Borrow;

// 逻辑方法信息
#[derive(Debug)]
trait ICallBackInfo {
    fn get_name(&self) -> String;
    fn get_mid_func(&self) -> fn();
    fn get_return_table(&self) -> bool;
}

struct CallBackInfo {
    name_: Some(String),
    hash_: u32,
    mid_func_: fn(),
    return_table_: bool,
}

impl ICallBackInfo for CallBackInfo {
    fn get_name(&self) -> Option<String> {
        self.name_
    }

    fn get_mid_func(&self) -> fn() {
        self.mid_func_
    }

    fn get_return_table(&self) -> bool {
        self.return_table_
    }
}

impl CallBackInfo {
    fn get_hash(&self) -> u32 {
        self.hash_
    }

    fn set_name(&mut self, value: String) {
        self.name_ = value;
    }

    fn set_hash(&mut self, value: u32) {
        self.hash_ = value;
    }

    fn set_mid_func(&mut self, value: fn()) {
        self.mid_func_ = value;
    }

    fn set_return_table(&mut self, value: bool) {
        self.return_table_ = value;
    }
}

pub(crate) trait IBaseLogicInfo {
    // 获得创建起
    fn get_creator<T: IBaseLogicCreator>(&self) -> Option<T>;

    // 返回名字空间
    fn get_space_name(&self) -> String;

    // 返回类名
    fn get_logic_name(&self) -> String;

    // 方法数量
    fn get_callback_count(&self) -> usize;

    // 获取方法名字列表
    fn get_callback_list<T: IArrayList>(&self, result: &T) -> usize;

    // 在本类中获取方法信息
    fn get_callback_info<T: ICallBackInfo>(&self, name: &str) -> Option<T>;
}

struct BaseLoginInfo {
    creator_: Some(dyn IBaseLogicCreator),
    space_name_: Some(String),
    logic_name_: Some(String),
    call_back_infos_: Vec<CallBackInfo>,
}

impl IBaseLogicInfo for BaseLoginInfo {
    fn get_creator<T: IBaseLogicCreator>(&self) -> Option<T> {
        self.creator_
    }

    fn get_space_name(&self) -> Option<String> {
        self.space_name_
    }

    fn get_logic_name(&self) -> String {
        self.logic_name_
    }

    fn get_callback_count(&self) -> usize {
        self.call_back_infos_.len()
    }

    fn get_callback_list<T: IArrayList>(&self, result: &mut T) -> u32 {
        result.clear();
        inner_get_callback_list(&result);
        result.get_count()
    }

    fn get_callback_info<T: ICallBackInfo>(&self, name: &str) -> Option<T> {
        let mut index: usize = 0;

        if !find_call_back_index(name, index) {
            None
        }

        Some(&self.call_back_infos_[index])
    }
}

impl BaseLoginInfo {
    fn inner_get_callback_list<T: IArrayList>(&self, result: &mut T) {
        let size = self.call_back_infos_.len();

        for i in 0..size {
            result << self.call_back_infos_[i].get_name();
        }
    }

    fn find_callback_index(&self, name: &str, mut index: &usize) -> bool {
        let hash = inlines::get_hash_value_case(name);

        for callback in self.call_back_infos_ {
            if callback.get_hash() == hash
                && String::from(callback.name_).eq(String::from(name).borrow())
            {
                index = i;
                true
            }
        }
        false
    }

    fn set_creator<T: IBaseLogicCreator>(&self, value: T) {
        self.creator_ = value;
    }

    fn set_space_name(&self, value: &str) {
        self.space_name_ = String::from(value);
    }

    fn set_logic_name(&self, value: &str) {
        self.logic_name_ = String::from(value);
    }

    fn add_callback_info(&mut self, name: &str, mid_func: fn(), ret_table: bool) {
        self.call_back_infos_.push(CallBackInfo {
            name_: name,
            hash_: get_hash_value_case(name),
            mid_func_: mid_func,
            return_table_: ret_table,
        });
    }

    fn add_callback_link(&mut self, call_back: Some(BaseLogicCallBack)) -> usize {
        let mut count: usize = 0;
        let mut temp: Some(BaseLogicCallBack) = call_back;
        let mut count = 0;
        while temp != None {
            let temp_callback: BaseLogicCallBack = temp.unwrap();
            temp = temp_callback.next_;
            count += 1;
        }

        self.call_back_infos_.resize_with(count, Default::default());
        let mut index = count - 1;

        temp = call_back;
        while temp != None {
            let mut data = &self.call_back_infos_[index];
            let temp_callback: BaseLogicCallBack = temp.unwrap();
            data.set_name(temp_callback.name_);
            data.set_hash(get_hash_value_case(temp_callback.name_.as_ref()));
            data.set_mid_func(temp_callback.mid_func_);
            data.set_return_table(temp_callback.return_table_);

            index -= 1;
            temp = temp_callback.next_;
        }

        count
    }
}
