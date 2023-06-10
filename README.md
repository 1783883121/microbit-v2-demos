# microbit-v2-demos
about microbit-v2 demos

File: ./a-template-demo 是一个独立的基础模板Demo用来演示Rust中如何使用microbit v2
Q：如何运行他？
A：
1.安装所需环境，详见[环境搭建]:https://jzow.github.io/discovery/microbit/03-setup/index.html
2.进入到该目录执行`cargo run`命令，他会进行编译构建 并打开GDB（各平台使用的GDB不同），关于GDB的安装 详见[环境搭建]:https://jzow.github.io/discovery/microbit/03-setup/windows.html
3.新建命令行进入到该目录执行`cargo embed` 命令，他会打开一个端口号监听 
4.在运行“cargo run命令的命令行”中 回车 输入:`target remote :1337` 回车 再输入:`continue`，回车即可在microbit:v2单片机中看到有一个灯亮着。可以修改`col2和row3`就可以实现指定位置的灯亮，如：设置为`col5和row1`那么就会看到第一行的第五个灯亮（修改代码后需重新编译，重新执行`cargo embed`，gdb中执行`target remote :1337`，`continue`）