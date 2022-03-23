use pinyin::ToPinyin;
// use once_cell::sync::Lazy;
use teloxide::prelude2::*;
// use pinyin::{ToPinyin, ToPinyinMulti};

/// 判斷是「共富國際」的可能率。
///
/// 回傳值將會是一個介於 0~1 的數字。
fn get_gong_fu_possibility(name: &str) -> f32 {
    let name = name.replacen('-', "", 1);
    let matched_characters = name
        .as_str()
        .to_pinyin()
        .flatten()
        .map(|v| v.plain())
        .filter(|v| matches!(*v, "gong" | "fu" | "guo" | "ji"))
        .count();

    {
        // https://www.desmos.com/calculator/wmvf92efik

        let x = matched_characters as f32;
        let n = name.chars().count() as f32;

        if n > 0.0 {
            1.0 - (1.0 - (x / n)).abs()
        } else {
            0.0
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("I: 開始偵測有「共富」字眼的內容⋯⋯");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        let possibility = message.text().map(get_gong_fu_possibility).unwrap_or(0.0);

        if possibility == 1.0 {
            log::info!("spam possibility = 1.0, kick out!");

            if let Some(sender_chat) = message.sender_chat() {
                bot.kick_chat_member(message.chat.id, sender_chat.id)
                    .await?;
                bot.send_message(
                    message.chat.id,
                    format!(
                        "{} 極有可能是廣告 - 已自動踢出，可自動拉回。",
                        sender_chat.title().unwrap_or("<?>")
                    ),
                )
                .await?;
            }
        }
        bot.send_message(message.chat.id, possibility.to_string())
            .await?;
        respond(())
    })
    .await;
}

#[cfg(test)]
mod tests {
    use pinyin::ToPinyin;

    #[test]
    fn experiment_to_pinyin_result() {
        let name = "共富-國際";
        // let mut name_pinyin_iter = name.to_pinyin();
        let matches = name
            .to_pinyin()
            .flatten()
            .map(|v| v.plain())
            .filter(|v| matches!(*v, "gong" | "fu" | "guo" | "ji"))
            .count();

        assert_eq!(matches, 4);
    }

    #[test]
    fn test_get_gong_fu_possibility_100_percent() {
        assert_eq!(super::get_gong_fu_possibility("共富-國際"), 1.0);
    }

    #[test]
    fn test_get_gong_fu_possibility_0_percent() {
        assert_eq!(super::get_gong_fu_possibility("Hello, World!"), 0.0);
        assert_eq!(super::get_gong_fu_possibility("嗨"), 0.0);
    }
}
