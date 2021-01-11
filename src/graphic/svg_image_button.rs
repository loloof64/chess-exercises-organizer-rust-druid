use druid::{theme, Affine,  Insets, LinearGradient, Size, UnitPoint};

use druid::widget::prelude::*;
use druid::widget::{Click, ControllerHost, SvgData};

use log::error;

pub struct SvgImageToggleButton {
    image_path_inactive: String,
    image_path_active: String,
}

impl SvgImageToggleButton {
    pub fn new(image_path_inactive: String, image_path_active: String) -> Self {
        Self {
            image_path_active,
            image_path_inactive,
        }
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut bool, &Env) + 'static,
    ) -> ControllerHost<Self, Click<bool>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl Widget<bool> for SvgImageToggleButton {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        _data: &mut bool,
        _env: &Env,
    ) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &bool,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &bool,
        _data: &bool,
        _env: &Env,
    ) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &bool,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &bool, env: &Env) {
        let size = ctx.size();
        let is_active = ctx.is_active();
        let is_hot = ctx.is_hot();
        let stroke_width = env.get(theme::BUTTON_BORDER_WIDTH);

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        let bg_gradient = if is_active {
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (env.get(theme::BUTTON_DARK), env.get(theme::BUTTON_LIGHT)),
            )
        } else {
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (env.get(theme::BUTTON_LIGHT), env.get(theme::BUTTON_DARK)),
            )
        };

        let border_color = if is_hot {
            env.get(theme::BORDER_LIGHT)
        } else {
            env.get(theme::BORDER_DARK)
        };

        ctx.stroke(rounded_rect, &border_color, stroke_width);

        ctx.fill(rounded_rect, &bg_gradient);

        let image_size = if size.width < size.height {
            size.width
        } else {
            size.height
        };
        let image_offset = if size.width < size.height {
            (0.0, image_size / 2.0)
        } else {
            (image_size / 2.0, 0.0)
        };
        let ratio = image_size / 45_f64;
        let affine_matrix = Affine::translate(image_offset) * Affine::scale(ratio);

        let image_to_use = if *data {
            &self.image_path_active
        } else {
            &self.image_path_inactive
        };
        let image_svg_data = match image_to_use.parse::<SvgData>() {
            Ok(svg) => svg,
            Err(err) => {
                error!("{}", err);
                error!("Using an empty SVG instead of {}.", image_to_use);
                SvgData::default()
            }
        };

        ctx.with_save(|ctx| {
            image_svg_data.to_piet(affine_matrix, ctx);
        });
    }
}
