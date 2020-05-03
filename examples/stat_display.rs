use ranger::{
    widget::{Widget, WidgetExt, Flex, Text, Block},
    lens::{lens_fn, lens_fn_ref},
    App,
};

struct Statblock {
    name: String,
    armor_class: u32,
    stats: Stats,
}

struct Stats {
    str_: i32,
    dex: i32,
    con: i32,
    int: i32,
    wis: i32,
    cha: i32,
}

fn main() {
    let data = Statblock{
        name: "Orc".to_string(),
        armor_class: 12,
        stats: Stats {
            str_: 16,
            dex: 12,
            con: 15,
            int: 8,
            wis: 11,
            cha: 10,
        },
    };
    let mut widget = build_widget();
    let mut app = App::new(std::io::stdout(), std::io::stdin());
    app.draw(&mut widget, &data).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10000));
}

fn build_widget() -> impl Widget<Statblock> {
    let mut inner = Flex::column();
    inner
        .add_child(
            Text::new().lens(lens_fn(|x: &Statblock| format!("AC {}", x.armor_class))).boxed(),
        )
        .add_child(
            build_stats_block().lens(lens_fn_ref(|x: &Statblock| &x.stats)).boxed()
        );
    Block::new(
        lens_fn_ref(|x: &Statblock| x.name.as_str()),
        inner
    )
}

fn build_stats_block() -> impl Widget<Stats> + 'static{
    Text::new().lens(lens_fn(|x: &Stats| {
        format!(
            "{:^4} {:^4} {:^4} {:^4} {:^4} {:^4}\n\
             {:^4} {:^4} {:^4} {:^4} {:^4} {:^4}\n\
             ({:+}) ({:+}) ({:+}) ({:+}) ({:+}) ({:+})",
            "STR", "DEX", "CON", "INT", "WIS", "CHA",
            x.str_, x.dex, x.con, x.int, x.wis, x.cha,
            modifier(x.str_), modifier(x.dex), modifier(x.con), modifier(x.int), modifier(x.wis), modifier(x.cha),
        )
    }))
}

fn modifier(x: i32) -> i32 {
    (x/2) - 5
}
