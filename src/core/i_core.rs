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
trait i_core {
    // 获取实例
    pub fn get_instance();

    // 获取工作路径
    pub fn get_work_path() -> &str;

    // 获取脚本路劲
    pub fn get_script_path() -> &str;

    // 获取资源路径
    pub fn get_resource_path() -> &str;

    // 获取主配置文件路径
    pub fn get_main_config_path -> &str;

    // 主循环休眠时间
    pub fn set_sleep_time(value i32);
    pub fn get_sleep_time() -> i32;

    // 程序退出
    pub fn set_quit(value bool);
    pub fn get_quit() -> bool;

    // 运行主循环
    pub fn excute() -> bool;

    // 日志文件
    pub fn set_log_type(log_type: i32);
    pub fn get_log_type() -> i32;
    pub fn trace_log(log: &str, log_type: i32);

    // 导出程序信息文件
    pub dump_file(type: &str, file: &str);

    // 查找功能接口
    pub find_interface(name: &str);
    // 获取功能接口
    pub get_interface(name: &str) -> *bi;
    // 获取统一名字空间中的功能接口
    pub get_interface_same_space(p_bi: *bi, name: &str);

    // 释放功能接口
    pub release_interface(p_bi: *bi);

    // 查找实体类信息
    pub get_entity_info(name: &str) -> *i_entity_info;
}

#[derive(Debug)]
enum E_LOG_TYPE
{
	E_LOG_DBG				= 0 ,		// 调试类日志
	E_LOG_INFORMATION		= 1 ,		// 信息类日志
	E_LOG_WARNING			= 2 ,		// 警告类日志
	E_LOG_ERROR				= 3 ,		// 错误类日志(程序可以继续执行)
	E_LOG_FATAL				= 4 ,		// 错误类日志(程序没法继续执行)
};