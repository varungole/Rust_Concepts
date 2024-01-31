fn main()
{
    enum Language {
        English,
        Spanish,
    }

    let language = Language::Russian;
    
    match language {
        Language::English => println!("Hello, World"),
        Language::Spanish => println!("Hola, amigo"),
    }
}