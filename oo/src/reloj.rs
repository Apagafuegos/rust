#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Reloj {
    horas: i32,
    minutos: i32,
    segundos: i32,
    formato_24_horas: bool,
}
impl Reloj {
    pub fn _get_horas(&self) -> i32 {
        self.horas
    }
    pub fn _get_minutos(&self) -> i32 {
        self.minutos
    }
    pub fn _get_segundos(&self) -> i32 {
        self.segundos
    }

    pub fn poner_en_hora(&mut self, horas: i32, minutos: i32, segundos: i32) {
        self.horas = horas;
        self.minutos = minutos;
        self.segundos = segundos;
    }

    pub fn new(horas: i32, minutos: i32, segundos: i32, formato_24_horas: bool) -> Reloj {
        Reloj {
            horas,
            minutos,
            segundos,
            formato_24_horas,
        }
    }
    pub fn to_string(&self) -> String {
        let mut _cadena = String::from("");
        if self.formato_24_horas && self.horas > 12 {
            _cadena = self.horas.to_string();
        } else {
            _cadena = (self.horas - 12).to_string();
        }

        if !self.formato_24_horas && (self.horas - 12) < 10 {
            _cadena = String::from("0") + &(self.horas - 12).to_string();
        } else if self.horas < 10 {
            _cadena = String::from("0") + &(self.horas).to_string();
        }

        format!("{}:{}:{}", _cadena, self.minutos, self.segundos)
    }
}
