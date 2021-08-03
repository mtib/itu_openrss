fn main() {
    pretty_env_logger::init();

    let vac = itu_openrss::get_vacancies();

    println!("{:?}", vac);
}
