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
trait ICore {
    // 获取实例
    fn get_instance();

    // 获取工作路径
    fn get_work_path() -> &str;

    // 获取脚本路劲
    fn get_script_path() -> &str;

    // 获取资源路径
    fn get_resource_path() -> &str;

    // 获取主配置文件路径
    fn get_main_config_path() -> &str;

    // 主循环休眠时间
    fn set_sleep_time(value: i32);
    fn get_sleep_time() -> i32;

    // 程序退出
    fn set_quit(value: bool);
    fn get_quit() -> bool;

    // 运行主循环
    fn excute() -> bool;

    // 日志文件
    fn set_log_type(log_type: i32);
    fn get_log_type() -> i32;
    fn trace_log(log: &str, log_type: i32);

    // 导出程序信息文件
    fn dump_file(file_type: &str, file: &str);

    // 查找功能接口
    fn find_interface(name: &str);
    // 获取功能接口
    fn get_interface(name: &str) -> *i_base_interface;
    // 获取统一名字空间中的功能接口
    fn get_interface_same_space(p_bi: *i_base_interface, name: &str);

    // 释放功能接口
    fn release_interface(p_bi: *i_base_interface);

    // 查找实体类信息
    fn get_entity_info(name: &str) -> *i_entity_info;
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