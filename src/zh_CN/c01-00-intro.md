# 第一章: 开始编写游戏前
欢迎来到 **《使用Rust编写推箱子游戏教程》**! 在开始动手编写游戏前，我们需要先了解下:

## 推箱子是个啥样的游戏嘞?
没玩过推箱子游戏?想当年用诺基亚黑白屏手机的时候就有这款游戏了。你可以下载一个玩一下或者[点这里看下维基百科的介绍](https://en.wikipedia.org/wiki/Sokoban)。本教程就是教大家怎么使用Rust和现有的游戏引擎、素材,编写一个可以玩的推箱子游戏。

## 谁编写了本教程呢? 
本教程是由[@oliviff](https://twitter.com/oliviff) 主笔编写，另外还得到了很多优秀贡献者的支持，感谢您们的辛勤付出（排名不分先后）：

[Blaine](https://github.com/wbprice)

[Ivan](https://github.com/zubivan)

[Hector](https://github.com/rojashr)

[Matt](https://github.com/mysterycommand)

[Guilhermo](https://github.com/GuilhermoReadonly)

[Christian](https://github.com/ChristianIsaacRoy)

## 为什么要使用Rust编写推箱子游戏呢?
我是2019年3月份开始学习Rust的，在编写本教程前我就使用Rust开发过游戏。在学习和使用Rust的过程中我还写了一些博客，感觉从Rust游戏开发中我学到了很多，于是乎我就有个想法：这么好的东西得分享给大家啊，让大家都来体验下啊，独乐乐不如众乐乐！然后就有了本教程。

## 那是不是得先去学习下Rust呢?
不需要。本教程会手把手一步一步教你怎么使用Rust编写游戏，也会对一些Rust的语法进行一些必要的解释。对于一些知识点我们也会提供更详细的介绍链接供您学习参考。当然本教程主要是通过编写一个有趣的游戏顺便对Rust语言进行简单的介绍，所以有些Rust的知识点我们可能不会也没必要过多的深入。

## 文本样式约定
我们使用下面这种样式的文本链接对Rust或者游戏开发等的知识点的扩展信息。
> **_MORE:_**  Read more here.

我们使用下面这种样式的文本链接本章内容相关的程序代码。

> **_CODELINK:_**  You can see the full code in this example here.


## 学习资源
如果在学习过程中你需要寻求帮助或者有问题需要找人帮忙解答，可以看下这些地方：

* [《Rust编程语言》](https://amzn.to/2tXzRdP)
* [《边练边学Rust》](https://doc.rust-lang.org/rust-by-example/)
* [Rust in motion course](https://www.manning.com/livevideo/rust-in-motion?a_aid=cnichols&a_bid=6a993c2e)
* [r/rust](http://reddit.com/r/rust)
* [r/rustgamedev](http://reddit.com/r/rust_gamedev)
* [#rustlang](https://twitter.com/hashtag/rustlang)
* [@rustlang](https://twitter.com/rustlang)
* [@rust_gamedev](https://twitter.com/rust_gamedev)


另外Rust背后还有一群很优秀的开发者组成的社区，所以如果有问题也可以寻求社区帮助。

就先介绍到这里吧，接下来让我们开始编写第一个Rust游戏（准确来说，是我的第二个，但希望这是你们的第一个😉）


_______
Made with 🦀 and 🧡 by [@oliviff](https://twitter.com/oliviff)

