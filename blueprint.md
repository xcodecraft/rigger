# 背景：

在服务器的程序管理，涉及到程序的启动、获得正确的配置、程序的升级、程序运行监控等问题。虽然docker解决相关问题，但还存在非docker的环境。

rigger-ng 获取环境配置，管理资源、启动服务上有一些经验。但还是依赖于python ，无法自动处理升级。



通俗一点来说，在服务器端需要一个“应用商店”



# 目标：

- 程序的正确启动: 　读取不同环境配置，正确启动
- 程序授控升级：
- 知识数据更新：
- 程序运行监控:

# 约束

程序大小在10M之内

运行不依赖第三程序和库，保证更好的适用性

运行进占用更小的资源: 10M内存

# 设计：

- Rgnx  :运行在目标机器的 CLI
- RGSvc: 中心服务，管理节点和程序库;





