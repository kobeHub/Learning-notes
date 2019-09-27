# Prometheus -- 不灭之火

![prome](https://i.imgur.com/ZbuHWq1.jpg)

最近一直在学习k8s以及prometheus，不得不佩服这两个开源项目的伟大。一个是引领航行的舵手，另一个则是照亮远方的不灭之火， 单从名字上讲，它们两个就已经占据了“哲学”意味上的不凡特性。一直在接受新知识，没有进行系统的总结，所以是时候写点东西了。

> 偷偷吐槽一下：k8s玩久了，写点代码或者文档竟然异常舒服。。

## 1. What is Prometheus?

[Prometheus]([https://prometheus.io](https://prometheus.io/)) 作为 [CNCF]([https://www.cncf.io](https://www.cncf.io/)) 第二个毕业项目，逐渐成为 k8s 生态不可或缺的一部分。该项目最早受启发于 Google 的 Brogmon 监控系统（相似的是k8s由Google的Brog系统演变而来），从2012年的开始由 Google 工程师在SoundCloud 以开源软件的形式进行开发，并与2015年对外发布早期版本。2016年5月继 k8s 之后第二个正式加入 CNCF 基金会。大事年表如下：

![time](https://blobscdn.gitbook.com/v0/b/gitbook-28427.appspot.com/o/assets%2F-LBdoxo9EmQ0bJP2BuUi%2F-LPMFlGDFIX7wuLhSHx9%2F-LPMFo9ZTdKYHyFzu4DJ%2Fprometheus-release-roadmaps.png?generation=1540136064641479&alt=media)

----

## 2. 监控的目标‘

在《SRE: Google 运维解密》一书中指出，监控系统需要有效的支持白盒监控以及黑盒监控。通过白盒可以了解内部运行的实际状态，通过对监控指标的观察可以预测可能出现的问题，从而对潜在的不确定因素进行优化。而黑盒监控，常见的如HTTP探针、TCP探针，可以在系统或者服务发生故障时快速的通知相关人员进行处理。监控的整体目标大致为:

+ **长期趋势分析**：通过对监控指标的长期收集以及统计，对监控指标进行长期趋势分析。例如：对于磁盘空间增长率的判断，可以预测在什么时间节点对资源进行扩容
+ **对照分析**：两个版本的系统运行资源使用情况的差异如何？在不同容量情况下系统的并发和负载变化如何？通过监控能够方便的对系统进行跟踪和比较。
+ **告警**：当系统出现或者即将出现故障时，监控系统需要迅速反应并通知管理员，从而能够对问题进行快速的处理或者提前预防问题的发生，避免出现对业务的影响。
+ **故障分析与定位**：当问题发生后，需要对问题进行调查和处理。通过对不同监控监控以及历史数据的分析，能够找到并解决根源问题。数据可视化：通过可视化仪表盘能够直接获取系统的运行状态、资源使用情况、以及服务运行状态等直观的信息。

## 3. 与其他监控系统的对比

一些常见的监控系统，往往不能有效解决以上问题。例如：Nagios、Zabbix。以Nagios为例，其基本架构如下：

![Nagios Arch](https://blobscdn.gitbook.com/v0/b/gitbook-28427.appspot.com/o/assets%2F-LBdoxo9EmQ0bJP2BuUi%2F-LVM6abLYacMuIwyN9mE%2F-LPMFo9aJ6b5g11oQVhO%2Fnagios-platform.png?generation=1546576101127624&alt=media)

Nagios 主要用于监控服务以及主机，Nagios软件需要安装在一个独立的服务器运行，该服务器被称为监控中心。每一台被监控的主机或者服务都需要一个可以与Nagios监控中心通信的后台程序，作为Agent向监控中心发送metrics。

首先对于Nagios而言，大部分监控能力都是围绕系统的一些**边缘性**问题，主要围绕系统服务、资源的状态以及应用程序的可用性。Nagios可以通过check_disk插件检查磁盘、check_load检查cpu负载。核心采用了测试和告警（Check&Alert）的监控模型，往往存在以下问题；

+ **与业务脱离**的监控：监控系统获取到的监控指标与业务本身也是一种分离的关系。好比客户可能关注的是服务的可用性、服务的SLA等级，而监控系统却只能根据系统负载去产生告警；
+ 运维管理难度大：Nagios这一类监控系统本身运维管理难度就比较大，需要有专业的人员进行安装，配置和管理，而且过程并不简单；
+ 可扩展性低： 监控系统自身难以扩展，以适应监控规模的变化；
+ 问题定位难度大：当问题产生之后（比如主机负载异常增加）对于用户而言，他们看到的*依然是一个黑盒*，他们无法了解主机上服务真正的运行情况，因此当故障发生后，这些告警信息并不能有效的支持用户对于故障根源问题的分析和定位。

----

## 3. 整体架构

### 3.1 基本组件

Prometheus 系统包含多个组件，其中一些是可选的：

+ **Prometheus Server**：用于获取时序数据（scrape），并且存储在时序数据库中(TSDB)
+ **Client Library**: 客户端库，为需要监控的服务生成相应的 metrics 并暴露给 Prometheus server。当 Prometheus server 来 pull 时，直接返回实时状态的 metrics。
+ **Push Gateway**: 主要用于短期的 jobs。由于这类 jobs 存在时间较短，可能在 Prometheus 来 pull 之前就消失了。为此，这次 jobs 可以直接向 Prometheus server 端推送它们的 metrics。这种方式主要用于服务层面的 metrics，对于机器层面的 metrices，需要使用 node exporter。
+ **Exporters**: 用于暴露已有的第三方服务的 metrics 给 Prometheus。
+ **Alertmanager**: 从 Prometheus server 端接收到 alerts 后，会进行去除重复数据，分组，并路由到对收的接受方式，发出报警。常见的接收方式有：电子邮件，pagerduty，OpsGenie, webhook 等。

### 3.2 架构

![arch](https://prometheus.io/assets/architecture.png)

从上图可以看出Prometheus的主要模块： Prometheus server、Job exporter、Pushgateway、Alertmanager、Web UI，大致的工作流程如下:

+ Prometheus server 定期从配置好的 jobs 或者 exporters 中拉 metrics，或者接收来自 Pushgateway 发过来的 metrics，或者从其他的 Prometheus server 中拉 metrics。
+ Prometheus server 在本地存储收集到的 metrics，并运行已定义好的 alert.rules，记录新的时间序列或者向 Alertmanager 推送警报。
+ Alertmanager 根据配置文件，对接收到的警报进行处理，发出告警。
+ 使用 Grafana 等进行可视化

## 4. Prometheus 特性以及优势

prometheus相比于其他监控系统，有以下特性：

- 由metric name以及键值对确定的时序数据（time series）,提供多维度的数据模型
- 拥有灵活的查询语言PromQL
- 不依赖于分布式存储，可以提供单点服务
- 通过HTPP请求，基于pull监控模型获取时序数据
- 可以借助外部网关push时序数据
- 可以通过服务发现或者静态配置发现监控Targets
- 支持图形以及dashboard多种展现形式

Prometheus是一个开源的完整监控解决方案，其对传统监控系统的测试和告警模型进行了彻底的颠覆，形成了基于中央化的规则计算、统一分析和告警的新模型。 相比于传统监控系统Prometheus具有以下优点：

### 4.1 易于管理 

Prometheus核心部分只有一个单独的二进制文件，不存在任何的第三方依赖(数据库，缓存等等)。唯一需要的就是本地磁盘，因此不会有潜在级联故障的风险。Prometheus基于Pull模型的架构方式，可以在任何地方（本地电脑，开发环境，测试环境）搭建我们的监控系统。对于一些复杂的情况，还可以使用Prometheus服务发现(Service Discovery)的能力动态管理监控目标。监控服务的内部运行状态Pometheus鼓励用户监控服务的内部状态，基于Prometheus丰富的Client库，用户可以轻松的在应用程序中添加对Prometheus的支持，从而让用户可以获取服务和应用内部真正的运行状态。

### 4.2 监控服务内部运行状态

Prometheus 鼓励用户监控服务的内部状态，基于Prometheus丰富的Client库，用户可以轻松的在应用程序中添加对Prometheus的支持，从而让用户可以获取服务和应用内部真正的运行状态。

![compare](https://blobscdn.gitbook.com/v0/b/gitbook-28427.appspot.com/o/assets%2F-LBdoxo9EmQ0bJP2BuUi%2F-LVM6abLYacMuIwyN9mE%2F-LVM6by5fAZL01wxgF9M%2Fmonitor-internal.png?generation=1546576102101850&alt=media)

### 4.3 强大的数据模型

所有采集的监控数据均以指标(metric)的形式保存在内置的时间序列数据库当中(TSDB)。所有的样本除了基本的指标名称以外，还包含一组用于描述该样本特征的标签。如下所示：

```c++
http_request_status{code='200',content_path='/api/path', environment='produment'} => [value1@timestamp1,value2@timestamp2...]
http_request_status{code='200',content_path='/api/path2', environment='produment'} => [value1@timestamp1,value2@timestamp2...]
```

每一条时间序列由指标名称(Metrics Name)以及一组标签(Labels)唯一标识。每条时间序列按照时间的先后顺序存储一系列的样本值。

表示维度的标签可能来源于你的监控对象的状态，比如code=404或者content_path=/api/path。也可能来源于的你的环境定义，比如environment=produment。基于这些Labels我们可以方便地对监控数据进行聚合，过滤，裁剪。

### 4.4 强大的查询语言PromQL

Prometheus内置了一个强大的数据查询语言PromQL。 通过PromQL可以实现对监控数据的查询、聚合。同时PromQL也被应用于数据可视化(如Grafana)以及告警当中。通过PromQL可以轻松回答类似于以下问题：

+ 在过去一段时间中95%应用延迟时间的分布范围？

+ 预测在4小时后，磁盘空间占用大致会是什么情况？

+ CPU占用率前5位的服务有哪些？(过滤)

### 4.5 高效

对于监控系统而言，大量的监控任务必然导致有大量的数据产生。而Prometheus可以高效地处理这些数据，对于单一Prometheus Server实例而言它可以处理：

+ 数以百万的监控指标
+ 每秒处理数十万的数据点。

### 4.6 可扩展

Prometheus是如此简单，因此你可以在每个数据中心、每个团队运行独立的Prometheus Sevrer。Prometheus对于联邦集群的支持，可以让多个Prometheus实例产生一个逻辑集群，当单实例Prometheus Server处理的任务量过大时，通过使用功能分区(sharding)+联邦集群(federation)可以对其进行扩展。

### 4.7 易于集成

使用Prometheus可以快速搭建监控服务，并且可以非常方便地在应用程序中进行集成。目前支持： Java， JMX， Python， Go，Ruby， .Net， Node.js等等语言的客户端SDK，基于这些SDK可以快速让应用程序纳入到Prometheus的监控当中，或者开发自己的监控数据收集程序。同时这些客户端收集的监控数据，不仅仅支持Prometheus，还能支持Graphite这些其他的监控工具。

同时Prometheus还支持与其他的监控系统进行集成：Graphite， Statsd， Collected， Scollector， muini， Nagios等。

Prometheus社区还提供了大量第三方实现的监控数据采集支持：JMX， CloudWatch， EC2， MySQL， PostgresSQL， Haskell， Bash， SNMP， Consul， Haproxy， Mesos， Bind， CouchDB， Django， Memcached， RabbitMQ， Redis， RethinkDB， Rsyslog等等。

### 4.8 可视化

Prometheus Server中自带了一个Prometheus UI，通过这个UI可以方便地直接对数据进行查询，并且支持直接以图形化的形式展示数据。同时Prometheus还提供了一个独立的基于Ruby On Rails的Dashboard解决方案Promdash。最新的Grafana可视化工具也已经提供了完整的Prometheus支持，基于Grafana可以创建更加精美的监控图标。基于Prometheus提供的API还可以实现自己的监控可视化UI。

### 4.9 开放性

通常来说当我们需要监控一个应用程序时，一般需要该应用程序提供对相应监控系统协议的支持。因此应用程序会与所选择的监控系统进行绑定。为了减少这种绑定所带来的限制。对于决策者而言要么你就直接在应用中集成该监控系统的支持，要么就在外部创建单独的服务来适配不同的监控系统。而对于Prometheus来说，使用Prometheus的client library的输出格式不止支持Prometheus的格式化数据，也可以输出支持其它监控系统的格式化数据，比如Graphite。因此你甚至可以在不使用Prometheus的情况下，采用Prometheus的client library来让你的应用程序支持监控数据采集。