use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::SettingGroupBuilder;

pub(crate) fn define() -> TargetIsa {
    let mut settings = SettingGroupBuilder::new("loongarch64");

    settings.add_bool(
        "has_lsx",
        "Loongson SIMD Extension support.",
        "",
        true,
    );
    settings.add_bool(
        "has_lasx",
        "Loongson Advanced SIMD Extension support.",
        "",
        true,
    );

    TargetIsa::new("loongarch64", settings.build())
}
