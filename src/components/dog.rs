use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 1. ⚡ 引入方案一的全局连接池函数（仅在服务器编译时有效）
#[cfg(feature = "server")]
use crate::db::pool;

#[derive(Deserialize, Serialize, Clone)]
struct DogApi {
    message: String,
}

#[component]
pub fn Dog() -> Element {
    // 使用 use_resource 异步拉取第三方 API 的随机狗狗图片
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { 
                src: img_src.cloned().unwrap_or_default(),
                style: "max-width: 300px; display: block; margin-bottom: 10px;" 
            }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            
            button {
                id: "save",
                // ⚡ 修复点 2：在 Dioxus 事件中执行异步 await，必须外层包裹 spawn
                onclick: move |_| {
                    if let Some(current) = img_src.cloned() {
                        img_src.restart();
                        // 创建微任务队列来执行异步 Server Function
                        spawn(async move {
                            let _ = save_dog(current).await;
                        });
                    }
                },
                "save!"
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct _Dog {
    pub id: i32,
    pub url: String,
}

/// ⚡ 修复点 1：使用 Dioxus 0.7 官方推荐的 `#[server]` 宏
/// Dioxus 会在底层自动将其映射为服务器路由并处理 WASM 端的网络请求
#[post("/api/dogs")]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    // 🎯 完美应用方案一：直接在括号中调用 pool()！
    // 提示：SQLx 的宏接收引用，由于 pool() 返回的是 &'static PgPool，本身就是引用，直接丢进去即可
    match sqlx::query!("INSERT INTO dogs (url) values ($1)", image)
        .execute(pool())
        .await 
    {
        Ok(res) => println!("插入成功: {:?}", res), // 在全栈控制台打印日志
        Err(e) => {
            eprintln!("数据库报错详情: {:?}", e);
            return Err(ServerFnError::new(e.to_string()));
        }
    }

    // （可选演示）你可以随时在下方继续直接使用 pool() 查库
     let dogs = sqlx::query_as!(_Dog, "SELECT id, url FROM dogs")
         .fetch_all(pool())
         .await
         .map_err(|e| ServerFnError::new(e.to_string()))?;
     println!("当前的全部狗狗: {:?}", dogs);

    Ok(())
}

