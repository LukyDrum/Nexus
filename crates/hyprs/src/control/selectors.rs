use crate::types::HyprsType;

const WORKSPACE_KEY: &str = "workspace";
const WINDOW_KEY: &str = "workspace";

pub trait Selector {
    fn as_arg_pair_string(&self) -> String;
}

impl Selector for () {
    fn as_arg_pair_string(&self) -> String {
        String::new()
    }
}

pub struct WorkspaceSelector<T>(pub T);
pub struct WindowSelector<T>(pub T);

impl<T> Selector for WorkspaceSelector<T>
where
    T: HyprsType,
{
    fn as_arg_pair_string(&self) -> String {
        format!("{WORKSPACE_KEY} = {}", self.0.as_selector())
    }
}

impl<T> Selector for WindowSelector<T>
where
    T: HyprsType,
{
    fn as_arg_pair_string(&self) -> String {
        format!("{WINDOW_KEY} = {}", self.0.as_selector())
    }
}
