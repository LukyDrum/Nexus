use std::collections::HashMap;

use iced::{Subscription, Task, futures::StreamExt, window::Id as WindowId};
use iced_exwlshell::settings::LayerShellSettings;
use koolctl::ControlMessage;

use crate::{
    BuildContext,
    language::Value,
    runner::{
        RunnerMessage, Signal, WidgetId, WidgetInstance, WidgetSettings, init_control_server,
        init_signal_channel, take_control_receiver, take_signal_receiver,
    },
};

#[derive(Debug)]
pub struct KoolRunner {
    instances: HashMap<WindowId, WidgetInstance>,
    windows: HashMap<WidgetId, WindowId>,
}

impl KoolRunner {
    pub fn run() -> Result<(), iced_exwlshell::Error> {
        init_control_server();
        init_signal_channel();

        let mut settings: iced_exwlshell::Settings = WidgetSettings::default().into();
        settings.layer_settings.start_mode = iced_exwlshell::settings::StartMode::Background;

        iced_exwlshell::daemon(KoolRunner::new, "Kool", Self::update, Self::view)
            .settings(settings)
            .subscription(|_| {
                Subscription::batch([Self::control_subscription(), Self::signal_subscription()])
            })
            .run()
    }

    fn new() -> Self {
        Self {
            instances: HashMap::new(),
            windows: HashMap::new(),
        }
    }

    fn signal_subscription() -> Subscription<RunnerMessage> {
        Subscription::run(|| {
            let receiver = take_signal_receiver();
            receiver.map(RunnerMessage::Signal)
        })
    }

    fn control_subscription() -> Subscription<RunnerMessage> {
        Subscription::run(|| {
            let receiver = take_control_receiver();
            receiver.map(RunnerMessage::Control)
        })
    }

    fn update(&mut self, message: RunnerMessage) -> Task<RunnerMessage> {
        match message {
            RunnerMessage::Control(control_msg) => {
                match control_msg {
                    ControlMessage::Run { path } => {
                        // This should handle both running new and reloading widgets
                        let id = WidgetId::new(path);

                        let already_spawned = self.windows.contains_key(&id);

                        match WidgetInstance::init(id.clone()) {
                            Ok(instance) => {
                                let window_id =
                                    self.windows.entry(id).or_insert_with(WindowId::unique);
                                let entry = self.instances.entry(*window_id).insert_entry(instance);
                                let instance = entry.get();

                                if already_spawned {
                                    return Task::batch(Self::update_settings_tasks(
                                        *window_id,
                                        instance.settings.clone(),
                                    ));
                                } else {
                                    let msg = RunnerMessage::NewLayerShell {
                                        settings: instance.settings.clone().into(),
                                        id: *window_id,
                                    };

                                    return Task::done(msg);
                                }
                            }
                            Err(error) => {
                                println!("{error}");
                            }
                        }
                    } // ControlMessage::Close(id) => {
                      //     if let Some(window_id) = self.windows.remove(&id) {
                      //         self.instances.remove(&window_id);

                      //         let msg = RunnerMessage::RemoveWindow(window_id);

                      //         return Task::done(msg);
                      //     }
                      // }
                      // ControlMessage::CloseAll => {
                      //     let mut tasks = Vec::with_capacity(self.windows.len());
                      //     for (_, window_id) in self.windows.drain() {
                      //         self.instances.remove(&window_id);

                      //         let msg = RunnerMessage::RemoveWindow(window_id);
                      //         tasks.push(Task::done(msg));
                      //     }

                      //     return Task::batch(tasks);
                      // }
                }
            }
            RunnerMessage::Signal(signal) => {
                match signal {
                    Signal::Refresh => {
                        // No need to do anything
                    }
                }
            }
            RunnerMessage::Widget { id, message } => {
                todo!("handle the message on the widget designated by its id")
            }

            // The rest of `RunnerMessage` is exwlshell auto-injected
            _ => {}
        }

        Task::none()
    }

    fn view<'a>(&'a self, window_id: WindowId) -> iced::Element<'a, RunnerMessage> {
        let Some(instance) = self.instances.get(&window_id) else {
            return iced::widget::space().into();
        };

        let Ok(Value::Element(element)) = instance.view_function.call_with_default_args() else {
            return iced::widget::text("View function did not return an element").into();
        };

        let context = BuildContext {
            default_style: Default::default(),
        };

        element
            .build(&context)
            .map(|message| RunnerMessage::Widget {
                id: instance.id.clone(),
                message,
            })
    }

    fn update_settings_tasks(
        id: WindowId,
        settings: WidgetSettings,
    ) -> impl IntoIterator<Item = Task<RunnerMessage>> {
        let settings: LayerShellSettings = settings.into();
        [
            Task::done(RunnerMessage::AnchorSizeChange {
                id,
                anchor: settings.anchor,
                size: settings.size.unwrap_or_default(),
            }),
            Task::done(RunnerMessage::LayerChange {
                id,
                layer: settings.layer,
            }),
            Task::done(RunnerMessage::MarginChange {
                id,
                margin: settings.margin,
            }),
            Task::done(RunnerMessage::ExclusiveZoneChange {
                id,
                zone_size: settings.exclusive_zone,
            }),
        ]
    }
}
