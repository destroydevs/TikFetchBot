//   Copyright 2025 Evgeny K.
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use crate::data::current_timestamp;
use crate::media_fetcher::{parse_tiktok_content, ContentType, Media};
use crate::parser::is_tiktok_url;
use reqwest::{Client, Url};
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

pub async fn start(bot: Bot, client: Arc<Client>) {
    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let client = Arc::clone(&client);
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

                let media: Box<dyn Media> = match parse_tiktok_content(&text, user.id.0 as i64, &client).await {
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
}