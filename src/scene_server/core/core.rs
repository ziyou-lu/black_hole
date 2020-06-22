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
use super::base_entity::IBaseEntity;
use super::base_interface::IBaseInterface;
use super::entity_info::IEntityInfo;
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

    fn get_interface(&self, name: &str) -> Box<impl IBaseInterface> {
        unimplemented!()
    }

    fn get_interface_same_space(&self, p_bi: Box<impl IBaseInterface>, name: &str) {
        unimplemented!()
    }

    fn release_interface(&self, p_bi: Box<impl IBaseInterface>) {
        unimplemented!()
    }

    fn get_entity_info(&self, name: &str) -> Box<IEntityInfo> {
        unimplemented!()
    }

    fn get_main_entity(&self) -> Box<IBaseEntity> {
        unimplemented!()
    }

    fn get_entity(&self) -> Box<IBaseEntity> {
        unimplemented!()
    }

    fn get_all_entity(&self, result: &mut IArrayList) -> u32 {
        unimplemented!()
    }

    fn lookup_entity(&self, name: &str) -> Box<IBaseEntity> {
        unimplemented!()
    }

    fn lookup_entity_more(&self, name: &str, result: &mut IArrayList) -> u32 {
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
        args: &IArrayList,
    ) -> Box<IBaseEntity> {
        unimplemented!()
    }

    pub fn delete_entity(&self, id: &ObjId) -> bool {
        unimplemented!()
    }

    fn set_can_del_by_script(&self, entity: Box<IBaseEntity>, value: bool) {
        unimplemented!()
    }

    fn get_property(&self, entity: Box<IBaseEntity>, prop: &str, value: &mut IAny) -> bool {
        unimplemented!()
    }

    fn set_property(&self, entity: Box<IBaseEntity>, prop: &str, value: &IAny) -> bool {
        unimplemented!()
    }

    fn invoke_method(
        &self,
        entity: Box<IBaseEntity>,
        func: &str,
        args: &IArrayList,
        res: &IArrayList,
    ) -> bool {
        unimplemented!()
    }

    fn run_function(&self, func: &str, args: &IArrayList, res: &IArrayList) -> bool {
        unimplemented!()
    }

    fn exec_async_proc(
        &self,
        script: &str,
        func: &str,
        args: &IArrayList,
        res: &IArrayList,
    ) -> bool {
        unimplemented!()
    }

    fn find_async_proc(&self, script: &str, func: &str, id: &ObjId) -> bool {
        unimplemented!()
    }

    fn kill_async_proc(&self, script: &str, func: &str, id: &ObjId) -> bool {
        unimplemented!()
    }

    fn gen_async_event(&self, id: &ObjId, event: &str, args: &IArrayList) -> u32 {
        unimplemented!()
    }

    fn bind_script(&self, entity: Box<IBaseEntity>, script: &str) -> bool {
        unimplemented!()
    }

    fn bind_logic(&self, entity: Box<IBaseEntity>, logic: &str, args: &IArrayList) -> bool {
        unimplemented!()
    }

    fn find_callback(&self, entity: Box<IBaseEntity>, event: &str) -> bool {
        unimplemented!()
    }

    fn exec_callback(
        &self,
        entity: Box<IBaseEntity>,
        event: &str,
        args: &IArrayList,
        res: &IArrayList,
    ) -> bool {
        unimplemented!()
    }

    fn add_execute(&self, entity: Box<IBaseEntity>) -> bool {
        unimplemented!()
    }

    fn remove_execute(&self, entity: &Box<IBaseEntity>) -> bool {
        unimplemented!()
    }

    fn find_global_value(&self, name: &str) -> bool {
        unimplemented!()
    }

    fn remove_global_value(&self, name: &str) -> bool {
        unimplemented!()
    }

    fn set_global_value(&self, name: &str, value: &IAny) -> bool {
        unimplemented!()
    }

    fn get_global_value(&self, name: &str) -> IAny {
        unimplemented!()
    }

    fn get_global_count(&self) -> u32 {
        unimplemented!()
    }

    fn get_global_list(&self, result: &mut IArrayList) -> u32 {
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
