use futures::{
    future::FutureExt, // 为了 `.fuse()`
    pin_mut,
    select,
};
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tauri::ipc::Channel;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Notify;

pub async fn start_server(
    on_event: Channel<Vec<u8>>,
    length: usize,
    notify: Arc<Notify>,
    address: &str, // 新增地址参数
) -> io::Result<()> {
    let counter = Arc::new(AtomicUsize::new(0));
    let listener = TcpListener::bind(address).await?;
    loop {
        select! {
            // 等待新的TCP连接
            result = listener.accept().fuse() => {
                match result {
                    Ok((mut socket, addr)) => {
                        println!("new client: {:?}", addr);
                        counter.fetch_add(1, Ordering::SeqCst);
                        if counter.load(Ordering::Relaxed) > 1 {
                            // 关闭连接
                            println!("Connection closed due to cont > 1");
                            let modified_data = format!("重复连接");

                            socket.write_all(modified_data.as_bytes()).await?;
                            socket.shutdown().await?;

                            counter.fetch_sub(1, Ordering::SeqCst);
                            continue;
                        }
                        let counter_clone = Arc::clone(&counter);
                        let on_event_clone = on_event.clone();
                        let notify_clone = notify.clone(); // Clone the notify before using it in the loo
                        tokio::spawn(async move {
                            let mut buf = vec![0; length];
                            loop {
                                let read_fut = socket.read(&mut buf).fuse();
                                let notify_fut = notify_clone.notified().fuse();
                                pin_mut!(read_fut, notify_fut);
                                select! {
                                    result = read_fut => {
                                        match result {
                                                    Ok(n) if n > 0 => {
                                            let data = String::from_utf8_lossy(&buf[..n]);

                                            // 关键修改：发送数据的拷贝而不是引用
                                            let data_to_send = buf[..n].to_vec();
                                            on_event_clone.send(data_to_send).unwrap();
                                            // println!("received data: {}", data);

                                            // // 修改数据，例如添加前缀
                                            // let modified_data = format!("Modified: {}", data);

                                            // // 发送修改后的数据回客户端
                                            // if let Err(e) = socket.write_all(modified_data.as_bytes()).await {
                                            //     eprintln!("failed to send data: {:?}", e);
                                            //     counter_clone.fetch_sub(1, Ordering::SeqCst);
                                            //     break; // 发送失败时退出循环
                                            // }
                                        }
                                        Ok(_) => {
                                            eprintln!("client disconnected");
                                            counter_clone.fetch_sub(1, Ordering::SeqCst);
                                            break; // 客户端断开连接时退出循环
                                        }
                                        Err(e) => {
                                            eprintln!("failed to read from socket: {:?}", e);
                                            counter_clone.fetch_sub(1, Ordering::SeqCst);
                                            break; // 读取失败时退出循环
                                        }}

                                    }
                                    _ = notify_fut => {
                                        socket.shutdown().await.unwrap();
                                        println!("Received stop notification");
                                         // 关闭监听器
                                         break;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("failed to accept connection: {:?}", e);
                        // 根据需要处理错误，可能退出循环
                    }
                }
            }
            // 等待通知信号
            _ = notify.notified().fuse() => {
                println!("Received notification, performing some action...");
                break;
            }
        }
    }

    println!("退出start_server函数\n");

    return Ok(());
}
