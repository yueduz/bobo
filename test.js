import net from 'net';

// 配置要连接的主机和端口
const HOST = '127.0.0.1'; // 目标服务器的 IP 地址
const PORT = 9333;       // 目标服务器的端口号

// 创建一个 TCP 客户端
const client = new net.Socket();
var intervalId 
// 连接到服务器
client.connect(PORT, HOST, () => {
    console.log(`已连接到服务器 ${HOST}:${PORT}`);
    const numberArray = [1, 2, 3, 4, 5, 255]; // 确保数值在 0-255 范围内，或进行适当处理
    
    // 将 number 数组转换为 Uint8Array（如果数值超出 0-255，需要先处理）
    const typedArray = new Uint8Array(numberArray);
    
    // 将 Uint8Array 转换为 Buffer 对象
    const binaryData = Buffer.from(typedArray);
    // client.write(binaryData);
    
    // 使用 setInterval 每秒发送一次 binaryData
     intervalId = setInterval(() => {
        client.write(binaryData);
        console.log(binaryData);
    }, 1000); // 1000 毫秒 = 1 秒
});

// 监听数据接收事件
client.on('data', (data) => {
    console.log('收到服务器数据:', data.toString());
    // 根据需要，可以在这里处理接收到的数据
    // 如果需要，可以在处理完数据后关闭连接
    // client.destroy(); // 关闭连接
});

// 监听连接关闭事件
client.on('close', () => {
    console.log('连接已关闭');
    clearInterval(intervalId);  // 添加这行来停止定时器
});

// 监听错误事件
client.on('error', (err) => {
    console.error('发生错误:', err.message);
});