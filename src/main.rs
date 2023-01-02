use std::sync::{ Arc, Mutex, mpsc::channel };
use std::thread;

use std::time::Duration;

fn main() {
    
    // test_move();
    test_channel();
    // test_receive1();
    test_mutex();
    test_mutex1();
}

pub fn test_mutex1() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // 错误信息表明 counter 值在上一次循环中被移动 不能将 counter 锁的所有权移动到多个线程中
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}

pub fn test_mutex() {
    let x = Mutex::new(6);
    
    {
        // 这个调用会阻塞当前线程，直到我们拥有锁为止
        let mut num = x.lock().unwrap();
        *num = 10;
    }
    println!("x = {:?}", x);
}

pub fn test_receive1() {
    let (tx, rx) = channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let strings = vec![
            String::from("1:hi"),
            String::from("1:hello"),
            String::from("1:good"),
            String::from("1:china"),
        ];

        for item in strings {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    thread::spawn(move || {
        let strings = vec![
            String::from("hi"),
            String::from("hello"),
            String::from("good"),
            String::from("china"),
        ];

        for item in strings {
            tx1.send(item).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for result in rx {
        println!("{:?}", result);
    }
}

pub fn test_receive() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let strings = vec![
            String::from("hi"),
            String::from("hello"),
            String::from("good"),
            String::from("china"),
        ];

        for item in strings {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 在主线程中，不再显式调用 recv 函数：而是将 rx 当作一个迭代器
    for result in rx {
        println!("{:?}", result);
    }
}

pub fn test_channel() {
    let (sender, receiver) = channel();

    thread::spawn(move || {
        let message = String::from("Hello");
        sender.send(message).unwrap();
        // borrow of moved value: `message` value borrowed here after move
        // println!("{:?}", message);
    });

    // recv: 这个方法会阻塞主线程执行直到从信道中接收一个值
    // try_recv 不会阻塞，立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息
    let result = receiver.recv().unwrap();
    println!("{:?}", result);
}

pub fn test_move() {
    let vec = vec![1, 2, 3, 4, 5];
    // move 强制闭包获取其使用的值的所有权
    let hadnle = thread::spawn(move || { 
        println!("{:?}", vec);
    });
    hadnle.join().unwrap();
}

pub fn test_spawn() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。
    // 阻塞（Blocking）线程意味着阻止该线程执行工作或退出
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
