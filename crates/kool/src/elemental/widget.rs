use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    KoolWidget,
    element::{BuildContext, KoolElement, kool},
    elemental::{Signal, SignalReceiver},
    language::{Function, SharedEnvironment, Value},
    settings::WidgetSettings,
    style::WidgetStyle,
};

const VIEW_FUNCTION_NAME: &str = "view";

#[derive(Clone, Debug)]
pub struct ElementalWidget {
    name: String,
    settings: WidgetSettings,
    style: WidgetStyle,

    root: Arc<KoolElement>,
    _runtime: SharedEnvironment,
    // Must be `Arc` for `Self` to be `Clone`, in reality only a single instance will be used.
    signals: Arc<Mutex<SignalReceiver>>,
    view_function: Arc<Function>,
}

#[derive(Clone, Debug)]
pub enum ElementalMessage {
    Empty,
    Signal(Signal),
}

impl ElementalWidget {
    pub fn new(
        name: String,
        settings: WidgetSettings,
        style: WidgetStyle,
        runtime: SharedEnvironment,
        signals: SignalReceiver,
    ) -> Self {
        let Value::Function(view_function) = runtime
            .get_variable(VIEW_FUNCTION_NAME)
            .expect("No view function found")
        else {
            panic!("No view function found");
        };
        let view_function = view_function.clone();
        let root = Self::get_root(&view_function);

        Self {
            name,
            settings,
            style,

            root,
            _runtime: runtime,
            signals: Arc::new(Mutex::new(signals)),
            view_function,
        }
    }

    fn get_root(view_function: &Function) -> Arc<KoolElement> {
        let element = match view_function.call_with_default_args() {
            Ok(Value::Element(element)) => return element,
            Ok(value) => KoolElement::Error(kool::Error::new(format!(
                "view function returned non-element value: {value}"
            ))),
            Err(error) => KoolElement::Error(kool::Error::new(format!(
                "view function call ended with error: {error:?}"
            ))),
        };

        Arc::new(element)
    }

    fn wait_for_signal_task(&self) -> iced::task::Task<ElementalMessage> {
        let signals = self.signals.clone();
        let future = async move {
            signals
                .lock()
                .await
                .recv()
                .await
                .map_or(ElementalMessage::Empty, ElementalMessage::Signal)
        };

        iced::task::Task::future(future)
    }
}

impl KoolWidget<ElementalMessage> for ElementalWidget {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn settings(&self) -> WidgetSettings {
        self.settings.clone()
    }

    fn update(&mut self, message: ElementalMessage) -> iced::Task<ElementalMessage> {
        self.root = Self::get_root(&self.view_function);

        let task = match message {
            ElementalMessage::Empty => iced::Task::none(),
            ElementalMessage::Signal(signal) => {
                match signal {
                    Signal::Refresh => {}
                }

                iced::Task::none()
            }
        };

        iced::task::Task::batch([task, self.wait_for_signal_task()])
    }

    fn view<'a>(&'a self) -> impl Into<iced::Element<'a, ElementalMessage>> {
        let context = BuildContext {
            style: self.style.style_tree(),
        };

        self.root.build(&context)
    }

    fn style(&self) -> &crate::style::WidgetStyle {
        &self.style
    }

    fn startup_task(&self) -> iced::Task<ElementalMessage> {
        self.wait_for_signal_task()
    }
}
