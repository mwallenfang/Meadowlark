pub enum IconType {
    ArrowDown,
    ArrowDownFilled,
    Automation,
    Cursor,
    Dropdown,
    DrumSequencer,
    Eraser,
    FileHierarchy,
    Folder,
    Menu,
    HatMinus,
    HatPlus,
    Hierarchy,
    Home,
    Loop,
    Mixer,
    Pencil,
    Piano,
    MarkerLeft,
    MarkerRight,
    Play,
    Plug,
    Plus,
    Quantize,
    QuantizeBolt,
    Record,
    Sample,
    Cut,
    Search,
    Stop,
    Grid,
    Stack,
    Terminal,
    Tools,
    ZoomFrame,
    ZoomFit,
    ZoomIn,
    ZoomOut,
}

impl Into<&'static str> for IconType {
    fn into(self) -> &'static str {
        match self {
            IconType::ArrowDown => "\u{0041}",
            IconType::ArrowDownFilled => "\u{0042}",
            IconType::Automation => "\u{0043}",
            IconType::Cursor => "\u{0044}",
            IconType::Dropdown => "\u{0045}",
            IconType::DrumSequencer => "\u{0046}",
            IconType::Eraser => "\u{0047}",
            IconType::Folder => "\u{0048}",
            IconType::Menu => "\u{0049}",
            IconType::HatMinus => "\u{004A}",
            IconType::HatPlus => "\u{004B}",
            IconType::Hierarchy => "\u{004C}",
            IconType::FileHierarchy => "\u{004D}",
            IconType::Home => "\u{004E}",
            IconType::Loop => "\u{004F}",
            IconType::Mixer => "\u{0050}",
            IconType::Pencil => "\u{0051}",
            IconType::Piano => "\u{0052}",
            IconType::MarkerLeft => "\u{0053}",
            IconType::MarkerRight => "\u{0054}",
            IconType::Play => "\u{0055}",
            IconType::Plug => "\u{0056}",
            IconType::Plus => "\u{0057}",
            IconType::Quantize => "\u{0058}",
            IconType::QuantizeBolt => "\u{0059}",
            IconType::Record => "\u{005A}",
            IconType::Sample => "\u{0061}",
            IconType::Cut => "\u{0062}",
            IconType::Search => "\u{0063}",
            IconType::Stop => "\u{0064}",
            IconType::Grid => "\u{0065}",
            IconType::Stack => "\u{0066}",
            IconType::Terminal => "\u{0067}",
            IconType::Tools => "\u{0068}",
            IconType::ZoomFrame => "\u{0069}",
            IconType::ZoomFit => "\u{006A}",
            IconType::ZoomIn => "\u{006B}",
            IconType::ZoomOut => "\u{006C}",
        }
    }
}
