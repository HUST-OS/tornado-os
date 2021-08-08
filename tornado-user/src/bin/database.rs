#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;

use alloc::string::String;
use alloc::vec::Vec;

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

#[derive(Debug)]
enum Command<'i> {
    Select(Vec<&'i str>, &'i str, Option<Where<'i>>), // select [*] from [table]
    Insert(&'i str, Vec<(&'i str, &'i str)>), // insert into [table] (字段1, 字段2) values (值1, 值2);
    Delete(&'i str, Where<'i>), // delete from 表格 where 字段1 = 值1;
    ShowTables, // show tables;
    Create(&'i str, Vec<(&'i str, &'i str)>), // create table 表格 (字段1 integer, 字段2 integer);
    Drop(&'i str), // drop table [table];
    Describe(&'i str), // describle [table];
}

#[derive(Debug)]
struct Where<'i> { // where [left] = [right] 
    left: &'i str, 
    right: &'i str,
}

use pest_derive::Parser;
use pest::Parser;
use pest::iterators::Pairs;

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
        if cmd.as_ref() == "q" || cmd.as_ref() == "exit" {
            println!("[·] 程序退出，感谢再次使用！");
            break
        }
        let parse_result = ConsoleParser::parse(Rule::inputs, cmd.trim());
        if let Ok(mut pairs) = parse_result {
            let pair = pairs.next();
            if pair == None {
                unreachable!()
            }
            let pair = pair.unwrap();
            if pair.as_rule() != Rule::inputs {
                unreachable!();
            }
            let inner_pairs = pair.into_inner();
            // println!("{:?}", inner_pairs);
/*[
Pair { rule: select, span: "select * from tables" , inner: [
    Pair { rule: column_selector, span: Span { str: "*", start: 7, end: 8 }, inner: [] }, 
    Pair { rule: table, span: Span { str: "tables", start: 14, end: 20 }, inner: [] }
] }, 
Pair { rule: select, span: "select * from tables" }, inner: [
    Pair { rule: column_selector, span: Span { str: "*", start: 29, end: 30 }, inner: [] },
    Pair { rule: table, span: Span { str: "tables", start: 36, end: 42 }, inner: [] }
] }, 
Pair { rule: EOI, span: Span { str: "", start: 43, end: 43 }, inner: [] }
] */                    
            let mut commands = Vec::new();
            for pair in inner_pairs {
                // println!("{:?}", pair);
                // println!("{:?}", pair.as_rule());
                match pair.as_rule() {
                    Rule::select => commands.push(parse_select(pair.into_inner())),
                    Rule::create => commands.push(parse_create(pair.into_inner())),
                    Rule::EOI => break,
                    _ => todo!()
                }
            }
            println!("Commands: {:?}", commands);
        } else { // if let Ok(mut pairs) = parse_result
            let e = parse_result.unwrap_err(); // 一定是Err(e)
            println!("[!] 无法识别的指令: {}。错误：{}", cmd, e);
        }
    }
    0
}

fn parse_select<'i>(select_pairs: Pairs<'i, Rule>) -> Command {
    let mut var_list = Vec::new();
    let mut table_name = "";
    let mut where_clause = None;
    for select_pair in select_pairs {
        match select_pair.as_rule() {
            Rule::columns | Rule::column_selector => {
/*[
Pair { rule: columns, span: Span { str: "a, b", start: 7, end: 11 }, inner: [
Pair { rule: column, span: Span { str: "a", start: 7, end: 8 }, inner: [] }, 
Pair { rule: column, span: Span { str: "b", start: 10, end: 11 }, inner: [] }
] }
] */
                // let column_pairs = select_pair.into_inner();
                // println!("[[]] {:?}", column_pairs);
                for column_pair in select_pair.into_inner() {
                    // println!("[] {:?}", column_pair.as_str());
                    // println!("[] {:?}", column_pair);
                    var_list.push(column_pair.as_str());
                }
                // 如果inner是“*”，var_list为空
            }
            Rule::table => table_name = select_pair.as_span().as_str(),
            Rule::where_clause => {
                where_clause = Some(parse_where_clause(select_pair.into_inner()))
            },
            _ => unreachable!()
        }
    }
    Command::Select(var_list, table_name, where_clause)
}

fn parse_where_clause<'i>(where_pairs: Pairs<'i, Rule>) -> Where {
    /*
    - where_clause > equation
      - left_value: "a"
      - right_value: "1"
    */
    let mut left_value = "";
    let mut right_value = "";
    for where_pair in where_pairs {
        if where_pair.as_rule() != Rule::equation {
            continue; // 其它的where语句暂不支持
        }
        for equation_pair in where_pair.into_inner() {
            match equation_pair.as_rule() {
                Rule::left_value => left_value = equation_pair.as_str(),
                Rule::right_value => right_value = equation_pair.as_str(),
                _ => unreachable!()
            }
        }
    }
    Where { left: left_value, right: right_value }
}

/*
- table: "a"
- create_content
  - parameter
    - column: "b"
    - data_type: "integer"
  - parameter
    - column: "c"
    - data_type: "integer"
*/
fn parse_create<'i>(create_pairs: Pairs<'i, Rule>) -> Command {
    let mut table_name = "";
    let mut name_type_list = Vec::new();
    for create_pair in create_pairs {
        match create_pair.as_rule() {
            Rule::create_content => {
                for create_content_pair in create_pair.into_inner() {
                    let mut column = "";
                    let mut data_type = "";
                    for parameter_pair in create_content_pair.into_inner() {
                        match parameter_pair.as_rule() {
                            Rule::column => column = parameter_pair.as_str(),
                            Rule::data_type => data_type = parameter_pair.as_str(),
                            _ => unreachable!()
                        }
                    }
                    name_type_list.push((column, data_type));
                }
            }
            Rule::table => table_name = create_pair.as_span().as_str(),
            _ => unreachable!()
        }
    }
    Command::Create(table_name, name_type_list)
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
