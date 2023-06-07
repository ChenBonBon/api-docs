pub mod utils {
    use jsdoc::Input;
    use swc_common::BytePos;

    pub fn convert_code_to_input(code: &String) -> Input {
        Input::new(BytePos::DUMMY, BytePos::SYNTHESIZED, &code)
    }
}
