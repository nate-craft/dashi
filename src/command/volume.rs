use pulse::callbacks::ListResult;
use pulse::context::State as PaState;
use pulse::volume::{ChannelVolumes, Volume as PaVolume};
use pulse::{
    context::{Context, FlagSet},
    error::{Code, PAErr},
    mainloop::standard::{IterateResult, Mainloop},
};
use std::error::Error;
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::sync::{Arc, Mutex};

use crate::notify::notify;

use super::VolumeCommand;

pub struct VolumeSpec {
    silent: bool,
}

#[derive(Default, Clone, Copy)]
struct DashiVolume(i32);

struct PaFeedback<T>(Arc<Mutex<Option<Result<T, PAErr>>>>);

impl VolumeSpec {
    pub fn new(silent: bool) -> Self {
        VolumeSpec { silent }
    }

    pub fn run(&self, modifier: VolumeCommand) -> Result<(), Box<dyn Error>> {
        let mut pulse = Mainloop::new().ok_or(PAErr::from(Code::Access))?;
        let mut context = Context::new(&pulse, "dashi").ok_or(PAErr::from(Code::Access))?;

        context.connect(None, FlagSet::NOFLAGS, None)?;
        loop {
            match pulse.iterate(true) {
                IterateResult::Success(_) => {}
                IterateResult::Quit(_) => return Err(Box::new(PAErr::from(Code::Killed))),
                IterateResult::Err(err) => {
                    return Err(Box::new(err));
                }
            };

            match context.get_state() {
                PaState::Unconnected | PaState::Failed | PaState::Terminated => {
                    return Err(Box::new(PAErr::from(Code::Access)))
                }
                PaState::Ready => break,
                PaState::Connecting | PaState::Authorizing | PaState::SettingName => continue,
            }
        }

        match modifier {
            VolumeCommand::Add { n } => {
                let (mut channels, mut volume, _) = self.get_output(&mut pulse, &mut context)?;
                volume = volume + n.into();
                self.set_volume(&mut pulse, &mut context, &mut channels, volume)?;
                notify(self.silent, "Volume", format!("{}%", volume))?;
            }
            VolumeCommand::Sub { n } => {
                let (mut channels, mut volume, _) = self.get_output(&mut pulse, &mut context)?;
                volume = volume - n.into();
                self.set_volume(&mut pulse, &mut context, &mut channels, volume)?;
                notify(self.silent, "Volume", format!("{}%", volume))?;
            }
            VolumeCommand::Set { n } => {
                let (mut channels, _, _) = self.get_output(&mut pulse, &mut context)?;
                let volume = n.into();
                self.set_volume(&mut pulse, &mut context, &mut channels, volume)?;
                notify(self.silent, "Volume", format!("{}%", volume))?;
            }
            VolumeCommand::Get => {
                let (_, volume, _) = self.get_output(&mut pulse, &mut context)?;
                notify(self.silent, "Volume", format!("{}%", volume))?;
            }
            VolumeCommand::Muted => {
                let (_, volume, muted) = self.get_output(&mut pulse, &mut context)?;
                if muted {
                    notify(self.silent, "Volume", "Muted")?;
                } else {
                    notify(self.silent, "Volume", format!("{}%", volume))?;
                }
            }
            VolumeCommand::MutedMic => {
                if self.get_input(&mut pulse, &mut context)? {
                    notify(self.silent, "Microphone", "Disabled")?;
                } else {
                    notify(self.silent, "Volume", "Enabled")?;
                }
            }
            VolumeCommand::Mute => {
                let (_, volume, muted) = self.get_output(&mut pulse, &mut context)?;
                self.mute_output(&mut pulse, &mut context, !muted)?;

                if !muted {
                    notify(self.silent, "Volume", "Muted")?;
                } else {
                    notify(self.silent, "Volume", format!("{}%", volume))?;
                }
            }
            VolumeCommand::MuteMic => {
                let mic_muted = self.get_input(&mut pulse, &mut context)?;
                self.mute_input(&mut pulse, &mut context, !mic_muted)?;

                if !mic_muted {
                    notify(self.silent, "Microphone", "Disabled")?;
                } else {
                    notify(self.silent, "Microphone", "Enabled")?;
                }
            }
        }

        Ok(())
    }

    fn get_output(
        &self,
        main_loop: &mut Mainloop,
        context: &mut Context,
    ) -> Result<(ChannelVolumes, DashiVolume, bool), PAErr> {
        let cmd_get_volume = PaFeedback::<(ChannelVolumes, DashiVolume, bool)>::new();
        cmd_get_volume.run(main_loop, |result| {
            context
                .introspect()
                .get_sink_info_by_name("@DEFAULT_SINK@", move |info| {
                    let mut result = result.lock().unwrap();
                    match info {
                        ListResult::Item(info) => {
                            *result = Some(Ok((info.volume, info.volume.max().into(), info.mute)))
                        }
                        ListResult::Error => *result = Some(Err(PAErr::from(Code::Internal))),
                        ListResult::End => {}
                    };
                });
        })
    }

    fn get_input(&self, main_loop: &mut Mainloop, context: &mut Context) -> Result<bool, PAErr> {
        let cmd_get_volume = PaFeedback::<bool>::new();
        cmd_get_volume.run(main_loop, |result| {
            context
                .introspect()
                .get_source_info_by_name("@DEFAULT_SOURCE@", move |info| {
                    let mut result = result.lock().unwrap();
                    match info {
                        ListResult::Item(info) => *result = Some(Ok(info.mute)),
                        ListResult::Error => *result = Some(Err(PAErr::from(Code::Internal))),
                        ListResult::End => {}
                    };
                });
        })
    }

    fn set_volume(
        &self,
        main_loop: &mut Mainloop,
        context: &mut Context,
        channels: &mut ChannelVolumes,
        volume: DashiVolume,
    ) -> Result<(), PAErr> {
        volume.set(channels);
        let cmd_get_volume = PaFeedback::<()>::new();
        cmd_get_volume.run(main_loop, |result| {
            context.introspect().set_sink_volume_by_name(
                "@DEFAULT_SINK@",
                channels,
                Some(Box::new(move |success| {
                    let mut result = result.lock().unwrap();
                    match success {
                        true => *result = Some(Ok(())),
                        false => *result = Some(Err(PAErr::from(Code::Internal))),
                    }
                })),
            );
        })
    }

    fn mute_output(
        &self,
        main_loop: &mut Mainloop,
        context: &mut Context,
        muted: bool,
    ) -> Result<(), PAErr> {
        let cmd_get_volume = PaFeedback::<()>::new();
        cmd_get_volume.run(main_loop, |result| {
            context.introspect().set_sink_mute_by_name(
                "@DEFAULT_SINK@",
                muted,
                Some(Box::new(move |success| {
                    let mut result = result.lock().unwrap();
                    match success {
                        true => *result = Some(Ok(())),
                        false => *result = Some(Err(PAErr::from(Code::Internal))),
                    }
                })),
            );
        })
    }

    fn mute_input(
        &self,
        main_loop: &mut Mainloop,
        context: &mut Context,
        muted: bool,
    ) -> Result<(), PAErr> {
        let cmd_get_volume = PaFeedback::<()>::new();
        cmd_get_volume.run(main_loop, |result| {
            context.introspect().set_source_mute_by_name(
                "@DEFAULT_SOURCE@",
                muted,
                Some(Box::new(move |success| {
                    let mut result = result.lock().unwrap();
                    match success {
                        true => *result = Some(Ok(())),
                        false => *result = Some(Err(PAErr::from(Code::Internal))),
                    }
                })),
            );
        })
    }
}

impl<T> PaFeedback<T> {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }

    fn run<F: FnOnce(Arc<Mutex<Option<Result<T, PAErr>>>>)>(
        self,
        main_loop: &mut Mainloop,
        action: F,
    ) -> Result<T, PAErr> {
        action(self.0.clone());

        loop {
            if let Some(result) = self.0.lock().unwrap().take() {
                return result;
            }

            match main_loop.iterate(true) {
                IterateResult::Success(_) => continue,
                IterateResult::Quit(_) => return Err(PAErr::from(Code::Killed)),
                IterateResult::Err(err) => return Err(err),
            };
        }
    }
}

impl DashiVolume {
    fn set(&self, channels: &mut ChannelVolumes) {
        channels
            .get_mut()
            .iter_mut()
            .for_each(|channel| *channel = self.into());
    }

    fn clamp(self) -> DashiVolume {
        DashiVolume(self.0.max(0).min(150))
    }
}

impl Add for DashiVolume {
    type Output = DashiVolume;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0).clamp()
    }
}

impl Sub for DashiVolume {
    type Output = DashiVolume;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0).clamp()
    }
}

impl From<u32> for DashiVolume {
    fn from(value: u32) -> Self {
        Self(value as i32)
    }
}

impl From<PaVolume> for DashiVolume {
    fn from(volume: PaVolume) -> Self {
        let range = PaVolume::NORMAL.0 as f32 - PaVolume::MUTED.0 as f32;
        let unrounded_percent =
            ((volume.0 as f32 - PaVolume::MUTED.0 as f32) * 100.0 / range) as i32;
        DashiVolume((unrounded_percent + 2) / 5 * 5 as i32).clamp()
    }
}

impl Into<PaVolume> for &DashiVolume {
    fn into(self) -> PaVolume {
        let range = PaVolume::NORMAL.0 as f32 - PaVolume::MUTED.0 as f32;
        PaVolume((PaVolume::MUTED.0 as f32 + self.0 as f32 * range / 100.0) as u32)
    }
}

impl Display for DashiVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
