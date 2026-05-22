// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::{Blog, Home, Navbar,Dogs,admin::PostEdit};

mod db;
/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Navbar)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Home {},
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] with the `:` syntax.
        // In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/blog/:id")]
        // Fields of the route variant will be passed to the component as props. In this case, the blog component must accept
        // an `id` prop of type `i32`.
        Blog { id: i32 },
        #[route("/dogs/")]
        // Fields of the route variant will be passed to the component as props. In this case, the blog component must accept
        // an `id` prop of type `i32`.
        Dogs {   },


        #[route("/admin/post/new/")]
            // Fields of the route variant will be passed to the component as props. In this case, the blog component must accept
            // an `id` prop of type `i32`.
        PostEdit {   },

}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {

    #[cfg(feature = "server")]
    {
        // 🚀 Dioxus 0.7 标配全栈服务器启动函数：serve
        // 它会自主在当前活动主进程里拉起合规的异步环境，杜绝运行时错位造成的 PoolTimedOut
        dioxus::server::serve(|| async move {
            use dioxus::server::{axum, ServeConfig};

            // 🎯 在此处调用你的异步初始化，连接池将直接诞生在生存期完美契合的主运行时内
            crate::db::create_pool().await;

            // 这里可以顺手执行一次简单测试 SQL，强行触发物理连库，不通过则在启动时立斩崩溃
            println!("🔌 验证 PostgreSQL 连接稳定性...");
            sqlx::query("SELECT 1")
                .execute(crate::db::pool())
                .await
                .expect("❌ 数据库启动健康检查失败！请核对配置！");
            println!("✅ 数据库健康检查通过，物理建连成功！");

            // 构建并返回搭载 Dioxus 全栈中间件的 Axum 路由容器
            Ok(axum::Router::new().serve_dioxus_application(
                ServeConfig::default(),
                || dioxus::prelude::rsx! { App {} }
            ))
        });
    }


    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    #[cfg(not(feature = "server"))]
    {
        dioxus::launch(App);
    }

}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}
