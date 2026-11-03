//! Shared egui theme and widget toolkit for the WinAppMigrator GUIs.
//!
//! Both the main WinAppMigrator app and the self-extracting installer render
//! with the same polished, dark/light theme and widget set. Keeping them in one
//! crate means the two front ends stay visually identical without duplication.

pub mod hardening;
pub mod theme;
pub mod widgets;
pub mod window;

/// What a person is told of why a window could not open, whatever the error was.
pub const NEEDS_DIRECT3D_12_TEXT: &str = "This needs Direct3D 12 to draw its window, which an old \
     graphics driver or a virtual machine may not have.";

/// What every window of these programs opens with. They run elevated, and what elevation hands on
/// includes the environment of the account that started them, which any program of that account can
/// set. So nothing about how a window is drawn comes from there: see [`graphics_options`].
pub fn window_options(viewport: eframe::egui::ViewportBuilder) -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport,
        wgpu_options: graphics_options(),
        ..Default::default()
    }
}

/// How a window's renderer is set up, with nothing of it taken from the environment.
///
/// eframe's defaults go by `WGPU_DX12_AGILITY_SDK_PATH`, which has Direct3D load `D3D12Core.dll` from
/// any folder named there, past the restriction of DLLs to System32, along with the rest of
/// `WGPU_*`. The one exception is `WGPU_BACKEND`: it can only narrow what there is down to nothing,
/// since these programs have no renderer but Direct3D 12, and that is how a check sees what a person
/// is told when a window cannot open.
pub fn graphics_options() -> eframe::egui_wgpu::WgpuConfiguration {
    use eframe::egui_wgpu::{WgpuConfiguration, WgpuSetup, WgpuSetupCreateNew};
    use eframe::wgpu;

    let backends =
        wgpu::Backends::from_env().unwrap_or(wgpu::Backends::DX12) & wgpu::Backends::DX12;
    let chosen = WgpuSetupCreateNew {
        instance_descriptor: wgpu::InstanceDescriptor {
            backends,
            flags: wgpu::InstanceFlags::from_build_config(),
            backend_options: wgpu::BackendOptions::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            display: None,
        },
        power_preference: wgpu::PowerPreference::HighPerformance,
        ..WgpuSetupCreateNew::without_display_handle()
    };
    WgpuConfiguration {
        wgpu_setup: WgpuSetup::CreateNew(chosen),
        ..WgpuConfiguration::default()
    }
}

/// Which graphics adapter a window is drawn with, for the log. On a PC with no graphics card it is
/// Windows' own software one, which is slower and is why a screen may be.
pub fn graphics_adapter_of(creation: &eframe::CreationContext<'_>) -> String {
    creation.wgpu_render_state.as_ref().map_or_else(
        || "none".to_owned(),
        |state| {
            let info = state.adapter.get_info();
            format!("{}, {:?}, {:?}", info.name, info.device_type, info.backend)
        },
    )
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use eframe::egui_wgpu::WgpuSetup;
    use eframe::wgpu;

    use super::*;

    /// Everything the pinned wgpu and egui-wgpu read for an instance, each set to what would change
    /// what is drawn with, or load something.
    const WHAT_THE_ENVIRONMENT_COULD_SAY: [(&str, &str); 14] = [
        ("WGPU_DX12_AGILITY_SDK_PATH", r"C:\planted"),
        ("WGPU_DX12_AGILITY_SDK_VERSION", "616"),
        ("WGPU_DX12_AGILITY_SDK_REQUIRE", "1"),
        ("WGPU_DX12_COMPILER", "dxc"),
        ("WGPU_DX12_PRESENTATION_SYSTEM", "dcomp"),
        ("WGPU_DX12_USE_FRAME_LATENCY_WAITABLE_OBJECT", "none"),
        ("WGPU_POWER_PREF", "low"),
        ("WGPU_ADAPTER_NAME", "planted"),
        ("WGPU_DEBUG", "1"),
        ("WGPU_VALIDATION", "1"),
        ("WGPU_GPU_BASED_VALIDATION", "1"),
        ("WGPU_DISCARD_HAL_LABELS", "1"),
        ("WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER", "1"),
        ("WGPU_NOOP_BACKEND", "1"),
    ];
    const CHILD_MARKER: &str = "WAM_GRAPHICS_OPTIONS_CHILD";

    fn asked_of_a_child(extra: &[(&str, &str)]) {
        let mut child = Command::new(std::env::current_exe().unwrap_or_default());
        child
            .args([
                "--exact",
                "tests::the_child_looks_at_what_it_was_given",
                "--nocapture",
            ])
            .env(CHILD_MARKER, "1");
        for (name, value) in WHAT_THE_ENVIRONMENT_COULD_SAY.iter().chain(extra) {
            child.env(name, value);
        }
        let output = child.output().map(|output| {
            (
                output.status.success(),
                String::from_utf8_lossy(&output.stdout).into_owned(),
            )
        });
        let (passed, said) = output.unwrap_or_default();
        assert!(passed, "{said}");
        assert!(said.contains("1 passed"), "the child ran nothing: {said}");
    }

    #[test]
    fn what_a_window_is_drawn_with_is_not_up_to_the_environment() {
        asked_of_a_child(&[]);
    }

    #[test]
    fn the_environment_can_only_narrow_the_renderer_away() {
        asked_of_a_child(&[("WGPU_BACKEND", "vulkan")]);
    }

    #[test]
    fn the_child_looks_at_what_it_was_given() {
        if std::env::var_os(CHILD_MARKER).is_none() {
            return;
        }
        let WgpuSetup::CreateNew(setup) = graphics_options().wgpu_setup else {
            panic!("these make their own instance");
        };
        let backend = setup.instance_descriptor.backend_options;
        assert!(
            backend.dx12.agility_sdk.is_none(),
            "{:?}",
            backend.dx12.agility_sdk
        );
        assert_eq!(
            format!("{:?}", backend.dx12.shader_compiler),
            format!("{:?}", wgpu::BackendOptions::default().dx12.shader_compiler)
        );
        assert_eq!(
            format!("{:?}", backend.dx12.presentation_system),
            format!(
                "{:?}",
                wgpu::BackendOptions::default().dx12.presentation_system
            )
        );
        assert!(!backend.noop.enable);
        assert_eq!(
            setup.power_preference,
            wgpu::PowerPreference::HighPerformance
        );
        assert!(setup.native_adapter_selector.is_none());
        assert_eq!(
            setup.instance_descriptor.flags,
            wgpu::InstanceFlags::from_build_config()
        );
        let backends = setup.instance_descriptor.backends;
        assert!(wgpu::Backends::DX12.contains(backends), "{backends:?}");
        match std::env::var("WGPU_BACKEND").ok().as_deref() {
            Some("vulkan") => assert!(backends.is_empty(), "{backends:?}"),
            _ => assert_eq!(backends, wgpu::Backends::DX12),
        }
    }

    #[test]
    fn a_windows_options_come_from_here() {
        let options = window_options(eframe::egui::ViewportBuilder::default());
        let WgpuSetup::CreateNew(setup) = options.wgpu_options.wgpu_setup else {
            panic!("these make their own instance");
        };
        assert!(
            setup
                .instance_descriptor
                .backend_options
                .dx12
                .agility_sdk
                .is_none()
        );
    }
}
