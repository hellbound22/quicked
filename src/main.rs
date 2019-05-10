use gtk::prelude::*;

use image::{self, imageops};

use std::cell::RefCell;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn main() {
    gtk::init().unwrap();

    let glade_src = "layout.glade";

    let builder = gtk::Builder::new_from_file(glade_src);

    let janela: gtk::Window = builder.get_object("janela").unwrap();
    let preview_window: Rc<RefCell<gtk::Window>> =
        Rc::new(RefCell::new(builder.get_object("preview_window").unwrap()));
    let close_button: gtk::Button = builder.get_object("close_preview").unwrap();

    let escolha1: gtk::FileChooser = builder.get_object("escolha1").unwrap();
    let arquivo1 = Rc::new(RefCell::new(String::new()));
    let escolha2: gtk::FileChooser = builder.get_object("escolha2").unwrap();
    let arquivo2 = Rc::new(RefCell::new(String::new()));
    let merge_button: gtk::Button = builder.get_object("merge").unwrap();
    let preview_button: gtk::Button = builder.get_object("preview").unwrap();

    let nome1: Rc<RefCell<gtk::Label>> =
        Rc::new(RefCell::new(builder.get_object("nome1").unwrap()));
    let nome2: Rc<RefCell<gtk::Label>> =
        Rc::new(RefCell::new(builder.get_object("nome2").unwrap()));
    let nomefinal: Rc<RefCell<gtk::Entry>> =
        Rc::new(RefCell::new(builder.get_object("nomefinal").unwrap()));
    let imagem_final: Rc<RefCell<gtk::Image>> =
        Rc::new(RefCell::new(builder.get_object("preview_image").unwrap()));

    janela.show_all();
    janela.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let u = (
        arquivo1.clone(),
        arquivo2.clone(),
        nome1.clone(),
        nome2.clone(),
        nomefinal.clone(),
        imagem_final.clone(),
        preview_window.clone(),
    );

    close_button.connect_clicked(clone! (u => move |_| {
        u.6.borrow().hide();
    }));

    escolha1.connect_update_preview(clone! (u => move |f| {
        u.0.borrow_mut().clear();
        u.0.borrow_mut().push_str(match f.get_preview_filename(){
                                    Some(b) => String::from(b.to_str().expect("erro 2")),
                                    None => String::from("/"),
        }.as_str());

        let p = String::from(u.0.borrow().clone());
        u.2.borrow().set_label(&p);
    }));

    escolha2.connect_update_preview(clone! (u => move |f| {
        u.1.borrow_mut().clear();
        u.1.borrow_mut().push_str(match f.get_preview_filename(){
                                    Some(b) => String::from(b.to_str().expect("erro 2")),
                                    None => String::from("/"),
        }.as_str());

        let p = String::from(u.1.borrow().clone());
        u.3.borrow().set_label(&p);
    }));

    preview_button.connect_clicked(clone! (u => move |_| {
        let final_path = u.4.borrow().get_text().unwrap();
        if !final_path.as_str().is_empty() {
            let mut img1 = image::open(Path::new(&u.0.borrow().clone())).unwrap();
            let mut img2 = image::open(Path::new(&u.1.borrow().clone())).unwrap();

            img2 = img2.resize_exact(img1.as_rgba8().unwrap().width(), img2.as_rgba8().unwrap().height(), image::FilterType::Nearest);

            let top = img1.as_rgba8().unwrap().height() - img2.as_rgba8().unwrap().height();
            println!("teste 2");

            imageops::overlay(&mut img1, &img2, 0, top);

            let mut tmp = File::create("tmp.png").unwrap();
            img1.resize(500, 400, image::FilterType::Nearest).write_to(&mut tmp, image::ImageOutputFormat::PNG).unwrap();
            println!("teste 3");

            u.5.borrow().set_from_file(Path::new("tmp.png"));
            std::fs::remove_file("tmp.png").unwrap();
            preview_window.borrow().show_all();
        }
    }));

    merge_button.connect_clicked(clone! (u => move |_| {
        let final_path = u.4.borrow().get_text().unwrap();
        if !final_path.as_str().is_empty() {
            let mut img1 = image::open(Path::new(&u.0.borrow().clone())).unwrap();
            let mut img2 = image::open(Path::new(&u.1.borrow().clone())).unwrap();

            img2 = img2.resize_exact(img1.as_rgba8().unwrap().width(), img2.as_rgba8().unwrap().height(), image::FilterType::Nearest);

            let top = img1.as_rgba8().unwrap().height() - img2.as_rgba8().unwrap().height();

            imageops::overlay(&mut img1, &img2, 0, top);

            let mut buffer = File::create(final_path.as_str()).unwrap();
            img1.write_to(&mut buffer, image::ImageOutputFormat::PNG).unwrap();
        }
   }));

    gtk::main();
}