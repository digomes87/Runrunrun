mod async_runtime;
mod constants;
mod files;
mod random;
mod strings;
mod variables;
mod vectors;
mod vectors_structs;

#[tokio::main]
async fn main() {
    variables::variables();
    constants::constants();
    files::read_file().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    strings::formatting_strings();
    random::random_numbers();
    random::random_structs();
    random::generating_password();
    random::generate_password_user_defined();
    vectors::vector_of_integer();
    vectors::vector_sort();
    vectors_structs::new_person();

    let result = async_runtime::fetch_network_request().await;
    println!("Fetched: {result}");
}
