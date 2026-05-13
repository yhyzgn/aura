//! Disable all Cascader interaction while keeping the current value visible.

use aura_components::{Cascader, CascaderOption};
use gpui::Context;

pub fn disabled_cascader(cx: &mut Context<Cascader>) -> Cascader {
    Cascader::new(region_options(), cx)
        .disabled(true)
        .selected_path(["zhejiang", "hangzhou", "xihu"])
        .width_md()
}

fn region_options() -> Vec<CascaderOption> {
    vec![CascaderOption::new("zhejiang", "浙江").child(
        CascaderOption::new("hangzhou", "杭州").child(CascaderOption::new("xihu", "西湖区")),
    )]
}
