pub struct Values {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

pub struct ColorGrade {
    pub layer: String,
    pub components: [String; 4],
    pub function_name: String,    
    pub level_path: String,

    pub values: Values,
}

impl ColorGrade {
    pub fn create_sliders(&self) {
        

    }

}

impl Values {
    pub fn to_JSON() {

    }
}