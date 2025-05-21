use crate::data::current_timestamp;
use crate::database::{Database, User, UserColumn};
use crate::fetcher::{parse_tiktok_content, ContentType, Media};
use crate::parser::is_tiktok_url;
use reqwest::Url;
use std::cmp::PartialEq;
use std::sync::Arc;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::{SendMessageSetters, SendVideoSetters};
use teloxide::prelude::{Message, Requester};
use teloxide::types::{ChatId, InputFile, InputMedia, InputMediaPhoto, ParseMode};
use teloxide::Bot;

#[allow(dead_code)]
const TEXT_LIMIT: u16 = 4096;
const MEDIA_LIMIT: u16 = 1024;

pub async fn start(bot: Bot, database: Arc<Database>) {
    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let db = Arc::clone(&database);
        async move {
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

                update_user(&bot, user.id.0 as i64, chat_id, &user.first_name, user_lang, &db).await;

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

                send_msg(
                    &bot,
                    chat_id,
                    "<b>⏳ Downloading TikTok video...</b>\n\nPlease wait while we process your request! 🎬",
                    "<b>⏳ Скачивание видео из TikTok...</b>\n\nПожалуйста, подождите, пока мы обрабатываем ваш запрос! 🎬",
                    user_lang
                ).await;

                let media: Box<dyn Media> = match parse_tiktok_content(&text, user.id.0 as i64, &db).await {
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

                let media_title = cut_text(media.get_title(),MEDIA_LIMIT);

                if media.get_content_type() == ContentType::Video {
                    bot.send_video(chat_id,
                                   InputFile::url(
                                       Url::parse(media.get_content_urls()[0]
                                           .as_str())
                                           .unwrap()
                                   ))
                        .caption(media_title)
                        .await
                        .unwrap();
                } else if media.get_content_type() == ContentType::Photo {
                    let mut media_vec: Vec<InputMedia> = vec![];
                    let mut has_caption = false;
                    for photo in media.get_content_urls() {

                        let mut photo_media = InputMediaPhoto::new(
                            InputFile::url(
                                Url::parse(&photo).unwrap()
                            )
                        );

                        if !has_caption {
                            photo_media = photo_media.caption(&media_title);
                            has_caption = true;
                        }

                        media_vec.push(
                            InputMedia::Photo(
                                photo_media,
                            )
                        )

                    }

                    bot.send_media_group(chat_id,media_vec)
                        .await
                        .unwrap();

                }

                bot.send_audio(chat_id,
                               InputFile::url(
                                   Url::parse(&media.get_audio_url())
                                       .unwrap()
                               ))
                    .await
                    .unwrap();

            }

            Ok(())

        }
    }).await;

    async fn send_msg(bot: &Bot, chat_id: ChatId, english: &str, russian: &str, lang: &String) {

        let mut text = english;

        if lang.eq("ru") // russia
            || lang.eq("rs") // serbia
            || lang.eq("ua") // ukraine
            || lang.eq("by") // belarus
            || lang.eq("kz") // kazakhstan
            || lang.eq("md") // moldova
            || lang.eq("sk") // slovakia
            || lang.eq("si") // slovenia
            || lang.eq("lv") // latvia
            || lang.eq("ee") { // estonia

            text = russian;

        }

        bot.send_message(chat_id, text)
            .parse_mode(ParseMode::Html)
            .await
            .unwrap();
    }

    fn is_beyond_scope(text: &String, size: u16) -> bool {

        text.len()>size as usize

    }

    fn cut_text(text: String, size: u16) -> String {
        if is_beyond_scope(&text, size) {
            let slice_size = size as usize;
            text[0..slice_size].to_string()
        } else {
            text
        }
    }

    async fn update_user(
        bot: &Bot,
        id: i64,
        chat_id: ChatId,
        first_name: &String,
        user_lang: &String,
        db: &Database
    ) {
        if db.has_user(id).await.unwrap() {
            let data_user: User = db.get_user(id).await.unwrap().unwrap();

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

            if !data_user.name.eq(first_name) {

                db.set_data(id,UserColumn::Name,first_name)
                    .await
                    .unwrap();

            }

            let data_chat_id = match db.get_data(id, UserColumn::ChatId).await {
                Ok(chat_id) => chat_id,
                _ => {
                    None
                }
            };

            if data_chat_id.is_none() {

                db.set_data(id, UserColumn::ChatId, chat_id.0.to_string().as_str())
                    .await
                    .unwrap();

            }

        } else {
            let data_user = User {
                id,
                chat_id: Some(chat_id.0),
                name: first_name.to_string(),
                requests_amount: 1,
                timestamp: current_timestamp(),
                register_timestamp: current_timestamp(),
            };

           db.add_user(data_user).await.unwrap();

            // wellcome message
            send_msg(
                &bot,
                chat_id,
                "<b>🎉 Welcome aboard!</b>\n🎉 It looks like you're new here. Enjoy our bot for free - no limits! 😊",
                "<b>🎉 Добро пожаловать!</b>\n🎉 Похоже, вы здесь впервые. Пользуйтесь ботом бесплатно - без ограничений! 😊",
                user_lang
            ).await;

        }
    }
}