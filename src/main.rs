use std::{process::Command, thread, time::Duration};

fn main() {
    let interval = Duration::from_millis(100); // ajusta aqui (10ms)

    loop {
        // clique esquerdo (0xC0)
        let ok = Command::new("ydotool")
            .args(["click", "0xC0"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if !ok {
            eprintln!("falhou ao rodar ydotool. o ydotoold está rodando no outro terminal?");
        }

        thread::sleep(interval);
    }
}
