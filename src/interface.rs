#![allow(dead_code, unused_variables)]

use crate::structs::{OrderRequest, CancelRequest, LoginForm};
use crate::app::CtpbeeR;
use actix::Addr;

/// 用户登录接口,包含用户的\
pub trait Interface {
    /// 发单
    fn send_order(&mut self, order: OrderRequest) -> String { unimplemented!() }
    /// 撤单
    fn cancel_order(&mut self, req: CancelRequest) -> String { unimplemented!() }
    /// 登录接口
    fn connect(&mut self, req: LoginForm) -> bool { unimplemented!() }
    /// 订阅行情
    fn subscribe(&mut self, symbol: String) -> bool { unimplemented!() }
    /// 释放退出接口
    fn exit(&mut self) { unimplemented!() }
    /// 获取到App
    fn get_app(&mut self) -> Addr<CtpbeeR> {
        unimplemented!()
    }
}

