use nih_plug::prelude::*;
use std::sync::Arc;

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

#[derive(Debug, Default)]
struct MidSideSwapper {
    params: Arc<MidSideSwapperParams>,
}

#[derive(Debug, Default, Params)]
struct MidSideSwapperParams {}

impl Plugin for MidSideSwapper {
    const NAME: &'static str = "Mid Side Swapper";
    const VENDOR: &'static str = "kiwi";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "your@email.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        if buffer.channels() < 2 {
            return ProcessStatus::Normal;
        }

        let mut samples_iter = buffer.iter_samples();
        // SAFETY: We checked that the buffer has at least two channels
        let left_channel = samples_iter.next().unwrap();
        let right_channel = samples_iter.next().unwrap();

        for (left_sample, right_sample) in left_channel.into_iter().zip(right_channel.into_iter()) {
            let mut mid = (*left_sample + *right_sample) / 2.0;
            let mut side = (*left_sample - *right_sample) / 2.0;

            std::mem::swap(&mut mid, &mut side);

            *left_sample = mid + side;
            *right_sample = mid - side;
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for MidSideSwapper {
    const CLAP_ID: &'static str = "com.10kiwis.mid-side-swapper";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Swap mid (mono) and side (stereo) signals");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for MidSideSwapper {
    const VST3_CLASS_ID: [u8; 16] = *b"MIDSIDESWAPPER!!";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(MidSideSwapper);
nih_export_vst3!(MidSideSwapper);
