use super::App;

impl App {
    pub(crate) fn set_host_terminal_context(
        &mut self,
        theme: crate::terminal_theme::TerminalTheme,
        appearance: Option<crate::terminal_theme::HostAppearance>,
        explicit: bool,
    ) -> bool {
        let theme_changed = self.state.host_terminal_theme != theme;
        let appearance_changed = self.state.host_terminal_appearance != appearance
            || self.state.host_terminal_appearance_explicit != explicit;
        if !theme_changed && !appearance_changed {
            return false;
        }
        self.state.host_terminal_theme = theme;
        self.state.host_terminal_appearance = appearance;
        self.state.host_terminal_appearance_explicit = explicit;
        for runtime in self.terminal_runtimes.values() {
            // Subscribers query OSC colors in response to the appearance report.
            if theme_changed {
                runtime.apply_host_terminal_theme(theme);
            }
            runtime.apply_host_terminal_appearance(appearance);
        }
        if theme_changed {
            self.render_dirty.request_generic();
            self.render_notify.notify_one();
        }
        let app_theme_changed = appearance_changed && self.refresh_effective_app_theme();
        theme_changed || app_theme_changed
    }

    #[cfg(test)]
    pub(crate) fn set_host_terminal_appearance_state(
        &mut self,
        appearance: Option<crate::terminal_theme::HostAppearance>,
        explicit: bool,
    ) -> bool {
        self.set_host_terminal_context(self.state.host_terminal_theme, appearance, explicit)
    }

    pub(super) fn refresh_effective_app_theme(&mut self) -> bool {
        let (palette, theme_name) = super::resolve_effective_theme(
            &self.state.theme_runtime,
            self.state.host_terminal_appearance,
        );
        if self.state.theme_name == theme_name && self.state.palette == palette {
            return false;
        }
        self.state.theme_name = theme_name;
        self.state.palette = palette;
        self.render_dirty.request_generic();
        self.render_notify.notify_one();
        true
    }
}
