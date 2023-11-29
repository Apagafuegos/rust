use printpdf::*;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

pub struct Config {
    //Ask in cmd for the word you wanna filter for, the path where the .txt is, the name of the .pdf and the size of the font
    pub query: String,
    pub file_path: String,
    pub file_name: String,
    pub font_size: f32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments given");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let file_name = args[3].clone();
        let font_size = args[4].clone().parse::<f32>().unwrap();

        Ok(Config {
            query,
            file_path,
            file_name,
            font_size,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn generar_pdf(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; //Reads the .txt
    let (doc, page1, layer1) = PdfDocument::new("Poema de cum", Mm(210.0), Mm(297.0), "Capa 1");
    //let (page2, layer2) = doc.add_page(Mm(10.0), Mm(250.0), "Page 2, Layer 1"); how to add a page
    let current_layer = doc.get_page(page1).get_layer(layer1); //Use of the first layer
    let font = doc
        .add_external_font(File::open("./fonts/cmunti.ttf").unwrap())
        .unwrap(); //Fixes the font
    let mut y = 290.0; //Starting y pos.
    for line in search(&config.query, &contents) {
        current_layer.use_text(line, config.font_size, Mm(5.0 /*x*/), Mm(y /*y*/), &font);
        y -= config.font_size / 2.0; //Adds the text in the .pdf
    }

    //x is hor. position starting from 0
    //y is the vertical position, here starting from 297 (as it's a DIN A4 size) going down to 0

    doc.save(&mut BufWriter::new(File::create(config.file_name).unwrap()))
        .unwrap();
    //Creates the doc.
    Ok(())
}
