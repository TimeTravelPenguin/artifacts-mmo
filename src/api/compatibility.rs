pub trait FromLegacy {
    type LegacyType;

    fn from_legacy(legacy: Self::LegacyType) -> Self;
}
