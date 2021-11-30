/* tslint:disable */
/* eslint-disable */
/**
* @param {ParamsType} params
* @returns {DiffResult}
*/
export function diff(params: ParamsType): DiffResult;

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


