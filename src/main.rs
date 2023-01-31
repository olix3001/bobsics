use bobsics_gui::{
    widgets::{
        layouts::{HorizontalStack, VerticalStack},
        Label,
    },
    BobsicsGUIApp,
};
use bobsics_render::Color;

fn main() {
    let mut gui = BobsicsGUIApp::new("Bobsics GUI");
    let vertical_stack = VerticalStack::new()
        .with_padding(5.0.into())
        .with_spacing(1.0.into())
        .add_child(
            Label::new("Hello, world 1!", 20.0)
                .with_color(Color::from_hex(0xff0000))
                .build(),
        )
        .add_child(
            Label::new("Hello, world 1!", 15.0)
                .with_color(Color::from_hex(0x00ff00))
                .build(),
        )
        .add_child(
            Label::new("Hello, world 1!", 10.0)
                .with_color(Color::from_hex(0x0000ff))
                .build(),
        )
        .build();
    let vertical_stack_2 = VerticalStack::new()
        .with_padding(5.0.into())
        .with_spacing(1.0.into())
        .add_child(
            Label::new("Hello, world 2!", 30.0)
                .with_color(Color::from_hex(0xffff00))
                .build(),
        )
        .add_child(
            Label::new("Hello, world 2!", 10.0)
                .with_color(Color::from_hex(0x00ff22))
                .build(),
        )
        .add_child(
            Label::new("Hello, world 2!", 20.0)
                .with_color(Color::from_hex(0x2020ff))
                .build(),
        )
        .build();
    gui.set_widget(
        HorizontalStack::new()
            .with_padding(5.0.into())
            .with_spacing(10.0.into())
            .add_child(vertical_stack)
            .add_child(vertical_stack_2)
            .build(),
    );
    gui.run();
}
