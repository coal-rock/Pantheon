use dioxus::logger::tracing::info;
use dioxus::prelude::*;

use crate::components::agents_overview::AgentsOverview;
use crate::components::agents_table::AgentsTable;
use crate::components::console::Console;
use crate::components::navbar::Navbar;
use crate::components::notepad::Notepad;
use crate::components::placeholder_panel::Placeholder;
use crate::components::sidebar::Sidebar;
use crate::components::tartarus_overview::TartarusOverview;

pub enum PanelType {
    Console,
    Notepad,
    AgentsTable,
    AgentsOverview,
    TartarusOverview,
    Placeholder,
}

#[derive(Clone)]
pub struct PanelManager {
    pub layout: Signal<Vec<i32>>,
    pub open_panels: Signal<Vec<(i32, Element)>>,
    current_id: i32,
}

impl PanelManager {
    pub fn stringify_external_layout(layout: Vec<i32>) -> String {
        layout
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("-")
    }

    pub fn stringify_layout(&self) -> String {
        self.layout
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("-")
    }

    pub fn remove_panel(&mut self, panel_id: i32) -> bool {
        let mut index: Option<usize> = None;

        for (idx, (id, _panel)) in self.open_panels.read().iter().enumerate() {
            if *id == panel_id {
                index = Some(idx);
                break;
            }
        }

        match index {
            Some(index) => {
                let _ = self.open_panels.write().remove(index);
                true
            }
            None => false,
        }
    }

    fn panel_from_enum(&mut self, panel_type: PanelType) -> (i32, Element) {
        let panel = (
            self.current_id,
            match panel_type {
                PanelType::Console => rsx! { Console{id: self.current_id} },
                PanelType::Notepad => rsx! { Notepad {id: self.current_id} },
                PanelType::AgentsTable => rsx! { AgentsTable{id: self.current_id} },
                PanelType::AgentsOverview => rsx! { AgentsOverview {id: self.current_id} },
                PanelType::TartarusOverview => rsx! { TartarusOverview {id: self.current_id} },
                PanelType::Placeholder => rsx! { Placeholder { } },
            },
        );

        self.current_id += 1;

        return panel;
    }

    pub fn add_element_unsafe(&mut self, panel: Element, id: i32) {
        self.open_panels.write().push((id, panel));
    }

    pub fn add_panel(&mut self, panel_type: PanelType) {
        let panel = self.panel_from_enum(panel_type);
        self.open_panels.write().push(panel);
    }

    pub fn set_layout(&mut self, layout: Vec<i32>) {
        self.layout.set(layout.clone());
        // self.open_panels.set(vec![]);
    }

    pub fn new(layout: Vec<i32>, panels: Vec<PanelType>) -> PanelManager {
        let open_panels: Vec<(i32, Element)> = vec![];
        let current_id = 0;

        let mut panel_manager = PanelManager {
            layout: Signal::new(layout),
            open_panels: Signal::new(open_panels),
            current_id,
        };

        for panel in panels {
            panel_manager.add_panel(panel);
        }

        panel_manager
    }
}

#[component]
pub fn Page() -> Element {
    let show_sidebar = use_signal(|| true);

    let panel_manager = use_context_provider(|| {
        PanelManager::new(
            vec![2, 2],
            vec![
                PanelType::AgentsOverview,
                PanelType::AgentsTable,
                PanelType::TartarusOverview,
                PanelType::Console,
            ],
        )
    });

    // FIXME: find a slightly nicer way of doing this
    let script = r#"
        // shim for draggable
        const initWindowManagement = () => {
            try {
                const swappable = new Draggable.Swappable(document.querySelectorAll('div'), {
                    draggable: '.draggable',
                    handle: '.handle',
                    mirror: {
                        constrainDimensions: true,
                    },
                    plugins: [Draggable.Plugins.SortAnimation, Draggable.Plugins.ResizeMirror],
                    swapAnimation: {
                      duration: 200,
                      easingFunction: 'ease-in-out',
                    },
                });

                swappable.on('drag:start', (event) => {
                    const draggedItem = event.source;
                    draggedItem.style.transform = 'scale(1.00)';
                    draggedItem.classList.add("blur-sm");
                });

                swappable.on('drag:move', (event) => {

                });

                swappable.on('drag:stop', (event) => {
                    const draggedItem = event.source;
                    draggedItem.style.transition = 'transform 0.2s ease';
                    draggedItem.style.transform = 'scale(1)';
                    draggedItem.classList.remove("blur");
                });

                swappable.on('mirror:created', (event) => {
                    const mirror = event.mirror;
                    mirror.style.zIndex = '1000'; 
                    mirror.style.position = 'absolute';
                    mirror.classList.remove("blur-sm");
                    mirror.classList.add("transition-color");
                    // mirror.classList.add("!scale-90");
                    mirror.classList.add("!border-blue-500");
                });

                console.log("swappable init success");
                return true;
            }
            catch {
                return false;
            }
        }
      
        
        // egregious hack, use setInterval or a callback?
        const attemptInitWindowManagement = (counter) => {
            let success = initWindowManagement();

            if (!success && counter < 15) {
                setTimeout(attemptInitWindowManagement, 100, counter + 1);
            }
        }

        attemptInitWindowManagement(0);
    "#;

    // let _ = use_resource(move || async move { document::eval(script).await });

    rsx! {
        div {
            class: "flex flex-col h-screen",

            Navbar {
                show_sidebar: show_sidebar,
                anemic: false,
            }
            div {
                class: "flex flex-row grow",

                Sidebar {
                    should_show: show_sidebar
                }

                LayoutHandler {}
            }
        }
    }
}

#[component]
fn LayoutHandler() -> Element {
    // TODO: fix clone spam
    let panel_manager = use_context::<PanelManager>();

    let mut panel_index = 0;
    rsx! {
        div {
            class: "grow bg-zinc-700 flex space-between items-center flex-row gap-2 p-2",
            for col in panel_manager.layout.read().clone() {
                PanelColumn {
                    for row in 0..col {
                        match panel_manager.open_panels.read().clone().get(panel_index) {
                            Some(panel) => {
                                panel_index += 1;
                                rsx! { {panel.1.clone()} }
                            },
                            None => {
                                panel_index += 1;
                                rsx! { Placeholder {} }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PanelColumn(children: Element) -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full p-0 grow shrink basis-0 w-0 gap-2",
            {children}
        }
    }
}
