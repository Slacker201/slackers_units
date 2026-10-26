pub trait UnitSystem: Default + std::fmt::Debug + Copy + Clone {
    const TIME_SUFFIX: &str;
    const DISTANCE_SUFFIX: &str;
    const MASS_SUFFIX: &str;
    const ELECTRIC_CURRENT_SUFFIX: &str;
    const TEMPERATURE_SUFFIX: &str;
    const AMOUNT_SUFFIX: &str;
    const LUMENOSITY_SUFFIX: &str;
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Metric;
impl UnitSystem for Metric {
    const TIME_SUFFIX: &str = "s";

    const DISTANCE_SUFFIX: &str = "m";

    const MASS_SUFFIX: &str = "kg";

    const ELECTRIC_CURRENT_SUFFIX: &str = "A";

    const TEMPERATURE_SUFFIX: &str = "K";

    const AMOUNT_SUFFIX: &str = "mol";

    const LUMENOSITY_SUFFIX: &str = "cd";
}
