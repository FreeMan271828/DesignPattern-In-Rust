pub trait Graphic{
    fn draw(&mut self);
}

pub struct Image{
    file_name: String
}
impl Image{
    fn new(name: Box<String>) -> Image{
        Self::load(name.clone());
        Image{ file_name: *name }
    }
    fn load(name: Box<String>){
        println!("{} is loading", name);
    }
}
impl Graphic for Image{
    fn draw(&mut self) {
        println!("{} is drawing", self.file_name);
    }
}

pub struct ImageProxy{
    file_name: String,
    image: Option<Image>
}
impl ImageProxy{
    pub fn new(name: Box<String>) -> ImageProxy{
        ImageProxy{
            file_name: *name.clone(),
            image: Option::from(
                Image::new(Box::new(*name))
            )
        }
    }
}
impl Graphic for ImageProxy{
    fn draw(&mut self) {
        if self.image.is_none(){
            self.image = Option::from(
                Image::new(Box::new(self.file_name.clone()))
            );
        }
        if let Some(image) = self.image.as_mut() {
            image.draw();
        }
    }
}