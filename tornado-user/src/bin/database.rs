#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

/*
飓风数据库。支持的语法示例：

- 展示数据库系统内所有的表格
show tables;
- 创建新表格
create table 表格 (字段1 integer, 字段2 integer);
- 删除表格
drop table 表格;
- 描述某一个表格
describe 表格;
- 得到表格内的数据
select 字段1, 字段2 from 表格;
select * from 表格;
- 得到表格内符合要求的数据
select * from 表格 where 字段1 = 值1;
- 插入新行
insert into 表格 (字段1, 字段2) values (值1, 值2);
- 删除行
delete from 表格 where 字段1 = 值1;
*/

async fn async_main() -> i32 {
    let stdin = tornado_user::stdin();
    let mut buf = alloc::string::String::new();
    print_welcome();
    loop {
        println!("[>] 请输入查询、操作或枚举语句来继续，使用q退出。");
        let len = stdin.read_line(&mut buf);
        let cmd = &buf[..len];
        // todo
        println!("[<] input: {}", cmd);
    }
    0
}

fn print_welcome() {
    println!("[·] 欢迎使用飓风数据库!");
    println!("[·] 您可以输入select、insert和delete语句来查询。");
    println!("[·] 操作表格，输入create、drop或describe语句。");
    println!("[·] 使用show tables来枚举所有的表格。");
}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    tornado_user::execute_async_main(async_main())
}
