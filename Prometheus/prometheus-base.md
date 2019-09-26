# Prometheus -- 不灭之火

![prome](https://i.imgur.com/ZbuHWq1.jpg)

最近一直在学习k8s以及prometheus，不得不佩服这两个开源项目的伟大。一个是引领航行的舵手，另一个则是照亮远方的不灭之火， 单从名字上讲，它们两个就已经占据了“哲学”意味上的不凡特性。一直在接受新知识，没有进行系统的总结，所以是时候写点东西了。

> 偷偷吐槽一下：k8s玩久了，写点代码或者文档竟然异常舒服。。

## 1. What is Prometheus?

[Prometheus]([https://prometheus.io](https://prometheus.io/)) 作为 [CNCF]([https://www.cncf.io](https://www.cncf.io/)) 第二个毕业项目，逐渐成为 k8s 生态不可或缺的一部分。该项目最早受启发于 Google 的 Brogmon 监控系统（相似的是k8s由Google的Brog系统演变而来），从2012年的开始由 Google 工程师在SoundCloud 以开源软件的形式进行开发，并与2015年对外发布早期版本。2016年5月继 k8s 之后第二个正式加入 CNCF 基金会。大事年表如下：

![time](https://blobscdn.gitbook.com/v0/b/gitbook-28427.appspot.com/o/assets%2F-LBdoxo9EmQ0bJP2BuUi%2F-LPMFlGDFIX7wuLhSHx9%2F-LPMFo9ZTdKYHyFzu4DJ%2Fprometheus-release-roadmaps.png?generation=1540136064641479&alt=media)



## 2. 整体架构

![arch](https://prometheus.io/assets/architecture.png)

