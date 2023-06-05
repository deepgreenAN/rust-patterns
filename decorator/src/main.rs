fn main() {
    use decorator::buf_reader::BufReader;
    use decorator::reader::Read;
    use decorator::reader::StringReader;

    {
        let mut s_reader = StringReader::new("Hello World! I'm Rust Decorator Pattern".to_string());
        let mut buf = [' '; 10];

        let pos = s_reader.read(&mut buf);

        println!("buf: {:?}, pos: {:?}", buf, pos);
    }

    {
        let mut decorator_reader = BufReader::new(StringReader::new(
            "Hello World! I'm Rust Decorator Pattern".to_string(),
        ));
        let mut buf = [' '; 20];

        let pos = decorator_reader.read(&mut buf);

        println!("buf: {:?}, pos: {:?}", buf, pos);
    }
}
