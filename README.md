# 飓风内核（开发中）
异步内核就像风一样快！  

## 基于共享调度器的异步内核设计
操作系统内核经历了几个主要的发展阶段，从裸机应用，批处理系统到多道任务系统，演变为至今主流的线程操作系统。这种系统基于线程的切换来调度任务；为了进一步提升性能，一些现代编程语言在应用层复用线程资源，提出了“协程的”的概念，节省任务调度的开销。  
在本项目中我们提出一种新的内核开发思想：由不同资源共享调度器，在操作系统层面提供协程。我们希望这种全新设计的内核在满足传统内核的易用性的同时，拥有着专有内核的高性能特点，“像风一样快”，因此取名**飓风内核**——**tornado-os**。  

## 如何运行
依赖工具：  
+ Rust 环境(nightly-2021-03-01或以上)
+ [Just 工具](https://github.com/casey/just)
+ [qemu-system-riscv64](https://github.com/qemu/qemu)

另外反汇编需要 `riscv64-linux-gnu-objdump`，该工具在 Ubuntu 操作系统上可以通过 `apt-get` 下载。  
调试工具：RISC-V 指令集支持的 [gdb](https://mirrors.tuna.tsinghua.edu.cn/gnu/gdb/?C=M&O=D)  

下载源码：  
```bash
git clone https://github.com/HUST-OS/tornado-os
```

快速运行：  
```bash
cd tornado-os
just qemu user_task
```

## 开发文档
+ [无相之风战队官方网站](https://qf.rs/)
+ 代码注释

## 衍生项目
项目开发过程中，我们经常会有一些想法和思路，在完整的项目中不是很好实现，因此衍生出一些其他的项目：  
+ [洛佳的异步内核实验室](https://github.com/HUST-OS/luojia-os-labs)
+ [洛佳的异步内核实验室第二版](https://github.com/HUST-OS/luojia-os-labs-v2)
+ [异步virtio块设备驱动](https://github.com/HUST-OS/async-virtio-driver)

其中，`洛佳的异步内核实验室`中实现了一个**内核中的生成器语法**，非常有研究价值，欢迎访问博客[执行器与生成语义](https://qf.rs/2021/05/01/%E6%89%A7%E8%A1%8C%E5%99%A8%E4%B8%8E%E7%94%9F%E6%88%90%E8%AF%AD%E4%B9%89.html)  
