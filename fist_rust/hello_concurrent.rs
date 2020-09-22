use std::thread;
use std::time::Duration;
use std::sync::mpsc;
fn spawn_function() {
    for i in 0..5 {
        println!("spawned thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    // spawn产生,产生一个新线程；
    //随着主线程的结束，spawn 线程也随之结束了，并没有完成所有打印。
    // thread::spawn(spawn_function);

    // for i in 0..3 {
    //     println!("main thread print {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // std::thread::spawn 函数的参数是一个无参函数，但上述写法不是推荐的写法，
    // 我们可以使用闭包（closures）来传递函数作为参数：

    thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    /*
    闭包是可以保存进变量或作为参数传递给其他函数的匿名函数。闭包相当于 Rust 中的 Lambda 表达式，格式如下：
    |参数1, 参数2, ...| -> 返回值类型 {
        // 函数体
    }
    */

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let inc = |num: i32| -> i32 {
        num + 1
    };
    println!("inc(5) = {}", inc(5));
    // 闭包可以省略类型声明使用 Rust 自动类型判断机制：
    let inc = |num| {
        num + 1
    };
    println!("inc(5) = {}", inc(5));


    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // join 方法可以使子线程运行结束后再停止运行程序。
    handle.join().unwrap();

    // 在子线程中尝试使用当前函数的资源，这一定是错误的！因为所有权机制禁止这种危险情况的产生，
    // 它将破坏所有权机制销毁资源的一定性。我们可以使用闭包的 move 关键字来处理：
    /*
    use std::thread;

    fn main() {
        let s = "hello";
    
        let handle = thread::spawn(move || {
            println!("{}", s);
        });

        handle.join().unwrap();
    }
    */
    // 消息传递
    // Rust 中一个实现消息传递并发的主要工具是通道（channel），
    // 通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）。

    // std::sync::mpsc 包含了消息传递的方法：
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

}