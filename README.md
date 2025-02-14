# VC (Validator Credentials)

Библиотека написанная на rust для быстрой валидации email/login/password

# Установка

Укажите библиотеку в `Cargo.toml`:
```toml
[dependencies]
vc = { git = "https://github.com/BigBrainsClub/VC" }
```

### **Пример использования**
```rust

use vc::is_valid_email;

fn main() -> std::io::Result<()> {
    let email = b"example@domain.com";
    if is_valid_email(email) {
        println!("is valid email :D");
    } else {
        println!("is invalid email :'|")
    }
    Ok(())
}
```

## Плюсы
1) Нет зависимостей
2) Скорость

## Минусы
1) Использует unsafe код

## Лицензия
Эта библиотека распространяется под лицензией BSD 2-Clause. См. [LICENSE](LICENSE) для деталей.

## Авторы и вклад
 - [@username](https://github.com/BigBrainsClub) - автор и разработчик
 - PR приветствуется! (Буду рад критике и улучшению данной библиотеки)