/*
 * Copyright 2026 Jiajia Liu
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fmt::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;
use std::process::Command;

use gtk::glib::{self, Variant, VariantDict, VariantTy};
use gtk::{Orientation, Align};
use gtk::gdk::{RGBA, Display};

use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use adw::{ToolbarView, HeaderBar, Clamp};
use adw::{PreferencesGroup, StyleManager, ColorScheme};
use gtk::{TextView, TextBuffer, ScrolledWindow, PolicyType, WrapMode};
use gtk::{Grid, ColorDialogButton, ColorDialog, Button, ToggleButton};
use gtk::{FileDialog, CssProvider};

const STYLE_NIGHT: KgxLivery = KgxLivery {
    color: [
        RGBA::new(0.651, 0.680, 0.631, 1.0),
        RGBA::new(0.0, 0.0, 0.0, 1.0),

        RGBA::new(0.537, 0.863, 0.922, 1.0),
        RGBA::new(0.000, 0.212, 0.212, 1.0)
    ],

    palette: [
        RGBA::new(0.141, 0.122, 0.192, 1.0),
        RGBA::new(0.753, 0.110, 0.157, 1.0),
        RGBA::new(0.180, 0.761, 0.494, 1.0),
        RGBA::new(0.961, 0.761, 0.067, 1.0),
        RGBA::new(0.118, 0.471, 0.894, 1.0),
        RGBA::new(0.596, 0.255, 0.733, 1.0),
        RGBA::new(0.039, 0.725, 0.863, 1.0),
        RGBA::new(0.753, 0.749, 0.737, 1.0),
        RGBA::new(0.369, 0.361, 0.392, 1.0),
        RGBA::new(0.929, 0.200, 0.231, 1.0),
        RGBA::new(0.341, 0.890, 0.537, 1.0),
        RGBA::new(0.973, 0.894, 0.361, 1.0),
        RGBA::new(0.318, 0.631, 1.000, 1.0),
        RGBA::new(0.753, 0.380, 0.796, 1.0),
        RGBA::new(0.310, 0.824, 0.992, 1.0),
        RGBA::new(0.965, 0.961, 0.957, 1.0)
    ]
};

const STYLE_DAY: KgxLivery = KgxLivery {
    color: [
        RGBA::new(0.0, 0.0, 0.0, 1.0),
        RGBA::new(0.921, 0.929, 0.921, 1.0),

        RGBA::new(0.298, 0.310, 0.412, 1.0),
        RGBA::new(0.922, 0.925, 0.922, 1.0)
    ],

    palette: [
        RGBA::new(0.141, 0.122, 0.192, 1.0),
        RGBA::new(0.753, 0.110, 0.157, 1.0),
        RGBA::new(0.180, 0.761, 0.494, 1.0),
        RGBA::new(0.961, 0.761, 0.067, 1.0),
        RGBA::new(0.118, 0.471, 0.894, 1.0),
        RGBA::new(0.596, 0.255, 0.733, 1.0),
        RGBA::new(0.039, 0.725, 0.863, 1.0),
        RGBA::new(0.753, 0.749, 0.737, 1.0),
        RGBA::new(0.369, 0.361, 0.392, 1.0),
        RGBA::new(0.929, 0.200, 0.231, 1.0),
        RGBA::new(0.341, 0.890, 0.537, 1.0),
        RGBA::new(0.973, 0.894, 0.361, 1.0),
        RGBA::new(0.318, 0.631, 1.000, 1.0),
        RGBA::new(0.753, 0.380, 0.796, 1.0),
        RGBA::new(0.310, 0.824, 0.992, 1.0),
        RGBA::new(0.965, 0.961, 0.957, 1.0)
    ]
};

struct KgxLivery {
    color: [RGBA; 4],
    palette: [RGBA; 16],
}

struct Livery {
    selector: [KgxLivery; 2],
    is_light: bool,

    button: Vec<ColorDialogButton>,
    palette_button: Vec<ColorDialogButton>,
    sync_button: bool,

    css_provider: Vec<CssProvider>,
}

fn main() -> glib::ExitCode {
    let livery = Rc::new(RefCell::new(Livery {
        selector: [ STYLE_NIGHT, STYLE_DAY ],
        is_light: false,
        button: Vec::with_capacity(4),
        palette_button: Vec::with_capacity(16),
        sync_button: false,
        css_provider: Vec::with_capacity(4),
    }));

    let application = Application::builder()
        .application_id("cc.call.KgxLivery")
        .build();

    application.connect_activate(glib::clone!(#[weak] livery, move |app| {
        for _ in 0..4 {
            let provider = CssProvider::new();

            gtk::style_context_add_provider_for_display(
                &Display::default().unwrap(),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

            livery.borrow_mut().css_provider.push(provider);
        }

        let style_manager = StyleManager::default();

        if style_manager.is_dark() {
            style_manager.set_color_scheme(ColorScheme::ForceDark);
        } else {
            livery.borrow_mut().is_light = true;
            style_manager.set_color_scheme(ColorScheme::ForceLight);
        }

        let vbox = gtk::Box::new(Orientation::Vertical, 6);

        let clamp = Clamp::builder().child(&vbox).maximum_size(1280).build();

        let preview = PreferencesGroup::builder()
            .title("Preview")
            .margin_start(6)
            .margin_end(6)
            .build();
        let text_buffer = TextBuffer::builder()
            .text("It was the best of times, it was the worst of times.")
            .build();
        let text_view = TextView::builder()
            .buffer(&text_buffer)
            .wrap_mode(WrapMode::Char)
            .monospace(true).vexpand(true).build();
        text_view.add_css_class("preview_fg");
        text_view.add_css_class("preview_bg");

        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .child(&text_view)
            .build();
        preview.add(&scrolled);

        let idx: usize = livery.borrow().is_light as usize;

        let text_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .build();

        let text_pref: [PreferencesGroup; 2] = [
            PreferencesGroup::builder()
                .title("Text")
                .margin_start(6)
                .margin_end(3)
                .hexpand(true)
                .build(),
            PreferencesGroup::builder()
                .title("Header")
                .margin_start(3)
                .margin_end(6)
                .hexpand(true)
                .build(),
        ];

        for i in 0..2 {
            let grid = Grid::builder()
                .column_homogeneous(true)
                .column_spacing(6)
                .row_spacing(6)
                .build();

            for j in 0..2 {
                let color_button = ColorDialogButton::builder()
                    .hexpand(false)
                    .rgba(&livery.borrow().selector[idx].color[i * 2 + j])
                    .dialog(&ColorDialog::new())
                    .build();

                color_button.connect_rgba_notify(glib::clone!(#[weak] livery, move |button| {
                    if !livery.borrow().sync_button {
                        let cur = livery.borrow().is_light as usize;
                        let id = i * 2 + j;
                        livery.borrow_mut().selector[cur].color[id] = button.rgba();
                        update_css(&livery.borrow().css_provider[id], id, &button.rgba());
                    }
                }));

                grid.attach(&color_button, j as i32, 0, 1, 1);

                livery.borrow_mut().button.push(color_button);
            }

            let place_holder = gtk::Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(0)
                .hexpand(true)
                .build();

            grid.attach(&place_holder, 2, 0, 2, 1);

            text_pref[i].add(&grid);
            text_box.append(&text_pref[i]);
        }

        let palette = PreferencesGroup::builder()
            .title("Palette")
            .margin_start(6)
            .margin_end(6)
            .build();

        let palette_grid = Grid::builder()
            .column_homogeneous(true)
            .column_spacing(6)
            .row_spacing(6)
            .hexpand(true)
            .build();

        palette.add(&palette_grid);

        for i in 0..16 {
            let color_button = ColorDialogButton::builder()
                .rgba(&livery.borrow().selector[idx].palette[i])
                .dialog(&ColorDialog::new()).build();

            color_button.connect_rgba_notify(glib::clone!(#[weak] livery, move |button| {
                if !livery.borrow().sync_button {
                    let idx = livery.borrow().is_light as usize;
                    livery.borrow_mut().selector[idx].palette[i] = button.rgba();
                }
            }));

            palette_grid.attach(&color_button, (i % 8) as i32, (i / 8) as i32, 1, 1);
            livery.borrow_mut().palette_button.push(color_button);
        }

        let apply_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::End)
            .spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(24)
            .margin_end(24)
            .build();

        let default = Button::with_label("Default");
        let reset = Button::with_label("Reset");
        let load = Button::with_label("Load");
        let save = Button::with_label("Save");
        let apply = Button::with_label("Apply");

        apply.connect_clicked(glib::clone!(#[weak] livery, move |_| {
            if let Ok(text) = livery_to_string(&*livery.borrow()) {
                match Command::new("gsettings")
                    .arg("set")
                    .arg("org.gnome.Console")
                    .arg("custom-liveries")
                    .arg(&text)
                    .status() {
                    Ok(status) => if !status.success() { return },
                    Err(_) => return,
                }

                match Command::new("gsettings")
                    .arg("set")
                    .arg("org.gnome.Console")
                    .arg("livery")
                    .arg("9f1374fd-f199-429f-bae6-9cf1260f6e3e")
                    .status() {
                    Ok(status) => if !status.success() { return },
                    Err(_) => return,
                }

                match Command::new("gsettings")
                    .arg("set")
                    .arg("org.gnome.Console")
                    .arg("livery")
                    .arg("a03c03a0-c0d1-11f1-8ce0-c42360f189fc")
                    .status() {
                    Ok(status) => if !status.success() { return },
                    Err(_) => return,
                }
            }
        }));

        apply_box.append(&default);
        apply_box.append(&reset);
        apply_box.append(&load);
        apply_box.append(&save);
        apply_box.append(&apply);

        vbox.append(&preview);
        vbox.append(&text_box);
        vbox.append(&palette);
        vbox.append(&apply_box);

        let view = ToolbarView::builder().content(&clamp).build();
        let header_bar = HeaderBar::new();
        view.add_top_bar(&header_bar);

        let style_button = ToggleButton::new();
        if !style_manager.is_dark() {
            style_button.set_active(false);
            style_button.set_icon_name("weather-clear-symbolic");
        } else {
            style_button.set_active(true);
            style_button.set_icon_name("weather-clear-night-symbolic");
        }

        style_button.connect_active_notify(glib::clone!(#[weak] livery, move |button| {
            if button.is_active() {
                button.set_icon_name("weather-clear-night-symbolic");
                style_manager.set_color_scheme(ColorScheme::ForceDark);
                livery.borrow_mut().is_light = false;
            } else {
                button.set_icon_name("weather-clear-symbolic");
                style_manager.set_color_scheme(ColorScheme::ForceLight);
                livery.borrow_mut().is_light = true;
            }

            livery_sync(&livery);
        }));

        header_bar.pack_start(&style_button);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Console Livery")
            .default_width(600)
            .default_height(700)
            .content(&view)
            .build();

        window.add_css_class("header_fg");
        window.add_css_class("header_bg");

        save.connect_clicked(glib::clone!(#[weak] window, #[weak] livery, move |_| {
            let file_dialog = FileDialog::new();
            file_dialog.save(Some(&window), None::<&gtk::gio::Cancellable>, move |ret| {
                if let Ok(file) = ret {
                    if let Some(path) = file.path() {
                        if let Ok(text) = livery_to_string(&*livery.borrow()) {
                            let _ = glib::file_set_contents(&path, text.as_bytes());
                        }
                    }
                }
            });
        }));

        load.connect_clicked(glib::clone!(#[weak] window, #[weak] livery,
                move |_| {
            let file_dialog = FileDialog::new();
            file_dialog.open(Some(&window), None::<&gtk::gio::Cancellable>, glib::clone!(move |ret| {
                if let Ok(file) = ret {
                    if let Some(path) = file.path() {
                        if let Ok(data) = glib::file_get_contents(path) {
                            if let Ok(text) = std::str::from_utf8(&data) {
                                if let Ok(variant) = Variant::parse(Some(&VariantTy::new("a{sv}").unwrap()), text) {
                                    variant_to_livery(&mut *livery.borrow_mut(), &variant);
                                    livery_sync(&livery);
                                } else {
                                    println!("failed to parse");
                                }
                            } else {
                                    println!("invalid utf8 text");
                            }
                        } else {
                            println!("failed to get contents");
                        }
                    }
                }
            }));
        }));

        reset.connect_clicked(glib::clone!(#[weak] livery, move |_| {
            livery.borrow_mut().selector = [ STYLE_NIGHT, STYLE_DAY ];
            livery_sync(&livery);
        }));

        default.connect_clicked(|_| {
            match Command::new("gsettings")
                .arg("set")
                .arg("org.gnome.Console")
                .arg("livery")
                .arg("9f1374fd-f199-429f-bae6-9cf1260f6e3e")
                .status() {
                Ok(status) => if !status.success() { return },
                Err(_) => return,
            }
        });

        livery_sync(&livery);
        window.present();
    }));

    application.run()
}

fn rgba_fmt(color: &RGBA) -> impl fmt::Display {
    fmt::from_fn(move |f| {
        write!(f, "({:.6}, {:.6}, {:.6})", color.red(), color.green(), color.blue())
    })
}

fn livery_to_string(livery: &Livery) -> Result<String, std::fmt::Error> {
    let mut text = String::with_capacity(2048);

    write!(&mut text, "{{ ")?;
    write!(&mut text, "'a03c03a0-c0d1-11f1-8ce0-c42360f189fc': <{{ ")?;
    write!(&mut text, "'uuid': <'a03c03a0-c0d1-11f1-8ce0-c42360f189fc'>, ")?;
    write!(&mut text, "'name': <'custom-livery'>, ")?;

    write!(&mut text, "'night': <{{ ")?;
    write!(&mut text, "'foreground': <{}>, ", rgba_fmt(&livery.selector[0].color[0]))?;
    write!(&mut text, "'background': <{}>, ", rgba_fmt(&livery.selector[0].color[1]))?;
    write!(&mut text, "'headerbar-fg': <{}>, ", rgba_fmt(&livery.selector[0].color[2]))?;
    write!(&mut text, "'headerbar-bg': <{}>, ", rgba_fmt(&livery.selector[0].color[3]))?;

    write!(&mut text, "'colours': <[ ")?;

    for i in 0..15 {
        write!(&mut text, "{}, ", rgba_fmt(&livery.selector[0].palette[i]))?;
    }
    write!(&mut text, "{} ", rgba_fmt(&livery.selector[0].palette[15]))?;

    write!(&mut text, "]> ")?;

    write!(&mut text, "}}>, ")?;

    write!(&mut text, "'day': <{{ ")?;
    write!(&mut text, "'foreground': <{}>, ", rgba_fmt(&livery.selector[1].color[0]))?;
    write!(&mut text, "'background': <{}>, ", rgba_fmt(&livery.selector[1].color[1]))?;
    write!(&mut text, "'headerbar-fg': <{}>, ", rgba_fmt(&livery.selector[1].color[2]))?;
    write!(&mut text, "'headerbar-bg': <{}>, ", rgba_fmt(&livery.selector[1].color[3]))?;

    write!(&mut text, "'colours': <[ ")?;

    for i in 0..15 {
        write!(&mut text, "{}, ", rgba_fmt(&livery.selector[1].palette[i]))?;
    }
    write!(&mut text, "{} ", rgba_fmt(&livery.selector[1].palette[15]))?;

    write!(&mut text, "]> ")?;

    write!(&mut text, "}}> ")?;

    write!(&mut text, "}}> ")?;
    write!(&mut text, "}}")?;

    Ok(text)
}

fn update_css(provider: &CssProvider, id: usize, color: &RGBA) {
    match id {
        0 => {
            let css = format!(".preview_fg {{ color: {}; }}", color);
            provider.load_from_string(&css);
        }
        1 => {
            let css = format!(".preview_bg {{ background-color: {}; }}", color);
            provider.load_from_string(&css);
        }
        2 => {
            let css = format!(".header_fg {{ color: {}; }}", color);
            provider.load_from_string(&css);
        },
        3 => {
            let css = format!(".header_bg {{ background-color: {}; }}", color);
            provider.load_from_string(&css);
        }
        _ => (),
    }
}

fn variant_to_kgx_livery(dict: &VariantDict, is_light: bool) -> Option<KgxLivery> {
    let mut kgx_livery = if is_light { STYLE_DAY } else { STYLE_NIGHT };

    let mut count: usize = 0;

    if let Ok(Some((r, g, b))) = dict.lookup::<(f64, f64, f64)>("foreground") {
        kgx_livery.color[0] = RGBA::new(r as f32, g as f32, b as f32, 1.0);
        count += 1;
        println!("foreground ({}, {}, {})", r, g, b);
    }

    if let Ok(Some((r, g, b))) = dict.lookup::<(f64, f64, f64)>("background") {
        kgx_livery.color[1] = RGBA::new(r as f32, g as f32, b as f32, 1.0);
        count += 1;
        println!("background ({}, {}, {})", r, g, b);
    }

    if let Ok(Some((r, g, b))) = dict.lookup::<(f64, f64, f64)>("headerbar-fg") {
        kgx_livery.color[2] = RGBA::new(r as f32, g as f32, b as f32, 1.0);
        println!("headerbar-fg ({}, {}, {})", r, g, b);
    }

    if let Ok(Some((r, g, b))) = dict.lookup::<(f64, f64, f64)>("headerbar-bg") {
        kgx_livery.color[3] = RGBA::new(r as f32, g as f32, b as f32, 1.0);
        println!("headerbar-bg ({}, {}, {})", r, g, b);
    }

    if let Ok(Some(palette)) = dict.lookup::<Vec<(f64, f64, f64)>>("colours") {
        if palette.len() == 16 {
            for i in 0..16 {
                kgx_livery.palette[i] =
                    RGBA::new(palette[i].0 as f32, palette[i].1 as f32, palette[i].2 as f32, 1.0);
                println!("palette {} ({}, {}, {})", i, palette[i].0, palette[i].1, palette[i].2);
            }

            count += 16;
        }
    }

    if count == 18 { Some(kgx_livery) } else { None }
}

fn variant_to_livery(livery: &mut Livery, custom: &Variant) {
    for child in custom.iter() {
         if let Some(_uuid) = child.child_value(0).get::<String>() {
            if let Some(value) = child.child_value(1).as_variant() {

                let dict = VariantDict::new(Some(&value));

                if let Ok(Some(value)) = dict.lookup::<VariantDict>("night") {
                    if let Some(night) = variant_to_kgx_livery(&value, false) {
                        livery.selector[0] = night;
                        println!("found night theme")
                    }
                }

                if let Ok(Some(value)) = dict.lookup::<VariantDict>("day") {
                    if let Some(day) = variant_to_kgx_livery(&value, true) {
                        livery.selector[1] = day;
                        println!("found day theme")
                    }
                }
            }

            break;
         }
    }
}

fn livery_sync(livery: &Rc<RefCell<Livery>>) {
    let idx = livery.borrow().is_light as usize;

    livery.borrow_mut().sync_button = true;

    {
        let console = &*livery.borrow();
        let kgx_livery = &console.selector[idx];

        for i in 0..4 {
            console.button[i].set_rgba(&kgx_livery.color[i]);
            update_css(&console.css_provider[i], i, &kgx_livery.color[i]);
        }

        for i in 0..16 {
            console.palette_button[i].set_rgba(&kgx_livery.palette[i]);
        }
    }

    livery.borrow_mut().sync_button = false;
}
