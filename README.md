# fht-iced

A custom [Iced](https://github.com/iced-rs) theme for `fht` projects

It is up to the application using this theme to load values to set the fields of `Theme`, then set
it as your application theme (either using [`iced::application`](https://docs.iced.rs/iced/fn.application.html)
, or whatever depending on your application).

This crate currently **DOES NOT** support all of Iced's widgets, I am implementing them gradually
whenever I need a new one for projects. Feel free to send a PR if you want something implemented!

## TO-DO

- [ ] Implement a "widget gallery" test to see how all the components look
- [ ] Finish implementing all iced widgets
