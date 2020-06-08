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
use crate::core::base_interface::IBaseInterface;
use crate::core::entity_info::IEntityInfo;
use crate::core::base_entity::IBaseEntity;
use crate::share::any_list::IArrayList;
use crate::share::obj_id::ObjId;
use crate::share::any::IAny;

pub(crate) trait ICore {
    // 获取实例
    fn get_instance(&self);

    // 获取工作路径
    fn get_work_path(&self) -> String;

    // 获取脚本路劲
    fn get_script_path(&self) -> String;

    // 获取资源路径
    fn get_resource_path(&self) -> String;

    // 获取主配置文件路径
    fn get_main_config_path(&self) -> String;

    // 主循环休眠时间
    fn set_sleep_time(&self, value: i32);
    fn get_sleep_time(&self) -> i32;

    // 程序退出
    fn set_quit(&self, value: bool);
    fn get_quit(&self) -> bool;

    // 运行主循环
    fn execute(&self) -> bool;

    // 日志文件
    fn set_log_type(&self, log_type: i32);
    fn get_log_type(&self) -> i32;
    fn trace_log(&self, log: &str, log_type: i32);

    // 导出程序信息文件
    fn dump_file(&self, file_type: &str, file: &str);

    // 查找功能接口
    fn find_interface(&self, name: &str);
    // 获取功能接口
    fn get_interface<T: IBaseInterface>(&self, name: &str) -> Option<T>;
    // 获取统一名字空间中的功能接口
    fn get_interface_same_space<T: IBaseInterface>(&self, p_bi: &T, name: &str);

    // 释放功能接口
    fn release_interface<T: IBaseInterface>(&self, bi: &T);

    // 查找实体类信息
    fn get_entity_info<T: IEntityInfo>(&self, name: &str) -> Option<T>;

    // 获得住实体
    fn get_main_entity<T: IBaseEntity>(&self) -> Option<T>;

    // 获得实体
    fn get_entity<T: IBaseEntity>(&self) -> Option<T>;

    // 获得所有的实体对象ID
    fn get_all_entity<T: IArrayList>(&self, result: &mut T) -> u32;

    // 查找名字符合的第一个实体
    fn lookup_entity<T: IBaseEntity>(&self, name: &str) -> Option<T>;

    // 查找名字复合的所有实体
    fn lookup_entity_more<T:IArrayList>(&self, name: &str, result: &mut T) -> u32;

    // 创建实体
    fn create_entity<T: IBaseEntity>(&self, name: &str) -> Option<T>;

    // 带参数创建实体
    fn create_entity_args<T: IBaseEntity, U: IArrayList>(&self, name: &str, args: &U) -> Option<T>;

    // 穿件统一名字空间中的实体
    fn create_entity_same_space<T: IBaseEntity, U: IArrayList>(&self, entity: &T, name: &str, args: &U) -> * dyn IBaseEntity;

    // 删除实体
    fn delete_entity(&self, id: &ObjId) -> bool;

    // 是否允许实体被脚本删除
    fn set_can_del_by_script<T: IBaseEntity>(&self, entity: &T, value: bool);

    // 获得实体属性
    fn get_property<T: IBaseEntity, U: IAny>(&self, entity: &T, prop: &str, value: &mut U) -> bool;

    // 设置实体属性
    fn set_property<T: IBaseEntity, U: IAny>(&self, entity: &T, prop: &str, value: &U) -> bool;

    // 调用实体方法
    fn invoke_method<T: IBaseEntity, U: IArrayList>(&self, entity: &T, func: &str, args: &U, res: &U) -> bool;

    // 运行脚本扩展函数
    fn run_function<T: IArrayList>(&self, func: &str, args: &T, res: &T) -> bool;

    // 运行异步进程
    fn exec_async_proc<T: IArrayList>(&self, script: &str, func: &str, args: &T, res: &T) -> bool;

    // 查找异步进程
    fn find_async_proc(&self, script: &str, func: &str, id: & ObjId) -> bool;

    // 终止异步进城
    fn kill_async_proc(&self, script: &str, func: &str, id: &ObjId) -> bool;

    // 昌盛异步时间，返回被触发的过程数量
    fn gen_async_event<T: IArrayList>(&self, id: & ObjId, event: &str, args: &T) -> u32;

    // 实体绑定脚本
    fn bind_script<T: IBaseEntity>(&self, entity: &T, script: &str) -> bool;

    // 实体绑定逻辑类
    fn bind_logic<T: IBaseEntity, U: IArrayList>(&self, entity: &T, logic: &str, args: &U) -> bool;

    // 查找脚本回调
    fn find_callback<T: IBaseEntity>(&self, entity: &T, event: &str) -> bool;

    // 执行脚本回调
    fn exec_callback<T: IBaseEntity, U: IArrayList>(&self, entity: &T, event: &str, args: &U, res: &U) -> bool;

    // 添加到运行队列
    fn add_execute<T: IBaseEntity>(&self, entity: &T) -> bool;

    // 从运行队列移除
    fn remove_execute<T: IBaseEntity>(&self, entity: &T) -> bool;

    // 全局变量
    fn find_global_value(&self, name: &str) -> bool;
    fn remove_global_value(&self, name: &str) -> bool;
    fn set_global_value<T: IAny>(&self, name: &str, value: &T) -> bool;
    fn get_global_value<T: IAny>(&self, name: &str) -> Option<T>;
    fn get_global_count(&self) -> u32;
    fn get_global_list<T: IArrayList>(&self, result: &mut T) -> u32;

    // 获得当前帧时间
    fn get_frame_seconds(&self) -> f32;
}

struct Core{}

impl ICore for Core {
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

    fn get_main_entity<T: IBaseEntity>(&self) -> Option<T> {
        unimplemented!()
    }

    fn get_entity<T: IBaseEntity>(&self) -> Option<T> {
        unimplemented!()
    }

    fn get_all_entity<T: IArrayList>(&self, result: &mut T) -> u32 {
        unimplemented!()
    }

    fn lookup_entity<T: IBaseEntity>(&self, name: &str) -> Option<T> {
        unimplemented!()
    }

    fn lookup_entity_more<T: IArrayList>(&self, name: &str, result: &mut T) -> u32 {
        unimplemented!()
    }

    fn create_entity<T: IBaseEntity>(&self, name: &str) -> Option<T> {
        unimplemented!()
    }

    fn create_entity_args<T: IBaseEntity, U: IArrayList>(&self, name: &str, args: &U) -> Option<T> {
        unimplemented!()
    }

    fn create_entity_same_space<T: IBaseEntity, U: IArrayList>(&self, entity: &T, name: &str, args: &U) -> *const dyn IBaseEntity {
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

    fn invoke_method<T: IBaseEntity, U: IArrayList>(&self, entity: &T, func: &str, args: &U, res: &U) -> bool {
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

    fn exec_callback<T: IBaseEntity, U: IArrayList>(&self, entity: &T, event: &str, args: &U, res: &U) -> bool {
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
enum ELogType
{
	ELogDbg = 0 ,		// 调试类日志
    ELogInformation = 1 ,		// 信息类日志
    ELogWarning = 2 ,		// 警告类日志
    ELogError = 3 ,		// 错误类日志(程序可以继续执行)
    ELogFatal = 4 ,		// 错误类日志(程序没法继续执行)
}