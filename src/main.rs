use std::rc::Rc;

sixtyfps::include_modules!();

fn main() {
    let field_models = Rc::new(sixtyfps::VecModel::<Field>::from(vec![]));

    let window = MainWindow::new();

    window.on_field_added({
        let field_models = field_models.clone();
        move |field| field_models.push(field)
    });

    window.on_run(|| {});

    window.set_field_models(sixtyfps::ModelHandle::new(field_models));

    window.run();
}
