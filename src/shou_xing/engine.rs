#![allow(
    clippy::approx_constant,
    clippy::unreadable_literal,
    clippy::suboptimal_flops,
    clippy::inconsistent_digit_grouping,
    clippy::excessive_precision,
    clippy::pedantic,
    clippy::nursery,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::float_cmp
)]
//! 寿星天文历算法引擎。
//!
//! 这是整个库的精度根基。所有函数与 lunar-go `ShouXingUtil.go` 逐行对齐，
//! 浮点运算顺序、取整方向、取模语义均保持一致，以保证节气 / 合朔儒略日
//! 与参考实现数值完全相同（差 1 秒即可能跨分钟）。

use std::sync::LazyLock;

use super::data::{DT_AT, NUT_B, QI_KB, SHUO_KB, XL0, XL1_0, XL1_1, XL1_2, XL1_3};

const PI: f64 = std::f64::consts::PI;
const PI_2: f64 = 2.0 * PI;
const ONE_THIRD: f64 = 1.0 / 3.0;
const SECOND_PER_DAY: f64 = 86400.0;
const SECOND_PER_RAD: f64 = 180.0 * 3600.0 / PI;

/// J2000.0 历元的儒略日（2451545.0）。
pub const J2000: f64 = 2_451_545.0;

const XL1_ROWS: &[&[f64; 2652]; 4] = &[XL1_0, XL1_1, XL1_2, XL1_3];
const XL1_OBL: usize = 2652;

/// 把寿星天文历特有的紧缩字母串展开为 `0/1/2` 位串。
///
/// 对应 Go 的 `decode(s string) string`：每个字母展开为不同长度的 `0`（与单个 `1`/`2`）序列，
/// 用于节气 / 合朔在低精度区间的 ±1 天修正。
fn decode(s: &str) -> String {
    let o = "0000000000";
    let o2 = format!("{o}{o}");
    let mut r = String::from(s);
    // 顺序与 Go 完全一致（先长后短）。
    let pairs: &[(&str, &str)] = &[
        ("J", "00"),
        ("I", "000"),
        ("H", "0000"),
        ("G", "00000"),
        ("t", "02"),
        ("s", "002"),
        ("r", "0002"),
        ("q", "00002"),
        ("p", "000002"),
        ("o", "0000002"),
        ("n", "00000002"),
        ("m", "000000002"),
        ("l", "0000000002"),
        ("k", "01"),
        ("j", "0101"),
        ("i", "001"),
        ("h", "001001"),
        ("g", "0001"),
        ("f", "00001"),
        ("e", "000001"),
        ("d", "0000001"),
        ("c", "00000001"),
        ("b", "000000001"),
        ("a", "0000000001"),
        ("A", &format!("{o2}{o2}{o2}")),
        ("B", &format!("{o2}{o2}{o}")),
        ("C", &format!("{o2}{o2}")),
        ("D", &format!("{o2}{o}")),
        ("E", &o2),
        ("F", o),
    ];
    // 注意：A..F 的展开串在自身替换结果里不含字母，可安全先做；这里严格按 Go 顺序逐一替换。
    for (from, to) in pairs {
        if let Some(idx) = from.chars().next() {
            // 仅对单字符做全量替换（与 strings.Replace(s, x, y, -1) 等价）
            if from.len() == 1 {
                r = r.replace(idx, to);
            } else {
                r = r.replace(from, to);
            }
        }
    }
    r
}

static QB: LazyLock<String> = LazyLock::new(|| {
    decode(
        "FrcFs22AFsckF2tsDtFqEtF1posFdFgiFseFtmelpsEfhkF2anmelpFlF1ikrotcnEqEq2FfqmcDsrFor22FgFrcgDscFs22FgEeFtE2sfFs22sCoEsaF2tsD1FpeE2eFsssEciFsFnmelpFcFhkF2tcnEqEpFgkrotcnEqrEtFermcDsrE222FgBmcmr22DaEfnaF222sD1FpeForeF2tssEfiFpEoeFssD1iFstEqFppDgFstcnEqEpFg11FscnEqrAoAF2ClAEsDmDtCtBaDlAFbAEpAAAAAD2FgBiBqoBbnBaBoAAAAAAAEgDqAdBqAFrBaBoACdAAf1AACgAAAeBbCamDgEifAE2AABa1C1BgFdiAAACoCeE1ADiEifDaAEqAAFe1AcFbcAAAAAF1iFaAAACpACmFmAAAAAAAACrDaAAADG0",
    )
});

static SB: LazyLock<String> = LazyLock::new(|| {
    decode(
        "EqoFscDcrFpmEsF2DfFideFelFpFfFfFiaipqti1ksttikptikqckstekqttgkqttgkqteksttikptikq2fjstgjqttjkqttgkqtekstfkptikq2tijstgjiFkirFsAeACoFsiDaDiADc1AFbBfgdfikijFifegF1FhaikgFag1E2btaieeibggiffdeigFfqDfaiBkF1kEaikhkigeidhhdiegcFfakF1ggkidbiaedksaFffckekidhhdhdikcikiakicjF1deedFhFccgicdekgiFbiaikcfi1kbFibefgEgFdcFkFeFkdcfkF1kfkcickEiFkDacFiEfbiaejcFfffkhkdgkaiei1ehigikhdFikfckF1dhhdikcfgjikhfjicjicgiehdikcikggcifgiejF1jkieFhegikggcikFegiegkfjebhigikggcikdgkaFkijcfkcikfkcifikiggkaeeigefkcdfcfkhkdgkegieidhijcFfakhfgeidieidiegikhfkfckfcjbdehdikggikgkfkicjicjF1dbidikFiggcifgiejkiegkigcdiegfggcikdbgfgefjF1kfegikggcikdgFkeeijcfkcikfkekcikdgkabhkFikaffcfkhkdgkegbiaekfkiakicjhfgqdq2fkiakgkfkhfkfcjiekgFebicggbedF1jikejbbbiakgbgkacgiejkijjgigfiakggfggcibFifjefjF1kfekdgjcibFeFkijcfkfhkfkeaieigekgbhkfikidfcjeaibgekgdkiffiffkiakF1jhbakgdki1dj1ikfkicjicjieeFkgdkicggkighdF1jfgkgfgbdkicggfggkidFkiekgijkeigfiskiggfaidheigF1jekijcikickiggkidhhdbgcfkFikikhkigeidieFikggikhkffaffijhidhhakgdkhkijF1kiakF1kfheakgdkifiggkigicjiejkieedikgdfcggkigieeiejfgkgkigbgikicggkiaideeijkefjeijikhkiggkiaidheigcikaikffikijgkiahi1hhdikgjfifaakekighie1hiaikggikhkffakicjhiahaikggikhkijF1kfejfeFhidikggiffiggkigicjiekgieeigikggiffiggkidheigkgfjkeigiegikifiggkidhedeijcfkFikikhkiggkidhh1ehigcikaffkhkiggkidhh1hhigikekfiFkFikcidhh1hitcikggikhkfkicjicghiediaikggikhkijbjfejfeFhaikggifikiggkigiejkikgkgieeigikggiffiggkigieeigekijcijikggifikiggkideedeijkefkfckikhkiggkidhh1ehijcikaffkhkiggkidhh1hhigikhkikFikfckcidhh1hiaikgjikhfjicjicgiehdikcikggifikigiejfejkieFhegikggifikiggfghigkfjeijkhigikggifikiggkigieeijcijcikfksikifikiggkidehdeijcfdckikhkiggkhghh1ehijikifffffkhsFngErD1pAfBoDd1BlEtFqA2AqoEpDqElAEsEeB2BmADlDkqBtC1FnEpDqnEmFsFsAFnllBbFmDsDiCtDmAB2BmtCgpEplCpAEiBiEoFqFtEqsDcCnFtADnFlEgdkEgmEtEsCtDmADqFtAFrAtEcCqAE1BoFqC1F1DrFtBmFtAC2ACnFaoCgADcADcCcFfoFtDlAFgmFqBq2bpEoAEmkqnEeCtAE1bAEqgDfFfCrgEcBrACfAAABqAAB1AAClEnFeCtCgAADqDoBmtAAACbFiAAADsEtBqAB2FsDqpFqEmFsCeDtFlCeDtoEpClEqAAFrAFoCgFmFsFqEnAEcCqFeCtFtEnAEeFtAAEkFnErAABbFkADnAAeCtFeAfBoAEpFtAABtFqAApDcCGJ",
    )
});

/// 黄经章动。对应 Go `nutationLon2`：变量 `a` 仅首项为 `-1.742*t`，其后归 0。
fn nutation_lon2(t: f64) -> f64 {
    let mut a = -1.742 * t;
    let t2 = t * t;
    let mut dl = 0.0;
    let j = NUT_B.len();
    let mut i = 0;
    while i < j {
        dl += (NUT_B[i + 3] + a) * (NUT_B[i] + NUT_B[i + 1] * t + NUT_B[i + 2] * t2).sin();
        a = 0.0;
        i += 5;
    }
    dl / 100.0 / SECOND_PER_RAD
}

fn e_lon(t: f64, n: i64) -> f64 {
    let t = t / 10.0;
    let mut v = 0.0;
    let mut tn = 1.0;
    let pn = 1usize;
    let m0 = XL0[pn + 1] - XL0[pn];
    for i in 0..6usize {
        let n1 = XL0[pn + i] as i64;
        let n2 = XL0[pn + 1 + i] as i64;
        let n0 = n2 - n1;
        if n0 == 0 {
            continue;
        }
        let m = if n < 0 {
            n2
        } else {
            let mut mm = (3.0_f64 * (n as f64) * (n0 as f64) / m0 + 0.5) as i64 + n1;
            if i != 0 {
                mm += 3;
            }
            if mm > n2 {
                mm = n2;
            }
            mm
        };
        let mut c = 0.0;
        let mut j = n1;
        while j < m {
            c += XL0[j as usize] * (XL0[(j + 1) as usize] + t * XL0[(j + 2) as usize]).cos();
            j += 3;
        }
        v += c * tn;
        tn *= t;
    }
    v /= XL0[0];
    let t2 = t * t;
    let t3 = t2 * t;
    v += (-0.0728 - 2.7702 * t - 1.1019 * t2 - 0.0996 * t3) / SECOND_PER_RAD;
    v
}

fn m_lon(t: f64, n: i64) -> f64 {
    let ob = XL1_ROWS;
    let obl = XL1_OBL;
    let mut tn = 1.0_f64;
    let mut v = 0.0;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let t5 = t4 * t;
    let tx = t - 10.0;
    v += (3.810_344_09 + 8399.684_730_072 * t - 3.319e-05 * t2 + 3.11e-08 * t3 - 2.033e-10 * t4) * SECOND_PER_RAD;
    v += 5028.792_262 * t + 1.112_440_6 * t2 + 0.000_076_99 * t3 - 0.000_023_479 * t4 - 0.000_000_017_8 * t5;
    if tx > 0.0 {
        v += -0.866 + 1.43 * tx + 0.054 * tx * tx;
    }
    let t2 = t2 / 1e4;
    let t3 = t3 / 1e8;
    let t4 = t4 / 1e8;

    let nn = if n < 0 { obl as i64 } else { n * 6 };
    for (i, f) in ob.iter().enumerate() {
        let l = f.len();
        let mut mm = ((nn as f64) * (l as f64) / (obl as f64) + 0.5) as i64;
        if i > 0 {
            mm += 6;
        }
        if mm > l as i64 {
            mm = l as i64;
        }
        let mut c = 0.0;
        let mut j = 0i64;
        while j < mm {
            c += f[j as usize]
                * (f[(j + 1) as usize]
                    + t * f[(j + 2) as usize]
                    + t2 * f[(j + 3) as usize]
                    + t3 * f[(j + 4) as usize]
                    + t4 * f[(j + 5) as usize])
                    .cos();
            j += 6;
        }
        v += c * tn;
        tn *= t;
    }
    v /= SECOND_PER_RAD;
    v
}

fn gxc_sun_lon(t: f64) -> f64 {
    let t2 = t * t;
    let v = -0.043_126 + 628.301_955 * t - 0.000_002_732 * t2;
    let e = 0.016_708_634 - 0.000_042_037 * t - 0.000_000_126_7 * t2;
    -20.495_52 * (1.0 + e * v.cos()) / SECOND_PER_RAD
}

fn ev(t: f64) -> f64 {
    let f = 628.307_585 * t;
    628.332
        + 21.0 * (1.527 + f).sin()
        + 0.44 * (1.48 + f * 2.0).sin()
        + 0.129 * (5.82 + f).sin() * t
        + 0.000_55 * (4.21 + f).sin() * t * t
}

fn sa_lon(t: f64, n: i64) -> f64 {
    e_lon(t, n) + nutation_lon2(t) + gxc_sun_lon(t) + PI
}

fn dt_ext(y: f64, jsd: f64) -> f64 {
    let dy = (y - 1820.0) / 100.0;
    -20.0 + jsd * dy * dy
}

fn dt_calc(y: f64) -> f64 {
    let size = DT_AT.len();
    let y0 = DT_AT[size - 2];
    let t0 = DT_AT[size - 1];
    if y >= y0 {
        let jsd = 31.0;
        if y > y0 + 100.0 {
            return dt_ext(y, jsd);
        }
        let v = dt_ext(y, jsd);
        let dv = dt_ext(y0, jsd) - t0;
        return v - dv * (y0 + 100.0 - y) / 100.0;
    }
    let mut i = 0usize;
    while i < size {
        if y < DT_AT[i + 5] {
            break;
        }
        i += 5;
    }
    let t1 = (y - DT_AT[i]) / (DT_AT[i + 5] - DT_AT[i]) * 10.0;
    let t2 = t1 * t1;
    let t3 = t2 * t1;
    DT_AT[i + 1] + DT_AT[i + 2] * t1 + DT_AT[i + 3] * t2 + DT_AT[i + 4] * t3
}

/// ΔT（转换为日）。
fn dt_t(t: f64) -> f64 {
    dt_calc(t / 365.2425 + 2000.0) / SECOND_PER_DAY
}

fn mv(t: f64) -> f64 {
    let mut v = 8399.71 - 914.0 * (0.7848 + 8328.691_425 * t + 0.000_152_3 * t * t).sin();
    v -= 179.0 * (2.543 + 15542.7543 * t).sin()
        + 160.0 * (0.1874 + 7214.0629 * t).sin()
        + 62.0 * (3.14 + 16657.3828 * t).sin()
        + 34.0 * (4.827 + 16866.9323 * t).sin()
        + 22.0 * (4.9 + 23871.4457 * t).sin()
        + 12.0 * (2.59 + 14914.4523 * t).sin()
        + 7.0 * (0.23 + 6585.7609 * t).sin()
        + 5.0 * (0.9 + 25195.624 * t).sin()
        + 5.0 * (2.32 - 7700.3895 * t).sin()
        + 5.0 * (3.88 + 8956.9934 * t).sin()
        + 5.0 * (0.49 + 7771.3771 * t).sin();
    v
}

fn sa_lon_t(w: f64) -> f64 {
    let v = 628.331_965_331_8;
    let mut t = (w - 1.75347 - PI) / v;
    let v = ev(t);
    t += (w - sa_lon(t, 10)) / v;
    let v = ev(t);
    t += (w - sa_lon(t, -1)) / v;
    t
}

fn msa_lon(t: f64, mn: i64, sn: i64) -> f64 {
    m_lon(t, mn) + (-3.4e-6) - (e_lon(t, sn) + gxc_sun_lon(t) + PI)
}

fn msa_lon_t(w: f64) -> f64 {
    let v = 7771.377_145_002_04;
    let mut t = (w + 1.08472) / v;
    t += (w - msa_lon(t, 3, 3)) / v;
    let v = mv(t) - ev(t);
    t += (w - msa_lon(t, 20, 10)) / v;
    t += (w - msa_lon(t, -1, 60)) / v;
    t
}

fn sa_lon_t2(w: f64) -> f64 {
    let v = 628.331_965_331_8;
    let mut t = (w - 1.75347 - PI) / v;
    t -= (0.000_005_297 * t * t
        + 0.033_416_6 * (4.669_257 + 628.307_585 * t).cos()
        + 0.000_206_1 * (2.678_23 + 628.307_585 * t).cos() * t)
        / v;
    t += (w - e_lon(t, 8) - PI + (20.5 + 17.2 * (2.1824 - 33.757_05 * t).sin()) / SECOND_PER_RAD) / v;
    t
}

fn msa_lon_t2(w: f64) -> f64 {
    let v = 7771.377_145_002_04;
    let mut t = (w + 1.08472) / v;
    let mut t2 = t * t;
    t -= (-0.000_033_09 * t2
        + 0.109_76 * (0.784_758 + 8328.691_424_6 * t + 0.000_152_292 * t2).sin()
        + 0.022_24 * (0.187_40 + 7214.062_865_4 * t - 0.000_218_48 * t2).sin()
        - 0.033_42 * (4.669_257 + 628.307_585 * t).cos())
        / v;
    t2 = t * t;
    let l = m_lon(t, 20)
        - (4.895_063_2
            + 628.331_965_331_8 * t
            + 0.000_005_297 * t2
            + 0.033_416_6 * (4.669_257 + 628.307_585 * t).cos()
            + 0.000_206_1 * (2.678_23 + 628.307_585 * t).cos() * t
            + 0.000_349 * (4.6261 + 1256.61517 * t).cos()
            - 20.5 / SECOND_PER_RAD);
    let v = 7771.38
        - 914.0 * (0.7848 + 8328.691_425 * t + 0.000_152_3 * t2).sin()
        - 179.0 * (2.543 + 15542.7543 * t).sin()
        - 160.0 * (0.1874 + 7214.0629 * t).sin();
    t += (w - l) / v;
    t
}

fn qi_high(w: f64) -> f64 {
    let mut t = sa_lon_t2(w) * 36525.0;
    t = t - dt_t(t) + ONE_THIRD;
    let v = (t + 0.5).rem_euclid(1.0) * SECOND_PER_DAY;
    if !(1200.0..=SECOND_PER_DAY - 1200.0).contains(&v) {
        t = sa_lon_t(w) * 36525.0 - dt_t(t) + ONE_THIRD;
    }
    t
}

fn shuo_high(w: f64) -> f64 {
    let mut t = msa_lon_t2(w) * 36525.0;
    t = t - dt_t(t) + ONE_THIRD;
    let v = (t + 0.5).rem_euclid(1.0) * SECOND_PER_DAY;
    if !(1800.0..=SECOND_PER_DAY - 1800.0).contains(&v) {
        t = msa_lon_t(w) * 36525.0 - dt_t(t) + ONE_THIRD;
    }
    t
}

fn qi_low(w: f64) -> f64 {
    let v = 628.331_965_331_8;
    let mut t = (w - 4.895_062_166) / v;
    t -= (53.0 * t * t + 334_116.0 * (4.67 + 628.307_585 * t).cos() + 2061.0 * (2.678 + 628.3076 * t).cos() * t)
        / v
        / 10_000_000.0;
    let n = 48_950_621.66
        + 6_283_319_653.318 * t
        + 53.0 * t * t
        + 334_166.0 * (4.669_257 + 628.307_585 * t).cos()
        + 3489.0 * (4.6261 + 1256.61517 * t).cos()
        + 2060.6 * (2.678_23 + 628.307_585 * t).cos() * t
        - 994.0
        - 834.0 * (2.1824 - 33.757_05 * t).sin();
    t -= (n / 10_000_000.0 - w) / 628.332 + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / SECOND_PER_DAY / 36525.0;
    t * 36525.0 + ONE_THIRD
}

fn shuo_low(w: f64) -> f64 {
    let v = 7771.377_145_002_04;
    let mut t = (w + 1.08472) / v;
    t -= (-0.000_033_1 * t * t + 0.109_76 * (0.785 + 8328.691 * t).cos() + 0.022_24 * (0.187 + 7214.0629 * t).cos()
        - 0.033_42 * (4.669 + 628.3076 * t).cos())
        / v
        + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / SECOND_PER_DAY / 36525.0;
    t * 36525.0 + ONE_THIRD
}

/// 计算合朔（初一）儒略日。对应 Go `CalcShuo`。
pub fn calc_shuo(jd: f64) -> f64 {
    let size = SHUO_KB.len();
    let mut d = 0.0;
    let pc = 14.0_f64;
    let jd = jd + 2451545.0;
    let f1 = SHUO_KB[0] - pc;
    let f2 = SHUO_KB[size - 1] - pc;
    let f3 = 2_436_935.0_f64;
    if jd < f1 || jd >= f3 {
        d = (shuo_high(((jd + pc - 2451551.0) / 29.5306).floor() * PI_2) + 0.5).floor();
    } else if jd >= f1 && jd < f2 {
        let mut i = 0usize;
        while i < size {
            if jd + pc < SHUO_KB[i + 2] {
                break;
            }
            i += 2;
        }
        d = SHUO_KB[i] + SHUO_KB[i + 1] * ((jd + pc - SHUO_KB[i]) / SHUO_KB[i + 1]).floor();
        d = (d + 0.5).floor();
        if d == 1_683_460.0 {
            d += 1.0;
        }
        d -= 2451545.0;
    } else if jd >= f2 && jd < f3 {
        d = (shuo_low(((jd + pc - 2451551.0) / 29.5306).floor() * PI_2) + 0.5).floor();
        let from = ((jd - f2) / 29.5306) as i64;
        let n = SB.as_bytes()[from as usize];
        if n == b'1' {
            d += 1.0;
        } else if n == b'2' {
            d -= 1.0;
        }
    }
    d
}

/// 计算节气儒略日。对应 Go `CalcQi`。
pub fn calc_qi(jd: f64) -> f64 {
    let size = QI_KB.len();
    let mut d = 0.0;
    let pc = 7.0_f64;
    let jd = jd + 2451545.0;
    let f1 = QI_KB[0] - pc;
    let f2 = QI_KB[size - 1] - pc;
    let f3 = 2_436_935.0_f64;
    if jd < f1 || jd >= f3 {
        d = (qi_high(((jd + pc - 2451259.0) / 365.2422 * 24.0).floor() * PI / 12.0) + 0.5).floor();
    } else if jd >= f1 && jd < f2 {
        let mut i = 0usize;
        while i < size {
            if jd + pc < QI_KB[i + 2] {
                break;
            }
            i += 2;
        }
        d = QI_KB[i] + QI_KB[i + 1] * ((jd + pc - QI_KB[i]) / QI_KB[i + 1]).floor();
        d = (d + 0.5).floor();
        if d == 1_683_460.0 {
            d += 1.0;
        }
        d -= 2451545.0;
    } else if jd >= f2 && jd < f3 {
        d = (qi_low(((jd + pc - 2451259.0) / 365.2422 * 24.0).floor() * PI / 12.0) + 0.5).floor();
        let from = ((jd - f2) / 365.2422 * 24.0) as i64;
        let n = QB.as_bytes()[from as usize];
        if n == b'1' {
            d += 1.0;
        } else if n == b'2' {
            d -= 1.0;
        }
    }
    d
}

fn qi_accurate(w: f64) -> f64 {
    let t = sa_lon_t(w) * 36525.0;
    t - dt_t(t) + ONE_THIRD
}

/// 把儒略日精化到最近的节气。对应 Go `QiAccurate2`。
pub fn qi_accurate_2(jd: f64) -> f64 {
    let d = PI / 12.0;
    let w = ((jd + 293.0) / 365.2422 * 24.0).floor() * d;
    let a = qi_accurate(w);
    if a - jd > 5.0 {
        qi_accurate(w - d)
    } else if a - jd < -5.0 {
        qi_accurate(w + d)
    } else {
        a
    }
}

/// 计算指定农历年附近第 `cycle_offset` 个 8 相月相的儒略日。
///
/// `phase_index` 取值按 `新月、蛾眉月、上弦月、盈凸月、满月、亏凸月、下弦月、残月`，
/// 奇数项会由调用方按 `tyme4rs` 规则顺延一天。
pub fn moon_phase_julian_day(lunar_year: i32, cycle_offset: i32, phase_index: usize) -> f64 {
    const PHASE_OFFSETS: [f64; 4] = [0.0, 0.25, 0.5, 0.75];
    let base = ((f64::from(lunar_year - 2000) * 365.2422 / 29.530_588_86).floor() as i32) + cycle_offset;
    let angle = (f64::from(base) + PHASE_OFFSETS[(phase_index % 8) / 2]) * PI_2;
    let t = msa_lon_t(angle) * 36525.0;
    J2000 + ONE_THIRD + t - dt_t(t)
}
