use egui::epaint::Shadow;
use egui::{style, Vec2, Color32, Stroke, Rounding, Context, Margin, Frame};
use egui::TextStyle::*;
use egui::FontId;
use egui::FontFamily::Proportional;

pub fn app_style(ctx: &Context) -> egui::Style {
    let m = 20.0;

    let spacing = style::Spacing {
        item_spacing: Vec2 { x: 5.0, y: 5.0 },
        button_padding: Vec2 { x: 4.0, y: 2.0},
        text_edit_width: 80.0,
        window_margin: Margin { left: m, right: m, top: m, bottom: m },
        menu_margin: Margin { left: m, right: m, top: m, bottom: m },
        ..Default::default()
    };

    // Widget visuals (buttons / labels / etc)
    let widget_visuals = style::WidgetVisuals {
        bg_fill: Color32::from_rgb(61, 52, 77),
        weak_bg_fill: Color32::DARK_GRAY,
        bg_stroke: Stroke::default(),
        rounding: Rounding::from(2.0),
        fg_stroke: Stroke::new(1.0, Color32::GOLD),
        expansion: 0.0,
    };

    // Light Mode
    let mut text_visuals = style::WidgetVisuals {
        fg_stroke: Stroke::new(1.0, Color32::BLACK),
        ..widget_visuals
    };

    // Dark Mode
    if ctx.style().visuals.dark_mode {
        text_visuals = style::WidgetVisuals {
            fg_stroke: Stroke::new(1.0, Color32::WHITE),
            ..widget_visuals
        };
    }    

    let widgets = style::Widgets {
        noninteractive: text_visuals,
        inactive: widget_visuals,
        //hovered: widget_visuals,
        active: widget_visuals,
        open: widget_visuals,
        ..Default::default()
    };

    let visuals = style::Visuals {
        widgets: widgets,
        ..Default::default()
    };
    
    let style = style::Style {
        spacing: spacing,
        visuals: visuals,
        ..Default::default()
    };

    return style;
}

pub fn configure_buttons_style(ctx: &Context) -> egui::Style {

    let mut style = app_style(ctx);

    // Redefine text_styles
    style.text_styles = [
        (Heading, FontId::new(30.0, Proportional)),
        (Name("Heading2".into()), FontId::new(25.0, Proportional)),
        (Name("Context".into()), FontId::new(23.0, Proportional)),
        (Body, FontId::new(18.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ].into();
  

    return style;
}

pub fn toppanel_frame(ctx: &Context) -> Frame {
    let m = 10.0;
    let o = 0.0;

    let frame = Frame { 
        inner_margin: Margin { left: m, right: m, top: m, bottom: m }, 
        outer_margin: Margin { left: o, right: o, top: o, bottom: o }, 
        rounding: Rounding::default(), 
        shadow: Shadow::default(), 
        fill: ctx.style().visuals.panel_fill, 
        stroke: Stroke::default() 
    };

    return frame;
}