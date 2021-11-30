# Rust Myers Diff

[myers](http://xmailserver.org/diff2.pdf) 算法的 rust 实现。

## rust 使用
```rust
// 参考 main.js
// 可以按照自己的需求进行分割，按字符或行
let diff = myers(
	old_str.split("").map(|x|x.to_string()).collect(),
	new_str.split("").map(|x|x.to_string()).collect()
)

diff == [ (Diff, startIndex, endIndex) ]
// Diff
// 		EQ：0：相等
// 		ADD：1：增加
// 		RM：2：删除

// 如果 Diff 为 EQ 或 RM，此索引范围代表其在旧数组中的索引范围。反之则代表新数组中的索引范围
// startIndex
// endIndex
```

## 格式化 diff 文本
```rust
// 判断新增或删除，从相应数组中取出文本进行拼接
fn format_result(old_arr: Vec<String>, new_arr: Vec<String>, diff_result: Vec<DiffResult>) -> String {
    // "-ABC+BAB-BA+C"
    diff_result.iter().fold("".to_string(), |mut r, last| {
        if last.0 == Diff::ADD {
            r.push_str("+");
            for i in last.1..last.2 + 1 {
                let s  = new_arr.get(i as usize).unwrap();
                r.push_str(s);
            }
        } else {
            if last.0 == Diff::RM  {
                r.push_str("-");
            }
            for i in last.1..last.2 + 1 {
                let s  = old_arr.get(i as usize).unwrap();
                r.push_str(s);
            }
        }
        r.push_str("\n");
        r
    })
}
```

## js 使用（wasm）
```js
// 参考 speed/wasm.js
const myers = require('../pkg/rust_myers_diff.js')

// 返回值同上所述
myers.diff({
	old_str: old_str.split(""),
	new_str: new_str.split("")
})
```

## 算法原理

myers 算法常见于 git diff，用来比较两个字符串的差异，寻找「最短的直观的差异」（人类更善于理解的差异）。

最短好理解，最直观是什么意思呢？下面的三种差异，实现的最终结果是一致的。

```js
1.  - A       2.  - A       3.  + C
    - B           + C           - A
      C             B             B
    - A           - C           - C
      B             A             A
    + A             B             B
      B           - B           - B
      A             A             A
    + C           + C           + C
```

但是第一种相比其他两种会更「直观」：
- 删除后新增，比新增后删除「直观」
- 整块的删除，然后新增。比删除新增交叉在一起要「直观」

从论文中可以看出，此算法将旧内容与新内容表示为图。

横轴表示旧内容，纵轴表示新内容。

![](https://gitee.com/lei451927/picture/raw/master/images/20211114204151.png)

算法的目的就是从起点 (0,0) 的位置走到终点 (x, y)  x 表示旧内容的长度，y 表示新内容的长度。

每走一步有三种可能性：
- 向右走，表示删除旧内容
- 向下走，表示新增新内容
- 斜向走，表示内容复用

要生成「最短最直观」的 diff，意味着：
- 尽可能多的复用（斜向多）
- 优先向右走，再向下走（先删除，后新增）

算法的实现，在论文中也已经用伪代码给出

![](https://gitee.com/lei451927/picture/raw/master/images/20211114205007.png)

不难看出，就是使用两层循环走遍所有可能的路径，然后进行回溯查找。

第一层循环代表步数（d）,也就是所走路径的长度，最长是 x + y，即新旧内容的总长度。

第二层表示走的方向（k），每一步都有两种可能性：向右走、向下走（斜走不算长度），所以 k 循环是每次走两步。
k 的取值是 -d 到 d（斜向）

走的路径如下

![](https://gitee.com/lei451927/picture/raw/master/images/20211114204948.png)


看不懂的话看看这张图

![](https://gitee.com/lei451927/picture/raw/master/images/20211113162840.png)

以上基本就是比较难理解的点，详细的可以去看论文与代码实现。

### 参考资料
- [论文](http://xmailserver.org/diff2.pdf)
- [Git 是怎样生成 diff 的：Myers 算法](https://cjting.me/2017/05/13/how-git-generate-diff/)

## WASM

代码用 rust 实现完之后，又将语法改了一遍 js，比较一下的速度差异。

**图片下面看，这里总结结果**：
- rust 每次都稳定在 `~3ms`，平均速度也是 `~3ms`，无疑是最快的
- js 前 4 次运行在 `~10ms`，之后就会被优化至 `~2ms`，平均速度在 `~4.2ms`
- wasm 就比较拉了，第 1 次执行在 `~13ms`，之后在 `~6ms`，平均速度在 `~6.3ms`

可以看到 js 作为解释型语言，前几次执行无疑是吃亏的。
但是等代码变「热」之后，其机器码就会被 v8 极致优化，稳定超过 rust。
（但前提是 js 代码的类型是按照强类型语言的思维编写的，如果类型胡乱变 v8 也无法优化）

而 wasm 就很尴尬，入口是 js 意味着也需要进行解析。
然后 js 输入的值到 rust 需要转换，rust 返回到 js值也需要转换，这样无疑增加了成本。
加上本身就是机器码，v8 无法对其解析优化。

总结来说，对于算法这类代码，wasm 并发挥不出太大的优势，js 本身已经很强大了（没必要为了性能再用 rust 重写后编译 wasm 了）
当然 wasm 的能力绝不于此，所以不能简单通过某个算法就否定 wasm。


--------


> 图中的执行输出：
> 上面的数组表示重复执行 10 次，每次所耗费的毫秒数
> 下面的表示 10 次的平均毫秒数

- 下图左为 原生 js，右边为 rust
![](https://gitee.com/lei451927/picture/raw/master/images/20211114201035.png)

- 下图为 nodejs 调用 wasm
![](https://gitee.com/lei451927/picture/raw/master/images/20211114201921.png)

