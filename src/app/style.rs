use egui::{style, Vec2, Color32, Stroke, Rounding};

pub fn app_style() -> egui::Style {
    let spacing = style::Spacing {
        item_spacing: Vec2 { x: 5.0, y: 5.0 },
        button_padding: Vec2 { x: 4.0, y: 2.0},
        text_edit_width: 80.0,
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

    let text_visuals = style::WidgetVisuals {
        fg_stroke: Stroke::new(1.0, Color32::WHITE),
        ..widget_visuals
    };

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