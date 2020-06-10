/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       gravity.rs.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/18
//  Description:     引力（核心接口）
//  Others:
//  History
*************************************************/

use super::black_hole_param::{LogicClassFunc, LogicEventFunc};
use super::logic_module::ILogicModule;
use super::object::IObject;
use crate::share::any_list::IArrayList;
use crate::share::obj_id::ObjId;

#[derive(Debug)]
pub(crate) struct IGravity {}

impl IGravity {
    // 获得指定名称的逻辑模块
    fn get_logic_module(&self, name: &str) -> Box<ILogicModule> {}

    // 获得指针对象
    fn get_obj(&self, id: &ObjId) -> Box<IObject> {}

    // 获得场景对象指针
    fn get_current_scene_obj(&self) -> Box<IObject> {}

    // 获取当前场景对象
    fn get_current_scene_obj_id(&self) -> Box<IObject> {}

    // 获取当前场景ID
    fn get_current_scene_id(&self) -> i32 {}

    // 获取最大的普通场景ID
    fn get_max_normal_scene_id(&self) -> i32 {}

    // 获得普通场景逻辑类名
    fn get_scene_script(&self, id: i32) -> String {}

    // 查找指定配置名的普通场景编号
    fn find_scene_id(&self, config: &str) -> i32 {}

    // 判断指定编号的场景是否存在
    fn is_scene_exists(&self, id: i32) -> bool {}

    // 获得指定场景内的在线玩家数量
    fn get_online_role_number(&self, id: i32) -> i32 {}

    // 获取指定场景内的玩家数量（包括在线和离线）
    fn get_all_role_number(&self, id: i32) -> i32 {}

    // 获取指定场景内玩家唯一id列表
    fn get_guid_list(&self, id: i32, result: &mut IArrayList) -> i32 {}

    // 获取指定场景类型
    fn get_scene_type(&self, id: i32) -> i32 {}

    // 获取分流场景号
    fn get_diff_scene_id(&self, id: i32) -> i32 {}

    // 获取主场景好
    fn get_scene_main_id(&self, id: i32) -> i32 {}

    // 获取分流场景允许的最大玩家数
    fn get_scene_max_players(&self, id: i32) -> i32 {}

    // 获取所有分流场景号
    fn get_diff_scenes(&self, main_id: i32, diff_scenes: &mut IArrayList) {}

    // 请求创建副本场景，在OnCloneScene回调里返回结果
    fn request_clone_scene(
        &self,
        prototype_scene_id: i32,
        guid: i64,
        down_time: i32,
        reuse: bool,
        args: &str,
    ) -> bool {
    }

    // 设置副本场景的回收时间
    fn set_clone_scene_close_time(&self, clone_scene_id: i32, close_time: i32) -> bool {}

    // 获取指定原型编号的副本数量
    fn get_prototype_scene_id(&self, clone_scene_id: i32) -> i32 {}

    // 指定场景好是否是副本场景的原型场景
    fn is_prototype_scene(&self, scene_id: i32) -> bool {}

    // 根据角色名获得角色guid,返回0表示角色不存在
    fn find_role_guid(&self, role_name: &str) -> i64 {}

    // 根据角色guid, 获取角色名，返回空字符串表示角色不存在
    fn find_role_name(&self, role_guid: i64) -> String {}

    // 获得角色是否已被删除
    fn is_role_deleted(&self, role_guid: i64) -> bool {}

    /***
     ******** 地形相关
     */

    // 获得是否可站立
    fn get_can_stand(&self, x: f32, y: f32, z: f32) -> bool {}

    // 获取是否客行走
    fn get_can_walk(&self, x: f32, y: f32, z: f32) -> bool {}

    // 取地图范围
    fn get_map_bound(&self, left: f32, top: f32, right: f32, bottom: f32) -> bool {}

    // 获取地图高度Y
    fn get_map_height(&self, x: f32, z: f32) -> f32 {}

    // 获取范围标记是否有效
    fn get_map_region(&self, name: &str, x: f32, z: f32) -> bool {}

    // 取区域名称
    fn get_map_area(&self, x: f32, z: f32) -> String {}

    // 取地表类型
    fn get_map_type(&self, x: f32, y: f32, z: f32) -> i32 {}

    // 取行走类型
    fn get_walk_type(&self, x: f32, z: f32) -> i32 {}

    // 测试一个点是否是客行走区域
    fn can_walk(&self, x: f32, z: f32) -> bool {}

    // 测试一个点对象是否可以行走
    fn object_can_walk(&self, obj: &ObjId, x: f32, z: f32) -> bool {}

    // 直线移动测试
    fn trace_line_walk(
        &self,
        walk_step: f32,
        src_x: f32,
        src_y: f32,
        src_z: f32,
        dst_x: f32,
        dst_y: f32,
        dst_z: f32,
        new_x: f32,
        new_y: f32,
        new_z: f32,
    ) -> bool {
    }

    // 碰撞数据是否就绪
    fn get_collide_enable(&self, x: f32, z: f32) -> bool {}

    // 获取指定位置的最高点高度
    fn get_apex_height(&self, x: f32, z: f32) -> f32 {}

    // 获取指定位置的最高点所在层
    fn get_apex_floor(&self, x: f32, z: f32) -> i32 {}

    // 获取地面高度
    fn get_ground_height(&self, x: f32, z: f32) -> f32 {}

    // 获得是否客行走
    fn get_walk_enable(&self, x: f32, z: f32) -> bool {}

    // 获取客行走高度
    fn get_walk_height(&self, x: f32, z: f32) -> f32 {}

    // 水面是否存在
    fn get_walk_water_exists(&self, x: f32, z: f32) -> bool {}

    // 获得精确的睡眠高度
    fn get_walk_water_height(&self, x: f32, z: f32) -> f32 {}

    // 获取总的层数量
    fn get_floor_count(&self, x: f32, z: f32) -> i32 {}

    // 获得层是否存在
    fn get_floor_exists(&self, x: f32, z: f32, floor: i32) -> bool {}

    // 获得层是否可移动
    fn get_floor_can_move(&self, x: f32, z: f32, floor: i32) -> bool {}

    // 获取层是否可以战力
    fn get_floor_can_stand(&self, x: f32, z: f32, floor: i32) -> bool {}

    // 获取层高度
    fn get_floor_height(&self, x: f32, z: f32, floor: i32) -> f32 {}

    // 获取层的无张海空间高度
    fn get_floor_space(&self, x: f32, z: f32, floor: i32) -> f32 {}

    // 获取层是否有墙面
    fn get_floor_has_wall(&self, x: f32, z: f32, floor: i32) -> bool {}

    // 获得指定高度位置是否存在墙面
    fn get_wall_exists(&self, x: f32, y: f32, z: f32) -> bool {}

    // 寻路
    fn point_find_path(
        &self,
        find_mode: i32,
        src_x: f32,
        src_y: f32,
        src_z: f32,
        dst_x: f32,
        dst_y: f32,
        dst_z: f32,
        point_list: &mut IArrayList,
    ) {
    }

    /***
     ******** 逻辑相关
     */
    // 逻辑类添加
    fn add_logic_class(&self, logic_class: &str, class_type: i32, parent_class: &str) -> bool {}

    // 添加逻辑类回调
    fn add_class_callback(
        &self,
        logic_class: &str,
        event: &str,
        func: &LogicClassFunc,
        prior: i32,
    ) -> bool {
    }

    // 添加逻辑事件回调
    fn add_event_callback(
        &self,
        logic_class: &str,
        event: &str,
        func: LogicEventFunc,
        prior: i32,
    ) -> bool {
    }

    // 添加字符串编号的对象命令处理函数
    fn add_command_hook(
        &self,
        logic_class: &str,
        msg_id: &str,
        func: LogicEventFunc,
        prior: i32,
    ) -> bool {
    }

    // 添加证书编号对象命令处理函数
    fn add_int_command_hook(
        &self,
        logic_class: &str,
        msg_id: i32,
        func: &LogicEventFunc,
        prior: i32,
    ) -> bool {
    }

    // 添加字符串编号的对象客户端消息处理函数
    fn add_custom_hook(
        &self,
        logic_class: &str,
        msg_id: &str,
        func: &LogicEventFunc,
        prior: i32,
    ) -> bool {
    }

    // 添加证书编号的对象客户端消息处理函数
    fn add_int_custom_hook(
        &self,
        logic_class: &str,
        msg: i32,
        func: &LogicEventFunc,
        prior: i32,
    ) -> bool {
    }

    // 执行指定编号的对象客户端回调处理函数
    fn run_int_custom_hook(&self, obj_id: &ObjId, msg_id: i32, args: &IArrayList) -> bool {}

    // 执行指定对象的事件回调函数函数
    fn run_event_callback(
        &self,
        event: &str,
        obj_id: &ObjId,
        sender: &ObjId,
        args: &IArrayList,
    ) -> bool {
    }
}
