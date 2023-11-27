pub trait Summary{
    fn summarise(&self) -> String;
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,   
}

impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("{} by {} ({})",self.headline, self.author,self.location)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarise(&self) -> String {
        format!("{}: {}",self.username,self.content)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{ // no se puede devolver una referencia sin saber su tiempo de vida
    if x.len() > y.len() {
        x
    }else{
        y   
    }
}

fn main() {

    let tuit = Tweet{
        username: String::from("cuenta en decadencia"),
        content: String::from("Cuenta en decadencia bombardeen Extremadura"),
        reply: false,
        retweet: true,
    };

    println!("Tweet nuevo de {}",tuit.summarise());

    let respuesta = longest("FERNANDO ALONSO DIAZ", "hola jeje");
    println!("{respuesta}")

    /*let r;
    {
        let x = 5;
        r = &x; incorrecto ya que x muere antes de que r sea pedido,
                por lo que &x no existe

    }
    println!("{r}");*/



}
