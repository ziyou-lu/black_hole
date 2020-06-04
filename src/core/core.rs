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
    fn get_interface(&self, name: &str) -> * dyn IBaseInterface;
    // 获取统一名字空间中的功能接口
    fn get_interface_same_space(&self, p_bi: * dyn IBaseInterface, name: &str);

    // 释放功能接口
    fn release_interface(&self, p_bi: * dyn IBaseInterface);

    // 查找实体类信息
    fn get_entity_info(&self, name: &str) -> * dyn IEntityInfo;

    // 获得住实体
    fn get_main_entity(&self) -> * dyn IBaseEntity;

    // 获得实体
    fn get_entity(&self) -> * dyn IBaseEntity;

    // 获得所有的实体对象ID
    fn get_all_entity(&self, result: &dyn IArrayList) -> u32;

    // 查找名字符合的第一个实体
    fn lookup_entity(&self, name: &str) -> *dyn IBaseEntity;

    // 查找名字复合的所有实体
    fn lookup_entity_more(&self, name: &str, result: &mut dyn IArrayList) -> u32;

    // 创建实体
    fn create_entity(&self, name: &str) -> * dyn IBaseEntity;

    // 带参数创建实体
    fn create_entity_args(&self, name: &str, args: & dyn IArrayList) -> *dyn IBaseEntity;

    // 穿件统一名字空间中的实体
    fn create_entity_same_space(&self, entity: * dyn IBaseEntity, name: &str, args: &dyn IArrayList) -> * dyn IBaseEntity;

    // 删除实体
    fn delete_entity(&self, id: &ObjId) -> bool;

    // 是否允许实体被脚本删除
    fn set_can_del_by_script(&self, entity: * dyn IBaseEntity, value: bool);

    // 获得实体属性
    fn get_property(&self, entity: * dyn IBaseEntity, prop: &str, value: &mut dyn IAny) -> bool;

    // 设置实体属性
    fn set_property(&self, entity: * dyn IBaseEntity, prop: &str, value: & dyn IAny) -> bool;

    // 调用实体方法
    fn invoke_method(&self, entity: * dyn IBaseEntity, func: &str, args: & dyn IArrayList, res: &mut dyn IArrayList) -> bool;

    // 运行脚本扩展函数
    fn run_function(&self, func: &str, args: & dyn IArrayList, res: &mut dyn IArrayList) -> bool;

    // 运行异步进程
    fn exec_async_proc(&self, script: &str, func: &str, args: & dyn IArrayList, res: * dyn IArrayList) -> bool;

    // 查找异步进程
    fn find_async_proc(&self, script: &str, func: &str, id: & ObjId) -> bool;

    // 终止异步进城
    fn kill_async_proc(&self, script: &str, func: &str, id: &ObjId) -> bool;

    // 昌盛异步时间，返回被触发的过程数量
    fn gen_async_event(&self, id: & ObjId, event: &str, args: & dyn IArrayList) -> u32;

    // 实体绑定脚本
    fn bind_script(&self, entity: * dyn IBaseEntity, script: &str) -> bool;

    // 实体绑定逻辑类
    fn bind_logic(&self, entity: * dyn IBaseEntity, logic: &str, args: & dyn IArrayList) -> bool;

    // 查找脚本回调
    fn find_callback(&self, entity: * dyn IBaseEntity, event: &str) -> bool;

    // 执行脚本回调
    fn exec_callback(&self, entity: * dyn IBaseEntity, event: &str, args: & dyn IArrayList, res: * dyn IArrayList) -> bool;

    // 添加到运行队列
    fn add_execute(&self, entity: * dyn IBaseEntity) -> bool;

    // 从运行队列移除
    fn remove_execute(&self, entity: * dyn IBaseEntity) -> bool;

    // 全局变量
    fn find_global_value(&self, name: &str) -> bool;
    fn remove_global_value(&self, name: &str) -> bool;
    fn set_global_value(&self, name: &str, value: & dyn IAny) -> bool;
    fn get_global_value(&self, name: &str) -> Box<dyn IAny>;
    fn get_global_count(&self) -> u32;
    fn get_global_list(&self, result: &mut dyn IArrayList) -> u32;

    // 获得当前帧时间
    fn get_frame_seconds(&self) -> f32;
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