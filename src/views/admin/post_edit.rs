use dioxus::prelude::*;


/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn PostEdit() -> Element {
    let mut title = use_signal(|| String::new());
    let mut content = use_signal(|| String::new());
    let mut is_published = use_signal(|| false);
    let navigator= use_navigator();

    let on_submit = move |_| {
        async move {
            //if let Ok(_) = crate::view
        }

    };

    
    rsx! {
        div { class:"max-w-4xl mx-auto p-6 bg-black shadow rounded-lg",
            h2 { class: "text-2xl font-bold mb-4", "📝 发布新文章" }
            form { onsubmit: on_submit,class:"space-y-4",
                div {
                    label { class: "block text-sm font-medium", "文章标题" }
                    input {
                        class: "w-full border p-2 rounded",
                        value: "{title}",
                        oninput: move |e| title.set(e.value())
                    }

                }
            }
            
        }
    }
}
