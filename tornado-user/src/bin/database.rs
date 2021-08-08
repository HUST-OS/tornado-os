#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;
// use core::future::Future;
// use core::pin::Pin;
// use core::task::{Context, Poll};
use alloc::string::String;

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

/*
控制台语法：

inputs = { SOI ~ input* ~ EOI }
input = _{ command ~ blank ~ ";" ~ blank }
command = _{ select | insert | show }
select = { "select" ~ " " ~ column_selector ~ " " ~
    "from" ~ " " ~ table ~ blank ~
    (where_clause)? }
insert = { "insert" ~ " " ~ "into" ~ " " ~ table ~ blank ~ insert_content }
show = { "show tables" }

where_clause = { "where" ~ " " ~ equation }
equation = {  left_value ~ blank~"="~blank~right_value }
left_value = { ident }
right_value = { ident }

insert_content = { "(" ~columns ~ ")"~blank~
    "values"~blank~"("~columns~")" }

column_selector = { "*" | columns }
columns = { column ~ (comma ~ column)+ }
column = { ident }
table = { ident }

comma = _{ blank ~ "," ~ blank }

alpha = _{ 'a'..'z' | 'A'..'Z' }
digit = _{ '0'..'9' }

ident = _{ (alpha | digit | "_" )+ }

blank = _{ (" " | "\r" | "\n")* }

解析结果，以普通语句为例：

select * from table where a = 1;
insert into table (a, b) values (1, 2);
show tables;

- inputs
  - select
    - column_selector: "*"
    - table: "table"
    - where_clause > equation
      - left_value: "a"
      - right_value: "1"
  - insert
    - table: "table"
    - insert_content
      - columns
        - column: "a"
        - column: "b"
      - columns
        - column: "1"
        - column: "2"
  - show: "show tables"
  - EOI: ""
*/


use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "bin/database-console.pest"]
struct ConsoleParser;

async fn async_main() -> i32 {
    // let stdin = tornado_user::stdin();
    let mut buf = alloc::vec![0u8; 1024];
    print_welcome();
    loop {
        println!("[>] 请输入查询、操作或枚举语句来继续，使用q退出。");
        let len = read_line(&mut buf); // stdin.read_line(&mut buf);
        let cmd = String::from_utf8_lossy(&buf[..len]);
        // todo
        println!("[<] 您输入的指令: {}", cmd);
        if cmd.as_ref() == "q" {
            println!("[·] 程序退出，感谢再次使用！");
            break
        }
        match ConsoleParser::parse(Rule::inputs, cmd.trim()) {
            Ok(mut pairs) => match pairs.next().map(|p| p.as_rule()) {
                Some(Rule::inputs) => match pairs.next() {
                    Some(r) => println!("[] rule: {:?}", r),
                    _ => println!("[!] todo")
                }
                _ => unreachable!()
            },
            Err(e) => {
                println!("[!] 无法识别的指令: {}。错误：{}", cmd, e);
            }
        }
    }
    0
}

fn read_line(bytes: &mut [u8]) -> usize {
    let mut input = tornado_user::syscall::sys_test_read_one().extra;
    let mut len = 0;
    while input != 13 && len < bytes.len() {
        // println!("[{}]", input);
        // if input == 127 { // 退格
        //     if len != 0 {
        //         tornado_user::syscall::sys_test_write_one(8);
        //         len -= 1;
        //     }
        //     continue
        // } // 暂不支持退格
        tornado_user::syscall::sys_test_write_one(input); // 回显
        bytes[len] = input as u8;
        len += 1;
        input = tornado_user::syscall::sys_test_read_one().extra;
    }
    tornado_user::syscall::sys_test_write_one(10);
    tornado_user::syscall::sys_test_write_one(13); // 换行
    len
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
