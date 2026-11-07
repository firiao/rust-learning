# Лаба 17. Умные указатели

**Время:** ~20 минут
**Тема:** `Box` в рекурсивном типе, `Rc`, `RefCell`, `Rc<RefCell<_>>`

## Цель

Собрать рекурсивный список на `Box` и разделяемый журнал на `Rc<RefCell<_>>`.

## Крейт уже создан

Открывай `practice/l17-smart-pointers` — типы и `new_log` уже написаны, `todo!()` в шести местах.

```bash
cargo test -p l17-smart-pointers   # 3 красных
```

## Реализуй

**1. `List::from_slice(values)`** — построить список, сохраняя порядок. Подсказка: `rev().fold(Nil, …)`.

**2. `List::len`**, **3. `List::is_empty`**, **4. `List::sum`** — рекурсивно по вариантам через `match`.

**5. `log_message(log, message)`** — дописать строку. Нужен `borrow_mut()`.

**6. `log_len(log)`**, **7. `log_snapshot(log)`** — прочитать. Нужен `borrow()`.

## Эксперимент

В `log_message` добавь строку, которая держит заимствование:

```rust
pub fn log_message(log: &SharedLog, message: &str) {
    log.borrow();
    log.borrow_mut().push(message.to_string());
}
```

Запусти тест. Запиши в `NOTES.md`, что произошло и почему компилятор это не поймал.

## Критерии приёмки

- [ ] 3 теста зелёные;
- [ ] `Rc::strong_count` в тесте `log_is_shared` равен 2 — значит, `Rc` действительно разделяет владение;
- [ ] `clippy -D warnings` и `fmt --check` чистые после реализации;
- [ ] в `NOTES.md` разобран эксперимент с двойным `borrow`.

## Подсказки

<details>
<summary>`List`</summary>

```rust
pub fn from_slice(values: &[i32]) -> Self {
    values
        .iter()
        .rev()
        .fold(List::Nil, |acc, &v| List::Cons(v, Box::new(acc)))
}

pub fn len(&self) -> usize {
    match self {
        List::Cons(_, tail) => 1 + tail.len(),
        List::Nil => 0,
    }
}

pub fn is_empty(&self) -> bool {
    matches!(self, List::Nil)
}

pub fn sum(&self) -> i32 {
    match self {
        List::Cons(head, tail) => head + tail.sum(),
        List::Nil => 0,
    }
}
```

`rev()` — потому что `fold` идёт слева направо и иначе наращивал бы список с конца.

</details>

<details>
<summary>Журнал</summary>

```rust
pub fn log_message(log: &SharedLog, message: &str) {
    log.borrow_mut().push(message.to_string());
}

pub fn log_len(log: &SharedLog) -> usize {
    log.borrow().len()
}

pub fn log_snapshot(log: &SharedLog) -> Vec<String> {
    log.borrow().clone()
}
```

`borrow_mut()` возвращает умный указатель, который отпускается в конце выражения — поэтому
в одну строку безопасно.

</details>

<details>
<summary>Эксперимент</summary>

```text
thread 'main' panicked at: already borrowed: BorrowMutError
```

`RefCell` проверяет правила **в рантайме**. Первый `borrow()` живёт до конца блока — и `borrow_mut()`
видит активное неизменяемое заимствование.

Компилятор не поймал, потому что заимствования спрятаны за методами: для него это обычные вызовы.

Вывод: `RefCell` меняет цену ошибки — вместо ошибки компиляции получаешь панику в рантайме.

</details>

---

**Готово?** Дальше — [лекция 18: потоки](../18-threads/lecture.md).
