#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate tornado_user;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use hashbrown::HashMap;

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
    Delete(&'i str, Where<'i>),               // delete from 表格 where 字段1 = 值1;
    ShowTables,                               // show tables;
    Create(&'i str, Vec<(&'i str, &'i str)>), // create table 表格 (字段1 integer, 字段2 integer);
    Drop(&'i str),                            // drop table [table];
    Describe(&'i str),                        // describle [table];
    Error,
}

#[derive(Debug)]
struct Where<'i> {
    // where [left] = [right]
    left: &'i str,
    right: &'i str,
}

async fn async_main() -> i32 {
    // let stdin = tornado_user::stdin();
    let mut buf = alloc::vec![0u8; 1024];
    let mut database = Database::new();
    init_database(&mut database);
    print_welcome();
    loop {
        println!("[>] 请输入查询、操作或枚举语句来继续，使用q退出。");
        let len = read_line(&mut buf); // stdin.read_line(&mut buf);
        let cmd = String::from_utf8_lossy(&buf[..len]);
        // todo
        println!("[<] 您输入的指令：{}", cmd);
        if cmd.as_ref() == "q" || cmd.as_ref() == "exit" {
            println!("[·] 程序退出，感谢再次使用！");
            break;
        }
        let parse_result = parse_commands(&cmd);
        if let Err(ref e) = parse_result {
            println!("[!] 无法识别的指令：{}。错误：{}", cmd, e);
            continue;
        }
        let commands = parse_result.unwrap();
        execute_commands(&mut database, commands);
    }
    0
}

struct Database {
    // 简单的数据库引擎
    tables: HashMap<String, Table>,
}

impl Database {
    #[inline]
    fn new() -> Database {
        Database {
            tables: HashMap::new(),
        }
    }
}

struct Table {
    fields: Vec<String>,
    values: Vec<i64>, // 访问行i（从1开始）的值：values[fields.len() * (i - 1)]开始的fields.len()个
    deleted: Vec<usize>,
}

fn execute_commands(database: &mut Database, commands: Vec<Command<'_>>) {
    // println!("Commands: {:?}", commands);
    for command in commands {
        execute_command(database, command)
    }
}

fn execute_command(database: &mut Database, command: Command<'_>) {
    match command {
        Command::Create(table_name, field_type_list) => {
            if database.tables.contains_key(table_name) {
                // 如果相同名字的表存在，返回错误
                println!("[!] 无法创建新表格，因为表格 {} 已存在。", table_name);
                return;
            }
            let mut fields: Vec<_> = field_type_list
                .iter()
                .map(|(n, _t)| n.to_string())
                .collect();
            fields.sort(); // 排序，方便后续插入操作
            let new_table = Table {
                fields,
                values: Vec::new(),
                deleted: Vec::new(),
            };
            database.tables.insert(table_name.to_string(), new_table);
            println!("[>] 成功创建表格 {} 。", table_name);
        }
        Command::Drop(table_name) => {
            if !database.tables.contains_key(table_name) {
                println!("[!] 数据库中不存在表格 {} ，删除失败。", table_name);
                return;
            }
            database.tables.remove(table_name);
            println!("[>] 成功删除表格 {}。", table_name);
        }
        Command::ShowTables => {
            if database.tables.is_empty() {
                println!("[>] 数据库中没有表格。");
                return;
            }
            let len = database.tables.len();
            let table_names = database
                .tables
                .keys()
                .map(|a| a.as_ref())
                .collect::<Vec<_>>()
                .join(", ");
            println!("[>] 数据库中有{}个表格。分别是：{}。", len, table_names);
        }
        Command::Describe(table_name) => {
            if !database.tables.contains_key(table_name) {
                println!("[!] 数据库中不存在表格 {} 。", table_name);
                return;
            }
            let table = database.tables.get(table_name).unwrap();
            println!("[·] | 字段 | 类型 |");
            for field_name in &table.fields {
                println!("[·] | {} | integer |", field_name);
            }
        }
        Command::Insert(table_name, mut kv_pairs) => {
            if !database.tables.contains_key(table_name) {
                println!("[!] 数据库中不存在表格 {} ，插入失败。", table_name);
                return;
            }
            let table = database.tables.get_mut(table_name).unwrap();
            for (ref key, _) in kv_pairs.iter() {
                if table
                    .fields
                    .binary_search_by(|field| field.as_str().cmp(key))
                    .is_err()
                {
                    println!("[!] 插入失败：表格 {} 不包含字段 {}。", table_name, key);
                    return;
                }
            }
            for field in &table.fields {
                if kv_pairs
                    .binary_search_by_key(&field.as_str(), |(k, _v)| k)
                    .is_err()
                {
                    println!(
                        "[!] 插入失败：尝试插入表格 {}，但插入语句中没有提供必要的字段 {}。",
                        table_name, field
                    );
                    return;
                }
            }
            kv_pairs.sort_by(|(ka, _), (kb, _)| ka.cmp(kb)); // 相同的排序方法，确保字段顺序一致
            for (_, value) in kv_pairs.iter() {
                let _value_int: i64 = match value.parse() {
                    Ok(a) => a,
                    Err(_e) => {
                        println!("[!] 插入失败：数据 {} 无法被识别为 i64 整数类型。", value);
                        return;
                    }
                };
            }
            for (_, value) in kv_pairs.iter() {
                let value_int: i64 = value.parse().unwrap();
                table.values.push(value_int);
            }
        }
        Command::Select(fields, table_name, where_clause) => {
            if !database.tables.contains_key(table_name) {
                println!("[!] 查找失败！数据库中不存在表格 {}。", table_name);
                return;
            }
            let table = database.tables.get_mut(table_name).unwrap();
            let mut field_idx_list = Vec::new();
            for field in &fields {
                let search_result = table.fields.binary_search_by(|a| a.as_str().cmp(&field));
                if let Err(_) = search_result {
                    println!(
                        "[!] 查找失败！表格 {} 中不存在名为 {} 的字段。",
                        table_name, field
                    );
                    return;
                }
                let field_idx = search_result.unwrap();
                field_idx_list.push(field_idx);
            }
            if fields.is_empty() {
                // *号的情况
                for field_idx in 0..table.fields.len() {
                    field_idx_list.push(field_idx);
                }
            }
            let mut where_clause_idx = Vec::new();
            let mut where_clause_number = Vec::new();
            if let Some(where_clause) = where_clause {
                let number_result: Result<i64, _> = where_clause.left.parse();
                if let Ok(num) = number_result {
                    where_clause_number.push(num);
                } else {
                    let search_result = table
                        .fields
                        .binary_search_by(|a| a.as_str().cmp(&where_clause.left));
                    if let Err(_) = search_result {
                        println!(
                            "[!] 查找失败！在条件判断语句中，表格 {} 中不存在名为 {} 的字段。",
                            table_name, where_clause.left
                        );
                        return;
                    }
                    where_clause_idx.push(search_result.unwrap());
                }
                let number_result: Result<i64, _> = where_clause.right.parse();
                if let Ok(num) = number_result {
                    where_clause_number.push(num);
                } else {
                    let search_result = table
                        .fields
                        .binary_search_by(|a| a.as_str().cmp(&where_clause.right));
                    if let Err(_) = search_result {
                        println!(
                            "[!] 查找失败！在条件判断语句中，表格 {} 中不存在名为 {} 的字段。",
                            table_name, where_clause.right
                        );
                        return;
                    }
                    where_clause_idx.push(search_result.unwrap());
                }
            }
            // 表格头
            print!("[·] | ");
            for field_idx in &field_idx_list {
                print!("{} | ", table.fields[*field_idx]);
            }
            println!("");
            // 表格内容
            let mut count = 0;
            let table_width = table.fields.len();
            'outer_select: for (row_idx, chunk) in table.values.chunks(table_width).enumerate() {
                if table.deleted.contains(&row_idx) {
                    // 已标记为删除行
                    continue;
                }
                // println!("{:?} {:?}", where_clause_idx, where_clause_number);
                if !where_clause_idx.is_empty() || !where_clause_number.is_empty() {
                    let mut eq_number = None;
                    for idx in &where_clause_idx {
                        if let Some(number) = eq_number {
                            if chunk[*idx] != number {
                                continue 'outer_select;
                            }
                        } else {
                            eq_number = Some(chunk[*idx]);
                        }
                    }
                    for n in &where_clause_number {
                        if let Some(number) = eq_number {
                            if *n != number {
                                continue 'outer_select;
                            }
                        } else {
                            eq_number = Some(*n);
                        }
                    }
                }
                count += 1;
                print!("[·] | ");
                for field_idx in &field_idx_list {
                    print!("{} | ", &chunk[*field_idx]);
                }
                println!("");
            }
            println!("[>] 查询返回 {} 条数据。", count);
        }
        Command::Delete(table_name, where_clause) => {
            if !database.tables.contains_key(table_name) {
                println!("[!] 数据库中不存在表格 {} ，无法删除数据。", table_name);
                return;
            }
            let table = database.tables.get_mut(table_name).unwrap();
            // 预处理where语句
            let mut where_clause_idx = Vec::new();
            let mut where_clause_number = Vec::new();
            {
                let number_result: Result<i64, _> = where_clause.left.parse();
                if let Ok(num) = number_result {
                    where_clause_number.push(num);
                } else {
                    let search_result = table
                        .fields
                        .binary_search_by(|a| a.as_str().cmp(&where_clause.left));
                    if let Err(_) = search_result {
                        println!(
                            "[!] 查找失败！在条件判断语句中，表格 {} 中不存在名为 {} 的字段。",
                            table_name, where_clause.left
                        );
                        return;
                    }
                    where_clause_idx.push(search_result.unwrap());
                }
                let number_result: Result<i64, _> = where_clause.right.parse();
                if let Ok(num) = number_result {
                    where_clause_number.push(num);
                } else {
                    let search_result = table
                        .fields
                        .binary_search_by(|a| a.as_str().cmp(&where_clause.right));
                    if let Err(_) = search_result {
                        println!(
                            "[!] 查找失败！在条件判断语句中，表格 {} 中不存在名为 {} 的字段。",
                            table_name, where_clause.right
                        );
                        return;
                    }
                    where_clause_idx.push(search_result.unwrap());
                }
            }
            // 开始删除
            let mut count = 0;
            let table_width = table.fields.len();
            'outer_delete: for (row_idx, chunk) in table.values.chunks(table_width).enumerate() {
                if !where_clause_idx.is_empty() || !where_clause_number.is_empty() {
                    let mut eq_number = None;
                    for idx in &where_clause_idx {
                        if let Some(number) = eq_number {
                            if chunk[*idx] != number {
                                continue 'outer_delete;
                            }
                        } else {
                            eq_number = Some(chunk[*idx]);
                        }
                    }
                    for n in &where_clause_number {
                        if let Some(number) = eq_number {
                            if *n != number {
                                continue 'outer_delete;
                            }
                        } else {
                            eq_number = Some(*n);
                        }
                    }
                }
                count += 1;
                table.deleted.push(row_idx);
            }
            println!("[>] 成功删除了 {} 条数据。", count);
        }
        Command::Error => {} // 什么也不做
    }
}

use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "bin/database-console.pest"]
struct ConsoleParser;

fn parse_commands(input_str: &str) -> Result<Vec<Command>, pest::error::Error<Rule>> {
    let mut input_pairs = ConsoleParser::parse(Rule::inputs, input_str.trim())?;
    let pair = input_pairs.next();
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
            Rule::insert => commands.push(parse_insert(pair.into_inner())),
            Rule::delete => commands.push(parse_delete(pair.into_inner())),
            Rule::create => commands.push(parse_create(pair.into_inner())),
            Rule::drop => commands.push(parse_drop(pair.into_inner())),
            Rule::describe => commands.push(parse_describe(pair.into_inner())),
            Rule::show => commands.push(Command::ShowTables),
            Rule::EOI => break,
            _ => todo!(),
        }
    }
    // println!("Commands: {:?}", commands);
    Ok(commands)
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
                for columns_pair in select_pair.into_inner() {
                    // println!("[] {:?}", column_pair.as_str());
                    // println!("[] {:?}", column_pair);
                    for column_pair in columns_pair.into_inner() {
                        var_list.push(column_pair.as_str());
                    }
                }
                // 如果inner是“*”，var_list为空
            }
            Rule::table => table_name = select_pair.as_span().as_str(),
            Rule::where_clause => where_clause = Some(parse_where_clause(select_pair.into_inner())),
            _ => unreachable!(),
        }
    }
    Command::Select(var_list, table_name, where_clause)
}

/*
- where_clause > equation
  - left_value: "a"
  - right_value: "1"
*/
fn parse_where_clause<'i>(where_pairs: Pairs<'i, Rule>) -> Where {
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
                _ => unreachable!(),
            }
        }
    }
    Where {
        left: left_value,
        right: right_value,
    }
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
                            _ => unreachable!(),
                        }
                    }
                    name_type_list.push((column, data_type));
                }
            }
            Rule::table => table_name = create_pair.as_span().as_str(),
            _ => unreachable!(),
        }
    }
    Command::Create(table_name, name_type_list)
}

/*
- drop > table: "a"
*/
fn parse_drop<'i>(drop_pairs: Pairs<'i, Rule>) -> Command {
    let mut table_name = "";
    for drop_pair in drop_pairs {
        match drop_pair.as_rule() {
            Rule::table => table_name = drop_pair.as_span().as_str(),
            _ => unreachable!(),
        }
    }
    Command::Drop(table_name)
}

/*
- table: "a"
- insert_content
  - columns
    - column: "b"
    - column: "c"
  - columns
    - column: "d"
    - column: "e"
*/
fn parse_insert<'i>(insert_pairs: Pairs<'i, Rule>) -> Command {
    let mut table_name = "";
    let mut kv_pairs = Vec::new();
    for insert_pair in insert_pairs {
        match insert_pair.as_rule() {
            Rule::insert_content => {
                let mut keys = Vec::new();
                let mut values = Vec::new();
                for (idx, insert_content_pair) in insert_pair.into_inner().enumerate() {
                    // println!("{} => {:?}", idx, insert_content_pair);
                    if idx == 0 {
                        // key
                        for columns_pair in insert_content_pair.into_inner() {
                            keys.push(columns_pair.as_str())
                        }
                    } else {
                        // value
                        for columns_pair in insert_content_pair.into_inner() {
                            values.push(columns_pair.as_str())
                        }
                    }
                }
                if keys.len() != values.len() {
                    println!("[!] 语法无效：键和值的长度不同。");
                    return Command::Error;
                }
                for (i, key) in keys.iter().enumerate() {
                    kv_pairs.push((*key, values[i]))
                }
            }
            Rule::table => table_name = insert_pair.as_span().as_str(),
            _ => unreachable!(),
        }
    }
    // println!("KV pairs: {:?}", kv_pairs);
    Command::Insert(table_name, kv_pairs)
}

/*
- table: "a"
- where_clause > equation
  - left_value: "b"
  - right_value: "c"
*/
fn parse_delete<'i>(delete_pairs: Pairs<'i, Rule>) -> Command {
    let mut table_name = "";
    let mut where_clause = Where {
        left: "",
        right: "",
    };
    for delete_pair in delete_pairs {
        match delete_pair.as_rule() {
            Rule::where_clause => where_clause = parse_where_clause(delete_pair.into_inner()),
            Rule::table => table_name = delete_pair.as_span().as_str(),
            _ => unreachable!(),
        }
    }
    Command::Delete(table_name, where_clause)
}

/*
- describe > table: "a"
*/
fn parse_describe<'i>(describe_pairs: Pairs<'i, Rule>) -> Command {
    let mut table_name = "";
    for describe_pair in describe_pairs {
        match describe_pair.as_rule() {
            Rule::table => table_name = describe_pair.as_span().as_str(),
            _ => unreachable!(),
        }
    }
    Command::Describe(table_name)
}

fn init_database(database: &mut Database) {
    let table_students = Table {
        fields: vec![
            "id".to_string(),
            "score".to_string(),
            "sleep_hours_per_day".to_string(),
        ],
        values: vec![201800001, 80, 9, 201800002, 90, 7, 201800003, 95, 6],
        deleted: Vec::new(),
    };
    database
        .tables
        .insert("students".to_string(), table_students);
    let table_campus_buildings = Table {
        fields: vec![
            "building_id".to_string(),
            "floors".to_string(),
            "since".to_string(),
        ],
        values: vec![10, 5, 1960, 11, 4, 1980, 12, 18, 2010],
        deleted: Vec::new(),
    };
    database
        .tables
        .insert("campus_buildings".to_string(), table_campus_buildings);
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
