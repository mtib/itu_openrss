use tabled::{Alignment, Full, Modify, Table};

fn main() {
    pretty_env_logger::init();

    let vac = itu_openrss::get_vacancies().unwrap();

    println!(
        "{}",
        Table::new(vac)
            .with(
                Modify::new(Full)
                    .with(Alignment::left())
                    .with(Alignment::top())
            )
            .to_string()
    );
}
