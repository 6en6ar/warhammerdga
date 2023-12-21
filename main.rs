use lopdf::Document;


fn dga(f: String){
    let tld = [".com", ".io", ".xyz", ".zip"];
    
    // iterate through text, take every 6th letter
    let take_6: String = f
    .chars()
    .enumerate()
    .filter_map(|(index, c)| 
    if (index + 1) % 6 == 0 { 
        Some(c) 
    } else { 
        None 
    })
    .collect();

    // create domains of 15 in length
    let domains: Vec<String> = take_6
    .chars()
    .filter(|c| !c.is_whitespace() && c.is_ascii_lowercase())
    .collect::<Vec<char>>()
    .chunks(15) 
    .map(|chunk| chunk.iter().collect())
    .collect(); 

    println!("[ + ] Domains generated: ");
    for d in domains {
        for t in  tld{
            println!("{}", d.trim().to_string() + t);
        }

    }
}
fn main() {
    // read the rules, get the lenght of pages, take text from every second page
    // iterate through text and take every 6 letter into an array
    println!("[ + ] In the grim darkness of the far future, there is only war!");
    let file = "warhammer40k.pdf";
    let mut content = String::new();

    match Document::load(file) {
        Ok(document) => {
            let pages = document.get_pages();


            for (i, _) in pages.iter().enumerate() {
                let page_number = (i + 1) as u32;
                if page_number % 2 == 0{
                    let text = document.extract_text(&[page_number]);

                    content = content + text.unwrap().as_str();

                }

            }
        }
        Err(err) => println!("Error: {}", err),
    }
    let filtered = content.trim();
    dga(filtered.to_string());


}
