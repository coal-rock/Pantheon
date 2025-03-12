use dioxus::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::sidebar::Sidebar;

#[component]
pub fn Page(children: Element) -> Element {
    let show_sidebar = use_signal(|| true);

    // FIXME: find a slightly nicer way of doing this
    let script = r#"
        setTimeout(() => {
            let script = document.createElement('script');
            script.setAttribute('src','https://cdn.jsdelivr.net/npm/@shopify/draggable/build/umd/index.min.js');
            document.head.appendChild(script);

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
                // draggedItem.style.transition = 'transform 0.2s ease';
                draggedItem.style.transform = 'scale(0.95)';
            });

            swappable.on('drag:move', (event) => {
                const draggedItem = event.source;
                draggedItem.style.transform = 'scale(0.95) transform(0.2)';
            });

            swappable.on('drag:stop', (event) => {
                const draggedItem = event.source;
                draggedItem.style.transition = 'transform 0.2s ease';
                draggedItem.style.transform = 'scale(1)';
            });

            console.log("swappable init over")
        }, 50);
    "#;

    let _ = use_resource(move || async move { document::eval(script).await });

    rsx! {
        div {
            class: "flex flex-col h-screen",

            Navbar {
                show_sidebar: show_sidebar,
            }
            div {
                class: "flex flex-row grow",

                Sidebar {
                    should_show: show_sidebar
                }

                {children}
            }
        }
    }
}
