fn main() {
    let notas: [f32; 4] = [6.5; 4];
    let index: usize = 0;

    println!("{}", notas[index]);

    for index in 0..notas.len() {
        println!("A nota {} é {}", index + 1, notas[index])
    }

    matriz();
    is_weekend(WeekDay::Quinta);
    colors();
    optinal_content();
    vectors();
    banking_account();
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

fn optinal_content() {
    let file_content = read_file(String::from(""));

    match &file_content {
        Some(value) => println!("{}", value),
        None => println!("Arquivo não existe"),
    };

    println!("{:?}", file_content);

    if let Some(_) = file_content {
        println!("Agora, tenho certeza de que há um valor!");
    }
}

fn read_file(_: String) -> Option<String> {
    Some(String::from("Conteúdo do arquivo"))
}

fn vectors() {
    let mut notas: Vec<f32> = vec![10.0, 8.8, 6.5, 9.0];
    println!("{:?}", notas);

    println!(
        "Nota 6 = {}",
        match notas.get(7) {
            Some(n) => *n,
            None => 0.0,
        }
    );

    if let Some(nota) = notas.pop() {
        println!("Último valor = {}", nota);
        println!("{:?}", notas);
    }
}

struct AccountOwner {
    name: String,
    last_name: String,
    birthday: String,
}

struct Account {
    owner: AccountOwner,
    balance: f64,
}

impl Account {
    fn cashout(&mut self, amount: f64) {
        self.balance -= amount
    }
}

fn banking_account() {
    let mut account: Account = Account {
        owner: AccountOwner {
            name: String::from("Felipe"),
            last_name: String::from("Heredia"),
            birthday: String::from("01/05/1986"),
        },
        balance: 100.0,
    };

    account.cashout(25.0);

    println!(
        "Dados da conta: Titular {} {}, Saldo {}, que nasceu em {}",
        account.owner.name, account.owner.last_name, account.balance, account.owner.birthday
    );
}
