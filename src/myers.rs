use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// myers 算法对比新旧文本 http://xmailserver.org/diff2.pdf
pub fn myers(x_str: Vec<String>, y_str: Vec<String>) -> DiffResult {
    let d_kxmap = diff(&x_str, &y_str);
    let snakes = gen_snakes(d_kxmap, x_str.len() as isize, y_str.len() as isize);
    resolve_result(snakes, x_str, y_str)
}

// diff 寻找路径
fn diff(x_str: &Vec<String>, y_str: &Vec<String>) -> Vec<HashMap<isize, isize>> {
    // x 文本的长度
    let x_max = x_str.len() as isize;
    // y 文本的长度
    let y_max = y_str.len() as isize;
    // 图的总长度
    let g_max = x_max + y_max;

    // 存储 每一步 k，和对应位置的 x
    let mut kx_map: HashMap<isize, isize> = HashMap::new();
    // 存储 每一步 d, 对应的 kx_map
    let mut d_kxmap = Vec::with_capacity(g_max as usize);

    // x 轴表示旧文本
    // y 轴表示新文本
    // 计算 从 0,0 走到 x,y 的最短、最直观路径
    // 每走一步，有两种可能：向右走表示删除，向下走表示新增
    let mut d = 0;

    while d <= g_max {
        let mut k = -d;
        d_kxmap.push(HashMap::new());

        while k <= d {
            // 当前点的 x
            // 要么就是上一步向右走过来的，x + 1
            // 要么就是上一步向下走过来的，x 不变
            let mut x;

            // 左边的 x (k - 1)
            let left_x = kx_map.get(&(k - 1)).map_or(-1, |x| *x);
            // 上边的 x（k + 1）
            let up_x = kx_map.get(&(k + 1)).map_or(-1, |x| *x);

            if k == -d {
                // 这意味着向下直走，全部新增，x 和上一步一样
                x = up_x;
            } else if k == d {
                // 这意味着向右直走，全部删除，x 需要基于上一步前进一位
                x = left_x + 1;
            } else {
                // 其他情况下
                // 因为存在相同节点的斜向走法，所以并不确定此时谁走的更远（x 更大，x 大意味着离终点近）
                // 所以需要比对两点的 x 值，选取值大的那条路
                // 同时需要注意，如果左边的 x 大，说明是向右走过来的，x 需要 + 1
                x = if left_x < up_x { up_x } else { left_x + 1 };
            }

            // 拿到 x 就可以计算出 y 了
            let mut y = x - k;

            // 相同文本情况 走斜向前进
            while y < y_max && x < x_max && eq(x_str, y_str, x, y) {
                x = x + 1;
                y = y + 1;
            }

            // 记录
            kx_map.insert(k, x);
            d_kxmap.last_mut().unwrap().insert(k, x);

            if x >= x_max && y >= y_max {
                return d_kxmap;
            }

            k = k + 2;
        }
        d = d + 1;
    }
    d_kxmap
}

// (x, y, 向左更优)
type Snakes = Vec<(isize, isize, Option<bool>)>;
// 生成最优路径
fn gen_snakes(d_kxmap: Vec<HashMap<isize, isize>>, x_max: isize, y_max: isize) -> Snakes {
    let mut snakes: Snakes = Vec::with_capacity(d_kxmap.len());

    // 从终点回溯
    let mut x = x_max;
    let mut y = y_max;

    // 终点先放进去
    snakes.push((x, y, None));

    // 倒退每一步 d
    let mut d = d_kxmap.len() - 1;

    while d > 0 {
        // 计算 k
        let k = x - y;

        let kx_map = d_kxmap.get(d - 1).unwrap();
        // 在上一个点向下走过来（k + 1）
        let up_x = kx_map.get(&(k + 1)).map_or(-1, |x| *x);
        // 在上一个点向右走过来 (k - 1)
        let left_x = kx_map.get(&(k - 1)).map_or(-1, |x| *x);

        // 两个都没有，说明已经到头了
        if left_x == -1 && up_x == -1 {
            return snakes;
        }

        // 计算两者的y坐标
        let up_y = up_x - k - 1;
        let left_y = left_x - k + 1;

        // 计算当前 x 坐标和上一个 x 坐标的差值
        let up_x_gap = x - up_x;
        let left_x_gap = x - left_x;

        // 说明左边更近
        let mut is_left_best = left_x_gap < up_x_gap;

        // 如果两个都有值, 就需要分别比对两者哪个更优，哪个点离当前更近
        // 如果 x 一样，需要比对 y
        if left_x_gap == up_x_gap {
            // 计算 y 轴差值
            let up_y_gap = up_y - y;
            let left_y_gap = left_y - y;

            // 如果左边的 y 差值更小，说明也是左边更好
            is_left_best = left_y_gap.abs() < up_y_gap.abs()
        }

        // 上一步最优的坐标点
        let pre_x = if is_left_best { left_x } else { up_x };
        let pre_y = if is_left_best { left_y } else { up_y };

        // 拿到上一个点的坐标，压栈
        snakes.push((pre_x, pre_y, Some(is_left_best)));
        x = pre_x;
        y = pre_y;

        d = d - 1;
    }
    snakes
}


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Diff {
    EQ,
    ADD,
    RM,
}

// Diff，内容，内容的开始索引，结束索引（连续的相同类型会被合并，这样字符串就丢失了索引信息）
pub type DiffResult = Vec<(Diff, String, usize, usize)>;

// 解析返回差异结果
fn resolve_result(mut snakes: Snakes, x_str: Vec<String>, y_str: Vec<String>) -> DiffResult {
    let mut result: DiffResult = vec![];

    // x 文本的长度
    let x_max = x_str.len() as isize;
    // y 文本的长度
    let y_max = y_str.len() as isize;

    let push = |result: &mut DiffResult, action: Diff, v: &str| {
        if let Some(last) = result.last_mut() {
            if last.0 == action {
                last.1 = format!("{}{}", last.1, v);
								last.3 = last.3 + 1;
                return;
            }
        }
				let i = result.len();
        result.push((action, v.to_string(), i, i));
    };

    // 相同字符处理
    let advance = |result: &mut DiffResult, mut x, mut y| {
        while x < x_max && y < y_max && eq(&x_str, &y_str, x, y) {
            push(result, Diff::EQ, x_str.get(x as usize).unwrap());
            x = x + 1;
            y = y + 1;
        }
    };

    // 处理头部相同的情况
    advance(&mut result, 0, 0);

    while !snakes.is_empty() {
        let (mut x, mut y, is_left_best) = snakes.pop().unwrap();
        //is_left_best 为空代表是最后一个点了
        if is_left_best == None {
            break;
        }

        let is_left_best = is_left_best.unwrap();

        if is_left_best {
            // 意味着向右走，删除
            push(&mut result, Diff::RM, x_str.get(x as usize).unwrap());
            x = x + 1;
            advance(&mut result, x, y);
        } else {
            // 新增
            push(&mut result, Diff::ADD, y_str.get(y as usize).unwrap());
            y = y + 1;
            advance(&mut result, x, y);
        }
    }

    result
}

// 判断是否相等
fn eq(x_str: &Vec<String>, y_str: &Vec<String>, x: isize, y: isize) -> bool {
    let x = x_str.get(x as usize).map_or("", |x| x);
    let y = y_str.get(y as usize).map_or("", |y| y);
    x == y
}

#[cfg(test)]
mod test {
    use crate::{myers};

    use super::{Diff, DiffResult};

    fn format_result(res: DiffResult) -> String {
				println!("{:#?}", res);
        res.iter().fold("".to_string(), |mut r, (action, v, ..)| {
            if *action == Diff::ADD {
                r.push_str("+");
            } else if *action == Diff::RM {
                r.push_str("-");
            }
            r.push_str(v);
            r.push_str("\n");
            r
        })
    }

    #[test]
    fn diff() {
        let res = myers(
            "ABCABBA".split("").filter(|x| !x.is_empty()).map(|x|x.to_string()).collect(),
            "CBABAC".split("").filter(|x| !x.is_empty()).map(|x|x.to_string()).collect(),
        );

        assert_eq!("-AB\nC\n+B\nAB\n-B\nA\n+C\n", format_result(res));
    }

    #[test]
    fn eq() {
        let res = myers(
            "ABCABBA".split("").filter(|x| !x.is_empty()).map(|x|x.to_string()).collect(),
            "ABCABBA".split("").filter(|x| !x.is_empty()).map(|x|x.to_string()).collect(),
        );

        assert_eq!("ABCABBA\n", format_result(res));
    }
}
