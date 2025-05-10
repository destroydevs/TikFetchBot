use std::thread;
use std::time::Duration;
use crate::data::{add_user, current_timestamp, get_user, has_user, User};
use crate::fetcher::{parse_tiktok_audio, parse_tiktok_video};
use crate::parser::is_tiktok_url;
use reqwest::Url;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::{SendMessageSetters, SendVideoSetters};
use teloxide::prelude::{Message, Requester};
use teloxide::types::{ChatId, InputFile, ParseMode};
use teloxide::Bot;

pub async fn start(bot: Bot) {
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {

        let text = match msg.text() {
            Some(text) => text,
            None => return Ok(()),
        };

        if !text.is_empty() && !msg.is_topic_message {

            let chat_id = match msg.chat_id() {
                Some(chat_id) => chat_id,
                None => return Ok(()),
            };

            let user = match &msg.from {
                Some(user) => user,
                None => return Ok(()),
            };

            let user_lang = match &user.language_code {
                Some(lang) => lang,
                None => &"en".to_string()
            };

            if !is_tiktok_url(&text) {
                send_msg(
                    &bot,
                    chat_id,
                    "<b>ℹ️ Current Bot Functions</b>\n\n\
    Currently, the bot only supports <b>TikTok video downloads</b>.\n\n\
    Please send a valid TikTok video link to proceed.\n\n\
    <i>Example: https://vm.tiktok.com/ABC123/</i> 🎥",

                    "<b>ℹ️ Доступные функции</b>\n\n\
    На данный момент бот поддерживает только <b>скачивание видео из TikTok</b>.\n\n\
    Отправьте рабочую ссылку на видео для продолжения.\n\n\
    <i>Пример: https://vm.tiktok.com/ABC123/</i> 🎬",
                    user_lang
                ).await;
                return Ok(());
            }

            if has_user(user.id.0).await.unwrap() {
                let data_user: User = get_user(user.id.0).await.unwrap().unwrap();

                // days after last bot use
                let time_after_last_use = (current_timestamp() - data_user.timestamp)/1000/60/60/24;

                if time_after_last_use >= 3 {
                    send_msg(
                        &bot,
                        chat_id,
                        "<b>🌟 Welcome back!</b>\n🌟 We missed you! 😊",
                        "<b>🌟 С возвращением!</b>\n🌟 Мы рады снова видеть вас! 😊",
                        user_lang
                    ).await;
                }


            } else {
                let data_user = User {
                    id: user.id.0,
                    name: user.first_name.clone(),
                    requests_amount: 1,
                    timestamp: current_timestamp(),
                    register_timestamp: current_timestamp(),
                };
                add_user(data_user).await.unwrap();

                // wellcome message
                send_msg(
                    &bot,
                    chat_id,
                    "<b>🎉 Welcome aboard!</b>\n🎉 It looks like you're new here. Enjoy our bot for free - no limits! 😊",
                    "<b>🎉 Добро пожаловать!</b>\n🎉 Похоже, вы здесь впервые. Пользуйтесь ботом бесплатно - без ограничений! 😊",
                    user_lang
                ).await;

            }

            send_msg(
                &bot,
                chat_id,
                "<b>⏳ Downloading TikTok video...</b>\n\nPlease wait while we process your request! 🎬",
                "<b>⏳ Скачивание видео из TikTok...</b>\n\nПожалуйста, подождите, пока мы обрабатываем ваш запрос! 🎬",
                user_lang
            ).await;

            let download_url = match parse_tiktok_video(&text, user.id.0).await {
                Ok(url) => url,
                Err(e) => {
                    send_msg(
                        &bot,
                        chat_id,
                        format!("<b>❌ Download failed!</b>\n\nReason: <code>{}</code>\n\nPlease try again later! 🎬",
                                e).as_str(),
                        format!("<b>❌ Скачивание не удалось!</b>\n\nПричина: <code>{}</code>\n\nПопробуйте позже! 🎬",
                                e).as_str(),
                        user_lang
                    ).await;
                    return Ok(())
                },
            };

            tokio::time::sleep(Duration::from_millis(100)).await;

            let audio_url = match parse_tiktok_audio(&text).await {
                Ok(url) => url,
                Err(e) => {
                    send_msg(
                        &bot,
                        chat_id,
                        format!("<b>❌ Download failed!</b>\n\nReason: <code>{}</code>\n\nPlease try again later! 🎬",
                                e).as_str(),
                        format!("<b>❌ Скачивание не удалось!</b>\n\nПричина: <code>{}</code>\n\nПопробуйте позже! 🎬",
                                e).as_str(),
                        user_lang
                    ).await;
                    return Ok(())
                }
            };

            bot.send_video(chat_id, InputFile::url(
                Url::parse(download_url.key()).unwrap())).caption(download_url.value())
                .await.unwrap();

            tokio::time::sleep(Duration::from_millis(100)).await;

            bot.send_audio(chat_id, InputFile::url(Url::parse(&audio_url).unwrap())).await.unwrap();

        }

        Ok(())

    }).await;

    async fn send_msg(bot: &Bot, chat_id: ChatId, english: &str, russian: &str, lang: &String) {
        let mut text = english;
        if lang.eq("ru") {
            text = russian;
        }

        bot.send_message(chat_id, text)
            .parse_mode(ParseMode::Html).await.unwrap();
    }
}