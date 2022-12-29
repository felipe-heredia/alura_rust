fn main() {
    let notas: [f32; 4] = [6.5; 4];
    let index: usize = 0;

    println!("{}", notas[index]);

    for index in 0..notas.len() {
        println!("A nota {} é {}", index + 1, notas[index])
    }

    matriz();
    is_weekend(WeekDay::Quinta);
    colors()
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [[0.0, 1.2, 0.1], [1.3, 0.3, 1.4]];

    for line in matriz {
        for item in line {
            println!("Item é {}", item)
        }
    }
}

#[allow(dead_code)]
enum WeekDay {
    Domingo,
    Segunda,
    Terça,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

fn is_weekend(weekday: WeekDay) {
    println!(
        "É fim de semana? {}",
        match weekday {
            WeekDay::Domingo | WeekDay::Sabado => true,
            _ => false,
        }
    )
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor {
        cyan: u8,
        magente: u8,
        yellow: u8,
        black: u8,
    },
}

fn colors() {
    let color = Color::CymkColor {
        cyan: 0,
        magente: 15,
        yellow: 18,
        black: 255,
    };

    println!(
        "A cor é {}",
        match color {
            Color::Red => "vermelho",
            Color::Green => "verde",
            Color::Blue => "azul",
            Color::RgbColor(0, 0, 0)
            | Color::CymkColor {
                cyan: _,
                magente: _,
                yellow: _,
                black: 255,
            } => "preto",
            Color::RgbColor(_, _, _) => "RGB desconhecido",
            Color::CymkColor {
                cyan: _,
                magente: _,
                yellow: _,
                black: _,
            } => "CYMK desconhecido",
        }
    )
}
