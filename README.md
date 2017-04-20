# TCP-Mailbox Beispiel

Dieses Beispiel implementiert eine kleine Mailbox, die über TCP - eine nach der anderen - Nachrichten annimmt und auf Anfrage wieder zurückgibt.

Das Protokoll ist simpel: alle Nachrichten werden gespeichert, ausser, wenn sie "READ" lauten. Dann wird die älteste Nachricht zurück an den Client gesendet.

## Verwendung

```sh
$ git clone git@github.com:skade/mailbox-konstanz.git
$ cargo build
$ cargo run
```

In einer zweiten Konsole:

```sh
$ echo "Eine Nachricht!" | nc 127.0.0.1 7200
$ echo "READ" | nc 127.0.0.1 7200
Eine Nachricht!
$ echo "READ" | nc 127.0.0.1 7200
No message in inbox!
```
