use printpdf::*;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

pub struct Config {
    //Ask in cmd for the path where the .txt is, the name of the .pdf and the size of the font
    // E.G. : (in src folder) hola.txt penis.pdf (gotta put the .pdf else it wont generate one) 10
    pub file_path: String,
    pub file_name: String,
    pub font_size: f32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments given");
        }
        let file_path = args[1].clone();
        let file_name = args[2].clone();
        let font_size = args[3].clone().parse::<f32>().unwrap();

        if font_size < 10.0 {
            panic!(
                "ERROR: The font size should be 10 or bigger, the program got {}",
                font_size
            );
        }

        Ok(Config {
            file_path,
            file_name,
            font_size,
        })
    }
}

pub fn generate_pdf(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; //Reads the .txt
    let (doc, page1, layer1) = PdfDocument::new(&config.file_name, Mm(210.0), Mm(297.0), "Capa 1");
    // 297 x 210 DIN A4 size
    //2.1mm per char at size 10, prox 4.2mm per char at size 20 (calculations)
    //size_per_char  = 2.1*(size/10) --> size/10 is the relation between the def. font and the selected one

    let current_layer = doc.get_page(page1).get_layer(layer1); //Use of the first layer
    let font = doc
        .add_external_font(File::open("./fonts/cmunti.ttf").unwrap())
        .unwrap(); //Fixes the font

    let y = 280.0; //Starting y pos.
    current_layer.begin_text_section();
    current_layer.set_font(&font, config.font_size);
    current_layer.set_text_cursor(Mm(10.5), Mm(y));
    current_layer.set_line_height(config.font_size);

    margin_generator(&current_layer, contents, config.font_size, font);

    current_layer.end_text_section();
    //x is hor. position starting from 0
    //y is the vertical position, here starting from 280 (as it's a DIN A4 size) going down to 0

    doc.save(&mut BufWriter::new(File::create(config.file_name).unwrap()))
        .unwrap();
    //Creates the doc.
    Ok(())
}

fn margin_generator(
    current_layer: &PdfLayerReference,
    contents: String,
    font_size: f32,
    font: IndirectFontRef,
) {
    let mut row_size = 0;

    for word in contents.split_whitespace() {
        //Adds the text in the .pdf
        let limit = (font_size) as i32 / 10;
        let word_size = (word.len() + 1) as i32; //Length of the word plus the space
        if (row_size + word_size) > 85 / limit {
            //85 is the standard limit for size 10, caution for font values less than 10
            current_layer.write_text(word, &font);
            current_layer.add_line_break();
            row_size = 0;
        } else if word.contains(".") {
            current_layer.write_text(word, &font);
            current_layer.add_line_break(); //If contains a fullstop it will automatically break line (not sure bout leaving this)
            row_size = 0;
        } else {
            row_size += word_size;
            current_layer.write_text(word, &font);
            current_layer.write_text(" ", &font);
        }
    }
}
