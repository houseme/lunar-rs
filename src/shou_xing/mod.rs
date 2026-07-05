//! 寿星天文历（ShouXing）—— 高精度天文算法引擎。
//!
//! 基于 VSOP87（太阳黄经）与 ELP（月亮黄经）截断级数、IAU 章动模型、
//! 分段 ΔT 拟合，提供节气（太阳黄经 = k·15°）与合朔（日月黄经差 = 0）
//! 的儒略日求解。所有计算精度到秒级，是农历月大小、闰月、节气时刻的根基。
//!
//! 详见 `engine.rs` 与 `data.rs`。

mod data;
mod engine;

pub use engine::{J2000, calc_qi, calc_shuo, qi_accurate_2};
