// Third party
use kiss3d::conrod::widget_ids;

widget_ids! {
    /// Ids of the widgets used when rendering the top left of the screen
    pub struct InfoWidgetId {
        text_point_count,
        text_dimensionality,
        text_render_mode,
        text_explanation_mode,
        text_error,
    }
}

widget_ids! {
    /// Ids of the widgets used in the legends
    pub struct LegendWidgetId {
        canvas_color_legend,
        text_dim_0,
        color_block_0,
        text_dim_1,
        color_block_1,
        text_dim_2,
        color_block_2,
        text_dim_3,
        color_block_3,
        text_dim_4,
        color_block_4,
        text_dim_5,
        color_block_5,
        text_dim_6,
        color_block_6,
        text_dim_7,
        color_block_7,
        text_dim_other,
        color_block_other,
    }
}

widget_ids! {
    /// Ids of the widgets used in the menus
    pub struct MenuWidgetId {
        // Buttons that allow you to expand certain menus
        general_menu_toggle,
        explanation_menu_toggle,
        viewer_settings_menu_toggle,
        explanation_settings_menu_toggle,
        // Buttons controlling the viewer
        button_reset,
        button_dimension_switch,
        buttom_render_mode,
        // - size of the blobs / points
        text_size_slider,
        button_size_reset,
        slider_point_size,
        slider_blob_size,
        // - gamma slider
        text_gamma_slider,
        button_gamma_reset,
        slider_gamma,
        // - Calculate da silva / van driel button
        button_explanation_1,
        button_explanation_2,
        // - Recompute the current metric
        text_recompute,
        button_recompute,
        button_switch_neighborhood_type,
        slider_neighborhood,
        // - Color normalization focus
        text_color_normalization_bounds,
        slider_color_normalization,
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    #[derive(Clone)]
    pub struct LegacyWidgetId {
        text_point_count,
        text_dimensionality,
        text_render_mode,
        text_explanation_mode,
        text_error,
        // Buttons controlling the viewer
        button_reset,
        button_dimension_switch,
        buttom_render_mode,
        // Widgets used for the color legend
        canvas_color_legend,
        text_dim_0,
        color_block_0,
        text_dim_1,
        color_block_1,
        text_dim_2,
        color_block_2,
        text_dim_3,
        color_block_3,
        text_dim_4,
        color_block_4,
        text_dim_5,
        color_block_5,
        text_dim_6,
        color_block_6,
        text_dim_7,
        color_block_7,
        text_dim_other,
        color_block_other,
        // Settings panel for the current renderer.
        // - size of the blobs / points
        text_size_slider,
        button_size_reset,
        slider_point_size,
        slider_blob_size,
        // - gamma slider
        text_gamma_slider,
        button_gamma_reset,
        slider_gamma,
        // - Calculate da silva / van driel button
        button_explanation_1,
        button_explanation_2,
        // - Recompute the current metric
        text_recompute,
        button_recompute,
        button_switch_neighborhood_type,
        slider_neighborhood,
        // - Color normalization focus
        text_color_normalization_bounds,
        slider_color_normalization,
    }
}