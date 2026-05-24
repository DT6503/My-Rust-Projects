use std::rc::Rc;

struct ServerConfig {
    db_url: String,
}

struct AuthModule {
    config: Rc<ServerConfig>, // Разделяемое владение
}

struct PaymentModule {
    config: Rc<ServerConfig>, // Разделяемое владение
}

fn main() {
    // 1. Создай конфигурацию внутри Rc
    let shared_config = Rc::new(ServerConfig {
        db_url: String::from("postgres://localhost:5432"),
    });
    
    // 2. Создай AuthModule и передай ему КЛОН shared_config
    let auth = AuthModule {
        config: Rc::clone(&shared_config),
    };
    
    // 3. Создай PaymentModule и передай ему КЛОН shared_config
    let payment = PaymentModule {
        config: Rc::clone(&shared_config),
    };
    
    // Проверка: мы можем читать данные из модулей благодаря Deref-магии Rc
    println!("Auth DB: {}", auth.config.db_url);
    println!("Payment DB: {}", payment.config.db_url);
    
    // 4. Напечатай количество сильных ссылок на shared_config
    println!("Количество владельцев конфигурации: {}", Rc::strong_count(&shared_config));
    // Ожидаемый вывод: 3
}