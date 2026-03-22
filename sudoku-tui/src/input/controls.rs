// input/controls.rs: Control struct 定义

#[derive(Debug, Clone, Copy)]
pub struct Control {
    pub key: &'static str,
    pub label: &'static str,
}
