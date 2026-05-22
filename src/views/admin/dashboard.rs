use dioxus::prelude::*;

#[component]
pub fn AdminDashboard() -> Element {
    rsx! {
        div { class: "max-w-6xl mx-auto",
            div { class: "flex justify-between items-center mb-6",
                h2 { class: "text-2xl font-bold text-gray-800", "CMS 控制面板" }
                button { 
                    class: "bg-green-600 text-white px-4 py-2 rounded shadow hover:bg-green-700",

                    "➕ 新增文章"
                }
            }

        }
    }

}