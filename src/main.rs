use bobsics_gui::{
    widgets::{
        layouts::VerticalStack,
        Button, Label,
    },
    BobsicsGUIApp,
};

fn main() {
    let mut gui = BobsicsGUIApp::new("Bobsics GUI");
    gui.set_widget(
        VerticalStack::new()
            .with_padding(5.0.into())
            .add_child(
                Label::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam dignissim, lectus nec tincidunt porttitor, eros orci tincidunt nibh, eu lobortis erat dui vel lorem. Phasellus eu blandit tortor. In in dapibus ante. Nam tincidunt facilisis felis, at hendrerit eros lacinia vel. Curabitur purus magna, gravida eget nisi dignissim, eleifend sollicitudin enim. Vivamus feugiat facilisis turpis, eu hendrerit urna sagittis eu. Vestibulum a euismod lacus. Morbi nec odio ullamcorper, finibus nibh id, suscipit sapien. Quisque posuere maximus congue. Sed semper rhoncus eros a interdum. Suspendisse eleifend varius urna et bibendum.", 15.0).build(),
            )
            .add_child(Button::new("Click me!")
                .with_width(200.0)
                .with_margin((5.0, 20.0).into())
                .on_click(|_| println!("Hello world from the button!")).build())
            .build(),
    );
    gui.run();
}
