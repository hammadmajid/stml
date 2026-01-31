#[derive(Debug)]
pub struct Document {
    pub title: String,
    pub sections: Vec<Section>,
}

#[derive(Debug)]
pub struct Section {
    pub id: String,
    pub items: Vec<SectionItem>,
}

#[derive(Debug)]
pub enum SectionItem {
    Paragraph(String),
    Ref(Ref),
    Link(Link),
    Ol(Vec<String>),
    Li(Vec<String>),
    Image(Image),
}

#[derive(Debug)]
pub struct Ref {
    pub title: String,
    pub id: String,
}

#[derive(Debug)]
pub struct Link {
    pub to: String,
    pub text: String,
}

#[derive(Debug)]
pub struct Image {
    src: String,
    alt: String,
}
