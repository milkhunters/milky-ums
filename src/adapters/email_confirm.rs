use std::collections::BTreeMap;
use async_trait::async_trait;
use deadpool_redis::Pool;
use lapin::protocol::basic::AMQPProperties;
use lapin::types::{AMQPValue, FieldTable, ShortString};
use rand::Rng;
use redis::AsyncCommands;
use crate::application::common::email_confirm::EmailConfirm;


pub struct EmailConfirmAdapter {
    redis: Box<Pool>,
    rmq_connection: Box<lapin::Connection>,
    exchange: String,
    sender_id: String,
    service_text_id: String,
    code_expiration: u32,
}

impl EmailConfirmAdapter {
    pub fn new(
        redis: Box<Pool>,
        rmq_connection: Box<lapin::Connection>,
        exchange: String,
        sender_id: String,
        service_text_id: String,
        code_expiration: u32,
    ) -> Self {
        Self {
            redis,
            rmq_connection,
            exchange,
            sender_id,
            service_text_id,
            code_expiration,
        }
    }
}

#[async_trait]
impl EmailConfirm for EmailConfirmAdapter {

    /// **Confirm email** - метод подтверждения почты
    ///
    /// Пользователю дается 3 попытки на то, чтобы ввести правильный код подтверждения.
    /// Если пользователь превысил лимит попыток, то пользователь дожидается ttl и
    /// запрашивает новый код подтверждения.
    async fn confirm_email(&self, email: &str, code: u32) -> Result<bool, String> {
        let mut redis = self.redis.get().await.unwrap();

        let stored_data: String = match redis.get(email).await.unwrap() {
            Some(data) => data,
            None => return Err("Сначала запросите код подтверждения".to_string())
        };
        let (stored_code, attempts) = stored_data.split_once(':').unwrap();

        if attempts.parse::<u32>().unwrap() >= 3 {
            return Err("Превышено количество попыток".to_string());
        }

        if stored_code.parse::<u32>().unwrap() == code {
            let _: usize = redis.del(email).await.unwrap();
            Ok(true)
        } else {
            let attempts = attempts.parse::<u32>().unwrap() + 1;
            let data = format!("{}:{}", stored_code, attempts);

            let ttl: i64 = redis.ttl(email).await.unwrap();
            let _: String = redis.set(email, data).await.unwrap();
            if ttl > 0 {
                let _: i32 = redis.expire(email, ttl).await.unwrap();
            }

            Ok(false)
        }

    }

    /// **Send code** - метод отправки кода подтверждения на почту
    ///
    /// Генерируется шестизначный код и отправляется на указанную почту
    /// В редис записывается код и количество попыток
    async fn send_code(&self, email: &str) -> Result<(), String> {
        let mut redis = self.redis.get().await.unwrap();

        let data: Option<String> = redis.get(email).await.unwrap();
        if data.is_some() {
            return Err("Код уже отправлен".to_string());
        }

        let code: u32 = rand::thread_rng().gen_range(100000..=999999);
        let _: String = redis.set(email, format!("{}:0", code)).await.unwrap();
        let _: i32 = redis.expire(email, self.code_expiration as i64).await.unwrap();

        let channel = self.rmq_connection.create_channel().await.unwrap();

        channel.exchange_declare(
            &self.exchange,
            lapin::ExchangeKind::Direct,
            lapin::options::ExchangeDeclareOptions {
                durable: true,
                auto_delete: false,
                ..Default::default()
            },
            FieldTable::default()
        ).await.unwrap();

        let headers= FieldTable::from(
            BTreeMap::from([
                (ShortString::from("To"), AMQPValue::ShortString(ShortString::from(email))),
                (ShortString::from("Subject"), AMQPValue::ShortString(ShortString::from("Подтверждение почты"))),
                (ShortString::from("FromId"), AMQPValue::ShortString(ShortString::from(self.sender_id.clone()))),
            ]
            ));

        channel.basic_publish(
            &self.exchange,
            "",
            lapin::options::BasicPublishOptions::default(),
            format!("Ваш код подтверждения: {code}").as_bytes(),
            AMQPProperties::default()
                .with_headers(headers)
                .with_timestamp(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
                .with_message_id(ShortString::from(uuid::Uuid::new_v4().to_string()))
                .with_app_id(ShortString::from(self.service_text_id.clone()))
                .with_priority(0)
                .with_content_type(ShortString::from("text/plain"))
        ).await.unwrap();

        Ok(())
    }
}