use druid::{theme, Affine, LinearGradient, Size, UnitPoint};

use druid::widget::prelude::*;
use druid::widget::{SvgData};

use log::error;

pub struct SvgImageToggleButton {
    svg_image_size : Size,
    image_path_inactive: String,
    image_path_active: String,
}

impl SvgImageToggleButton {
    pub fn new(svg_image_size: Size, image_path_inactive: String, image_path_active: String) -> Self {
        Self {
            svg_image_size,
            image_path_active,
            image_path_inactive,
        }
    }
}

impl Widget<bool> for SvgImageToggleButton {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut bool,
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
                    *data = ! *data;
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
        let max_size = bc.max();
        let max_width = max_size.width;
        let max_height = max_size.height;

        let new_common_size = if max_width < max_height {
            max_width
        } else {
            max_height
        };
        bc.constrain(Size::new(new_common_size, new_common_size))
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

        let ratio = if size.width < size.height {
            size.width / self.svg_image_size.width
        } else {
            size.height / self.svg_image_size.height
        };

        let affine_matrix = Affine::scale(ratio);

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
