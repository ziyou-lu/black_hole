/*************************************************
//  Copyright (C), 2020-2020 luwangda.
//  File name:       ci.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     core interface
//  Others:
//  History:
*************************************************/
use crate::core::base_entity::IBaseEntity;
use crate::core::base_interface::IBaseInterface;
use crate::core::entity_info::IEntityInfo;
use crate::share::any::IAny;
use crate::share::any_list::IArrayList;
use crate::share::obj_id::ObjId;

#[derive(Debug)]
pub(crate) struct ICore;

impl ICore {
    fn get_instance(&self) {
        unimplemented!()
    }

    fn get_work_path(&self) -> String {
        unimplemented!()
    }

    fn get_script_path(&self) -> String {
        unimplemented!()
    }

    fn get_resource_path(&self) -> String {
        unimplemented!()
    }

    fn get_main_config_path(&self) -> String {
        unimplemented!()
    }

    fn set_sleep_time(&self, value: i32) {
        unimplemented!()
    }

    fn get_sleep_time(&self) -> i32 {
        unimplemented!()
    }

    fn set_quit(&self, value: bool) {
        unimplemented!()
    }

    fn get_quit(&self) -> bool {
        unimplemented!()
    }

    fn execute(&self) -> bool {
        unimplemented!()
    }

    fn set_log_type(&self, log_type: i32) {
        unimplemented!()
    }

    fn get_log_type(&self) -> i32 {
        unimplemented!()
    }

    fn trace_log(&self, log: &str, log_type: i32) {
        unimplemented!()
    }

    fn dump_file(&self, file_type: &str, file: &str) {
        unimplemented!()
    }

    fn find_interface(&self, name: &str) {
        unimplemented!()
    }

    fn get_interface<T: IBaseInterface>(&self, name: &str) -> Option<T> {
        unimplemented!()
    }

    fn get_interface_same_space<T: IBaseInterface>(&self, p_bi: &T, name: &str) {
        unimplemented!()
    }

    fn release_interface<T: IBaseInterface>(&self, bi: &T) {
        unimplemented!()
    }

    fn get_entity_info<T: IEntityInfo>(&self, name: &str) -> Option<T> {
        unimplemented!()
    }

    fn get_main_entity(&self) -> Box<IBaseEntity> {
        unimplemented!()
    }

    fn get_entity(&self) -> Box<IBaseEntity> {
        unimplemented!()
    }

    fn get_all_entity<T: IArrayList>(&self, result: &mut T) -> u32 {
        unimplemented!()
    }

    fn lookup_entity(&self, name: &str) -> Box<IBaseEntity> {
        unimplemented!()
    }

    fn lookup_entity_more<T: IArrayList>(&self, name: &str, result: &mut T) -> u32 {
        unimplemented!()
    }

    fn create_entity(&self, name: &str) -> Box<IBaseEntity> {
        unimplemented!()
    }

    fn create_entity_args(&self, name: &str, args: &IArrayList) -> Box<IBaseEntity> {
        unimplemented!()
    }

    fn create_entity_same_space(
        &self,
        entity: &IBaseEntity,
        name: &str,
        args: &U,
    ) -> *const dyn IBaseEntity {
        unimplemented!()
    }

    fn delete_entity(&self, id: &ObjId) -> bool {
        unimplemented!()
    }

    fn set_can_del_by_script<T: IBaseEntity>(&self, entity: &T, value: bool) {
        unimplemented!()
    }

    fn get_property<T: IBaseEntity, U: IAny>(&self, entity: &T, prop: &str, value: &mut U) -> bool {
        unimplemented!()
    }

    fn set_property<T: IBaseEntity, U: IAny>(&self, entity: &T, prop: &str, value: &U) -> bool {
        unimplemented!()
    }

    fn invoke_method<T: IBaseEntity, U: IArrayList>(
        &self,
        entity: &T,
        func: &str,
        args: &U,
        res: &U,
    ) -> bool {
        unimplemented!()
    }

    fn run_function<T: IArrayList>(&self, func: &str, args: &T, res: &T) -> bool {
        unimplemented!()
    }

    fn exec_async_proc<T: IArrayList>(&self, script: &str, func: &str, args: &T, res: &T) -> bool {
        unimplemented!()
    }

    fn find_async_proc(&self, script: &str, func: &str, id: &ObjId) -> bool {
        unimplemented!()
    }

    fn kill_async_proc(&self, script: &str, func: &str, id: &ObjId) -> bool {
        unimplemented!()
    }

    fn gen_async_event<T: IArrayList>(&self, id: &ObjId, event: &str, args: &T) -> u32 {
        unimplemented!()
    }

    fn bind_script<T: IBaseEntity>(&self, entity: &T, script: &str) -> bool {
        unimplemented!()
    }

    fn bind_logic<T: IBaseEntity, U: IArrayList>(&self, entity: &T, logic: &str, args: &U) -> bool {
        unimplemented!()
    }

    fn find_callback<T: IBaseEntity>(&self, entity: &T, event: &str) -> bool {
        unimplemented!()
    }

    fn exec_callback<T: IBaseEntity, U: IArrayList>(
        &self,
        entity: &T,
        event: &str,
        args: &U,
        res: &U,
    ) -> bool {
        unimplemented!()
    }

    fn add_execute<T: IBaseEntity>(&self, entity: &T) -> bool {
        unimplemented!()
    }

    fn remove_execute<T: IBaseEntity>(&self, entity: &T) -> bool {
        unimplemented!()
    }

    fn find_global_value(&self, name: &str) -> bool {
        unimplemented!()
    }

    fn remove_global_value(&self, name: &str) -> bool {
        unimplemented!()
    }

    fn set_global_value<T: IAny>(&self, name: &str, value: &T) -> bool {
        unimplemented!()
    }

    fn get_global_value<T: IAny>(&self, name: &str) -> Option<T> {
        unimplemented!()
    }

    fn get_global_count(&self) -> u32 {
        unimplemented!()
    }

    fn get_global_list<T: IArrayList>(&self, result: &mut T) -> u32 {
        unimplemented!()
    }

    fn get_frame_seconds(&self) -> f32 {
        unimplemented!()
    }
}

#[derive(Debug)]
enum ELogType {
    ELogDbg = 0,         // 调试类日志
    ELogInformation = 1, // 信息类日志
    ELogWarning = 2,     // 警告类日志
    ELogError = 3,       // 错误类日志(程序可以继续执行)
    ELogFatal = 4,       // 错误类日志(程序没法继续执行)
}
