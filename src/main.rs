use pinyin::ToPinyin;
use teloxide::prelude2::*;

const DEBUG_MODE: bool = false;

/// 判斷是「共富國際」的可能率。
///
/// 回傳值將會是一個介於 0~1 的數字。
fn get_gong_fu_possibility(name: &str) -> f32 {
    let name = name.replace('-', "").replace('_', "");
    let matched_characters = name
        .as_str()
        .to_pinyin()
        .flatten()
        .map(|v| v.plain())
        .filter(|v| matches!(*v, "gong" | "fu" | "guo" | "ji" | "yu" | "le"))
        .count();

    {
        // https://www.desmos.com/calculator/wmvf92efik

        let x = matched_characters as f32;
        let n = name.chars().count() as f32;

        if n > 0.0 {
            // .max(4.0) 是因為 spam 的名字去掉 hyphen 至少要有 4 個字元。
            1.0 - (1.0 - (x / n.max(4.0))).abs()
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
        if let Some(from) = message.from() {
            let chat_title = if DEBUG_MODE {
                message.text().unwrap_or("<?>").to_string()
            } else {
                from.full_name()
            };

            let possibility = get_gong_fu_possibility(&chat_title);
            log::debug!("possiblity of {}: {}", from.id, possibility);

            if possibility == 1.0 {
                log::info!("spam possibility = 1.0, kick out {}!", from.id);

                bot.send_message(
                    message.chat.id,
                    format!("{chat_title} 極有可能是廣告 - 決定踢出，可自動拉回。"),
                )
                .await?;
                bot.kick_chat_member(message.chat.id, from.id)
                    .await?;
            } else if possibility >= 0.8 {
                log::info!("spam possibility >= 0.8, send message to notify administrators.");
                bot.send_message(
                    message.chat.id,
                    format!("{chat_title} 可能是廣告，請管理員留意。"),
                )
                .await?;
            }
        }

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
        assert_eq!(super::get_gong_fu_possibility("珙冨-国际"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富-国际"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富-菓殛"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富-帼暨"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富國際"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("珙冨国际"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富国际"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富菓殛"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富帼暨"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富國際"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("珙-冨国际"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富国际"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富菓殛"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富帼暨"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富國_際"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("珙-冨国_际"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富国_际"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富菓_殛"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富帼_暨"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富-國際娛樂"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("珙冨-国际娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富-国际娛樂"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富-菓殛娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富-帼暨娛樂"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富國際娛樂"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("珙冨国际娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富国际娛樂"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富菓殛娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共富帼暨娛樂"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富國際娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("珙-冨国际娛樂"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富国际娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富菓殛娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富帼暨娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富國_際娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("珙-冨国_际娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富国_际娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富菓_殛娱乐"), 1.0);
        assert_eq!(super::get_gong_fu_possibility("共-富帼_暨娱乐"), 1.0);
    }

    #[test]
    fn test_get_gong_fu_possibility_0_percent() {
        assert_eq!(super::get_gong_fu_possibility("Hello, World!"), 0.0);
        assert_eq!(super::get_gong_fu_possibility("嗨"), 0.0);
    }
}
