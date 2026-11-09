use bnum::Integer;

cfg_select! {
    feature = "converter_256" => {
        pub type NumberRepr = Integer<true, 256, 0, 0>;
    }
    feature = "converter_128" => {
        pub type NumberRepr = Integer<true, 128, 0, 0>;
    }
    feature = "converter_64" => {
        pub type NumberRepr = Integer<true, 64, 0, 0>;
    }
    feature = "converter_32" => {
        pub type NumberRepr = Integer<true, 32, 0, 0>;
    }
    feature = "converter_16" => {
        pub type NumberRepr = Integer<true, 16, 0, 0>;
    }
    feature = "converter_8" => {
        pub type NumberRepr = Integer<true, 8, 0, 0>;
    }
    _ => {
        pub type NumberRepr = Integer<true, 64, 0, 0>;
    }
}