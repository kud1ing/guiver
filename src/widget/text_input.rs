use crate::widget::{Text, WidgetCommand, WidgetError};
use crate::{SizeConstraints, SystemEvent, Widget, WidgetEvent, WidgetId};
use druid_shell::kurbo::{Point, Rect, RoundedRect, Size};
use druid_shell::piet::{Color, Error, PaintBrush, Piet, RenderContext};
use druid_shell::{KbKey, Region};
use std::borrow::BorrowMut;
use std::cmp::min;

/// A text input widget.
pub struct TextInput {
    corner_radius: f64,
    debug_rendering: bool,
    debug_rendering_stroke_brush: PaintBrush,
    debug_rendering_stroke_width: f64,
    fill_brush: Option<PaintBrush>,
    has_focus: bool,
    is_disabled: bool,
    is_hidden: bool,
    padding: f64,
    rectangle: Rect,
    size_constraints: SizeConstraints,
    stroke_brush: PaintBrush,
    stroke_brush_focused: PaintBrush,
    stroke_width: f64,
    text: String,
    text_widget: Text,
    widget_id: WidgetId,
    width: f64,
}

impl TextInput {
    ///
    pub fn new(
        widget_id: WidgetId,
        debug_rendering_stroke_brush: PaintBrush,
        debug_rendering_stroke_width: f64,
        text: impl Into<String>,
        width: f64,
        frame_color: Color,
        frame_color_focused: Color,
    ) -> Self {
        let child_widget_id = 0;
        let text = text.into();

        TextInput {
            corner_radius: 2.0,
            debug_rendering: false,
            debug_rendering_stroke_brush: debug_rendering_stroke_brush.clone(),
            debug_rendering_stroke_width: debug_rendering_stroke_width.clone(),
            fill_brush: None,
            has_focus: false,
            is_disabled: true,
            is_hidden: false,
            padding: 4.0,
            rectangle: Rect::default(),
            size_constraints: SizeConstraints::unbounded(),
            stroke_brush: PaintBrush::Color(frame_color),
            stroke_brush_focused: PaintBrush::Color(frame_color_focused),
            stroke_width: 1.0,
            text: text.clone(),
            text_widget: Text::new(
                child_widget_id,
                debug_rendering_stroke_brush,
                debug_rendering_stroke_width,
                text,
            ),
            widget_id,
            width,
        }
    }

    ///
    fn layout_child(&mut self) {
        let padding_size = Size::new(2.0 * self.padding, 2.0 * self.padding);

        // Apply the child widget's size constraints.
        let child_size = self
            .text_widget
            .borrow_mut()
            .apply_size_constraints(self.size_constraints.shrink(padding_size));

        // Make the text input at least as wide as its width.
        self.rectangle = self.rectangle.with_size(Size::new(
            self.width.max(child_size.width + padding_size.width),
            child_size.height + padding_size.height,
        ));

        // Set the child's origin.
        self.text_widget.borrow_mut().set_origin(
            self.rectangle.origin()
                + (
                    0.5 * (self.rectangle.size().width - child_size.width).max(0.0),
                    0.5 * (self.rectangle.size().height - child_size.height).max(0.0),
                ),
        );
    }
}

impl Widget for TextInput {
    fn apply_size_constraints(&mut self, size_constraints: SizeConstraints) -> Size {
        self.size_constraints = size_constraints;

        // Layout the child.
        self.layout_child();

        self.rectangle.size()
    }

    fn handle_command(&mut self, widget_command: WidgetCommand) -> Result<(), WidgetError> {
        match widget_command {
            WidgetCommand::AppendChild(_) => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::RemoveAllChildren => {
                return Err(WidgetError::CommandNotHandled(
                    self.widget_id,
                    widget_command,
                ));
            }
            WidgetCommand::RemoveChild(widget_id) => {
                return Err(WidgetError::NoSuchWidget(widget_id));
            }
            WidgetCommand::SetDebugRendering(debug_rendering) => {
                self.debug_rendering = debug_rendering;
            }
            WidgetCommand::SetHasFocus(has_focus) => {
                self.has_focus = has_focus;
            }
            WidgetCommand::SetIsDisabled(is_disabled) => {
                self.is_disabled = is_disabled;
            }
            WidgetCommand::SetIsHidden(is_hidden) => {
                self.is_hidden = is_hidden;
            }
            WidgetCommand::SetValue(_) => {
                return self.text_widget.handle_command(widget_command);
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, system_event: &SystemEvent, widget_events: &mut Vec<WidgetEvent>) {
        match system_event {
            SystemEvent::KeyDown(key_event) => match &key_event.key {
                KbKey::Character(string) => {
                    // Modify the text.
                    self.text.push_str(&string);

                    // Tell the text widget about the new text.
                    self.text_widget
                        .borrow_mut()
                        .handle_command(WidgetCommand::SetValue(Box::new(self.text.clone())))
                        .unwrap();

                    self.layout_child();

                    // Inform the world that out text has changed.
                    widget_events.push(WidgetEvent::ValueChanged(
                        self.widget_id,
                        Box::new(self.text.clone()),
                    ));
                }
                KbKey::Unidentified => {}
                KbKey::Alt => {}
                KbKey::AltGraph => {}
                KbKey::CapsLock => {}
                KbKey::Control => {}
                KbKey::Fn => {}
                KbKey::FnLock => {}
                KbKey::Meta => {}
                KbKey::NumLock => {}
                KbKey::ScrollLock => {}
                KbKey::Shift => {}
                KbKey::Symbol => {}
                KbKey::SymbolLock => {}
                KbKey::Hyper => {}
                KbKey::Super => {}
                KbKey::Enter => {}
                KbKey::Tab => {}
                KbKey::ArrowDown => {}
                KbKey::ArrowLeft => {}
                KbKey::ArrowRight => {}
                KbKey::ArrowUp => {}
                KbKey::End => {}
                KbKey::Home => {}
                KbKey::PageDown => {}
                KbKey::PageUp => {}
                KbKey::Backspace => {}
                KbKey::Clear => {}
                KbKey::Copy => {}
                KbKey::CrSel => {}
                KbKey::Cut => {}
                KbKey::Delete => {}
                KbKey::EraseEof => {}
                KbKey::ExSel => {}
                KbKey::Insert => {}
                KbKey::Paste => {}
                KbKey::Redo => {}
                KbKey::Undo => {}
                KbKey::Accept => {}
                KbKey::Again => {}
                KbKey::Attn => {}
                KbKey::Cancel => {}
                KbKey::ContextMenu => {}
                KbKey::Escape => {}
                KbKey::Execute => {}
                KbKey::Find => {}
                KbKey::Help => {}
                KbKey::Pause => {}
                KbKey::Play => {}
                KbKey::Props => {}
                KbKey::Select => {}
                KbKey::ZoomIn => {}
                KbKey::ZoomOut => {}
                KbKey::BrightnessDown => {}
                KbKey::BrightnessUp => {}
                KbKey::Eject => {}
                KbKey::LogOff => {}
                KbKey::Power => {}
                KbKey::PowerOff => {}
                KbKey::PrintScreen => {}
                KbKey::Hibernate => {}
                KbKey::Standby => {}
                KbKey::WakeUp => {}
                KbKey::AllCandidates => {}
                KbKey::Alphanumeric => {}
                KbKey::CodeInput => {}
                KbKey::Compose => {}
                KbKey::Convert => {}
                KbKey::Dead => {}
                KbKey::FinalMode => {}
                KbKey::GroupFirst => {}
                KbKey::GroupLast => {}
                KbKey::GroupNext => {}
                KbKey::GroupPrevious => {}
                KbKey::ModeChange => {}
                KbKey::NextCandidate => {}
                KbKey::NonConvert => {}
                KbKey::PreviousCandidate => {}
                KbKey::Process => {}
                KbKey::SingleCandidate => {}
                KbKey::HangulMode => {}
                KbKey::HanjaMode => {}
                KbKey::JunjaMode => {}
                KbKey::Eisu => {}
                KbKey::Hankaku => {}
                KbKey::Hiragana => {}
                KbKey::HiraganaKatakana => {}
                KbKey::KanaMode => {}
                KbKey::KanjiMode => {}
                KbKey::Katakana => {}
                KbKey::Romaji => {}
                KbKey::Zenkaku => {}
                KbKey::ZenkakuHankaku => {}
                KbKey::F1 => {}
                KbKey::F2 => {}
                KbKey::F3 => {}
                KbKey::F4 => {}
                KbKey::F5 => {}
                KbKey::F6 => {}
                KbKey::F7 => {}
                KbKey::F8 => {}
                KbKey::F9 => {}
                KbKey::F10 => {}
                KbKey::F11 => {}
                KbKey::F12 => {}
                KbKey::Soft1 => {}
                KbKey::Soft2 => {}
                KbKey::Soft3 => {}
                KbKey::Soft4 => {}
                KbKey::ChannelDown => {}
                KbKey::ChannelUp => {}
                KbKey::Close => {}
                KbKey::MailForward => {}
                KbKey::MailReply => {}
                KbKey::MailSend => {}
                KbKey::MediaClose => {}
                KbKey::MediaFastForward => {}
                KbKey::MediaPause => {}
                KbKey::MediaPlay => {}
                KbKey::MediaPlayPause => {}
                KbKey::MediaRecord => {}
                KbKey::MediaRewind => {}
                KbKey::MediaStop => {}
                KbKey::MediaTrackNext => {}
                KbKey::MediaTrackPrevious => {}
                KbKey::New => {}
                KbKey::Open => {}
                KbKey::Print => {}
                KbKey::Save => {}
                KbKey::SpellCheck => {}
                KbKey::Key11 => {}
                KbKey::Key12 => {}
                KbKey::AudioBalanceLeft => {}
                KbKey::AudioBalanceRight => {}
                KbKey::AudioBassBoostDown => {}
                KbKey::AudioBassBoostToggle => {}
                KbKey::AudioBassBoostUp => {}
                KbKey::AudioFaderFront => {}
                KbKey::AudioFaderRear => {}
                KbKey::AudioSurroundModeNext => {}
                KbKey::AudioTrebleDown => {}
                KbKey::AudioTrebleUp => {}
                KbKey::AudioVolumeDown => {}
                KbKey::AudioVolumeUp => {}
                KbKey::AudioVolumeMute => {}
                KbKey::MicrophoneToggle => {}
                KbKey::MicrophoneVolumeDown => {}
                KbKey::MicrophoneVolumeUp => {}
                KbKey::MicrophoneVolumeMute => {}
                KbKey::SpeechCorrectionList => {}
                KbKey::SpeechInputToggle => {}
                KbKey::LaunchApplication1 => {}
                KbKey::LaunchApplication2 => {}
                KbKey::LaunchCalendar => {}
                KbKey::LaunchContacts => {}
                KbKey::LaunchMail => {}
                KbKey::LaunchMediaPlayer => {}
                KbKey::LaunchMusicPlayer => {}
                KbKey::LaunchPhone => {}
                KbKey::LaunchScreenSaver => {}
                KbKey::LaunchSpreadsheet => {}
                KbKey::LaunchWebBrowser => {}
                KbKey::LaunchWebCam => {}
                KbKey::LaunchWordProcessor => {}
                KbKey::BrowserBack => {}
                KbKey::BrowserFavorites => {}
                KbKey::BrowserForward => {}
                KbKey::BrowserHome => {}
                KbKey::BrowserRefresh => {}
                KbKey::BrowserSearch => {}
                KbKey::BrowserStop => {}
                KbKey::AppSwitch => {}
                KbKey::Call => {}
                KbKey::Camera => {}
                KbKey::CameraFocus => {}
                KbKey::EndCall => {}
                KbKey::GoBack => {}
                KbKey::GoHome => {}
                KbKey::HeadsetHook => {}
                KbKey::LastNumberRedial => {}
                KbKey::Notification => {}
                KbKey::MannerMode => {}
                KbKey::VoiceDial => {}
                KbKey::TV => {}
                KbKey::TV3DMode => {}
                KbKey::TVAntennaCable => {}
                KbKey::TVAudioDescription => {}
                KbKey::TVAudioDescriptionMixDown => {}
                KbKey::TVAudioDescriptionMixUp => {}
                KbKey::TVContentsMenu => {}
                KbKey::TVDataService => {}
                KbKey::TVInput => {}
                KbKey::TVInputComponent1 => {}
                KbKey::TVInputComponent2 => {}
                KbKey::TVInputComposite1 => {}
                KbKey::TVInputComposite2 => {}
                KbKey::TVInputHDMI1 => {}
                KbKey::TVInputHDMI2 => {}
                KbKey::TVInputHDMI3 => {}
                KbKey::TVInputHDMI4 => {}
                KbKey::TVInputVGA1 => {}
                KbKey::TVMediaContext => {}
                KbKey::TVNetwork => {}
                KbKey::TVNumberEntry => {}
                KbKey::TVPower => {}
                KbKey::TVRadioService => {}
                KbKey::TVSatellite => {}
                KbKey::TVSatelliteBS => {}
                KbKey::TVSatelliteCS => {}
                KbKey::TVSatelliteToggle => {}
                KbKey::TVTerrestrialAnalog => {}
                KbKey::TVTerrestrialDigital => {}
                KbKey::TVTimer => {}
                KbKey::AVRInput => {}
                KbKey::AVRPower => {}
                KbKey::ColorF0Red => {}
                KbKey::ColorF1Green => {}
                KbKey::ColorF2Yellow => {}
                KbKey::ColorF3Blue => {}
                KbKey::ColorF4Grey => {}
                KbKey::ColorF5Brown => {}
                KbKey::ClosedCaptionToggle => {}
                KbKey::Dimmer => {}
                KbKey::DisplaySwap => {}
                KbKey::DVR => {}
                KbKey::Exit => {}
                KbKey::FavoriteClear0 => {}
                KbKey::FavoriteClear1 => {}
                KbKey::FavoriteClear2 => {}
                KbKey::FavoriteClear3 => {}
                KbKey::FavoriteRecall0 => {}
                KbKey::FavoriteRecall1 => {}
                KbKey::FavoriteRecall2 => {}
                KbKey::FavoriteRecall3 => {}
                KbKey::FavoriteStore0 => {}
                KbKey::FavoriteStore1 => {}
                KbKey::FavoriteStore2 => {}
                KbKey::FavoriteStore3 => {}
                KbKey::Guide => {}
                KbKey::GuideNextDay => {}
                KbKey::GuidePreviousDay => {}
                KbKey::Info => {}
                KbKey::InstantReplay => {}
                KbKey::Link => {}
                KbKey::ListProgram => {}
                KbKey::LiveContent => {}
                KbKey::Lock => {}
                KbKey::MediaApps => {}
                KbKey::MediaAudioTrack => {}
                KbKey::MediaLast => {}
                KbKey::MediaSkipBackward => {}
                KbKey::MediaSkipForward => {}
                KbKey::MediaStepBackward => {}
                KbKey::MediaStepForward => {}
                KbKey::MediaTopMenu => {}
                KbKey::NavigateIn => {}
                KbKey::NavigateNext => {}
                KbKey::NavigateOut => {}
                KbKey::NavigatePrevious => {}
                KbKey::NextFavoriteChannel => {}
                KbKey::NextUserProfile => {}
                KbKey::OnDemand => {}
                KbKey::Pairing => {}
                KbKey::PinPDown => {}
                KbKey::PinPMove => {}
                KbKey::PinPToggle => {}
                KbKey::PinPUp => {}
                KbKey::PlaySpeedDown => {}
                KbKey::PlaySpeedReset => {}
                KbKey::PlaySpeedUp => {}
                KbKey::RandomToggle => {}
                KbKey::RcLowBattery => {}
                KbKey::RecordSpeedNext => {}
                KbKey::RfBypass => {}
                KbKey::ScanChannelsToggle => {}
                KbKey::ScreenModeNext => {}
                KbKey::Settings => {}
                KbKey::SplitScreenToggle => {}
                KbKey::STBInput => {}
                KbKey::STBPower => {}
                KbKey::Subtitle => {}
                KbKey::Teletext => {}
                KbKey::VideoModeNext => {}
                KbKey::Wink => {}
                KbKey::ZoomToggle => {}
                KbKey::__Nonexhaustive => {}
            },
            SystemEvent::KeyUp(_key_event) => {
                // TODO
            }
            SystemEvent::MouseDown(mouse_event) => {
                // The mouse is down within this text input.
                if self.rectangle.contains(mouse_event.pos) {
                    // This widget was not focused.
                    if !self.has_focus {
                        // Give it focus.
                        self.has_focus = true;

                        // Tell the widget manager about the change of focus.
                        widget_events.push(WidgetEvent::GotFocus(self.widget_id))
                    }
                }
                // The mouse is down outside of this text input.
                else {
                    // This widget was focused.
                    if self.has_focus {
                        // Tell the widget manager about the change of focus.
                        widget_events.push(WidgetEvent::LostFocus(self.widget_id))
                    }

                    self.has_focus = false;
                }
            }
            SystemEvent::MouseMove(_) => {}
            SystemEvent::MouseUp(_) => {}
        }
    }

    fn paint(&self, piet: &mut Piet, region: &Region) -> Result<(), Error> {
        // Paint the input field itself.
        {
            let shape = RoundedRect::from_rect(self.rectangle, self.corner_radius);

            // Fill.
            if let Some(fill_brush) = &self.fill_brush {
                piet.fill(shape, fill_brush);
            }

            // Stroke.
            if self.has_focus {
                piet.stroke(shape, &self.stroke_brush_focused, self.stroke_width);
            } else {
                piet.stroke(shape, &self.stroke_brush, self.stroke_width);
            }
        }

        // Paint the text.
        self.text_widget.paint(piet, region)?;

        // Render debug hints.
        if self.debug_rendering {
            piet.stroke(
                self.rectangle,
                &self.debug_rendering_stroke_brush,
                self.debug_rendering_stroke_width,
            );
        }

        Ok(())
    }

    fn set_origin(&mut self, origin: Point) {
        self.rectangle = self.rectangle.with_origin(origin);

        // Layout the child.
        self.layout_child();
    }

    fn widget_id(&self) -> &WidgetId {
        &self.widget_id
    }
}
