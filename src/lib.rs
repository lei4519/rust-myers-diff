use wasm_bindgen::prelude::*;

use myers::{myers, DiffResult};

pub mod myers;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_CUSTOM_SECTION: &'static str = r#"
export interface ParamsType {
    /** 旧文本的字符串数组，可以按照需求进行切割（按字符、按行、按段） */
    old_arr: string[];
    /** 新文本的字符串数组 */
    new_arr: string[];
}

/**
 * 0: diff 动作 - 相等、新增、删除
 *
 * 1: 在数组中的开始索引 (新增时为新文本数组索引，反之为旧文本数组索引)
 *
 * 2: 在数组中的结束索引
 */
export type DiffResult = Array<["EQ" | "ADD" | "RM", number, number]>
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ParamsType")]
    pub type ParamsType;
    #[wasm_bindgen(typescript_type = "DiffResult")]
    pub type DiffResultType;
}

pub struct Params<'a> {
    pub old_arr: &'a Vec<&'a str>,
    pub new_arr: &'a Vec<&'a str>,
}

// #[wasm_bindgen]
pub fn diff(params: Params) -> Vec<DiffResult> {
    myers(params.old_arr, params.new_arr)
}
