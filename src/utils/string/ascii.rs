#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AsciiChar {
    Null = b'\x00',
    StartOfHeading = b'\x01',
    StartOfText = b'\x02',
    EndOfText = b'\x03',
    EndOfTransmission = b'\x04',
    Enquiry = b'\x05',
    Acknowledge = b'\x06',
    Bell = b'\x07',
    Backspace = b'\x08',
    HorizontalTab = b'\t', // Tabulación
    LineFeed = b'\n',      // Nueva línea (LF)
    VerticalTab = b'\x0B',
    FormFeed = b'\x0C',
    CarriageReturn = b'\r', // Retorno de carro (CR)
    ShiftOut = b'\x0E',
    ShiftIn = b'\x0F',
    DataLinkEscape = b'\x10',
    DeviceControl1 = b'\x11',
    DeviceControl2 = b'\x12',
    DeviceControl3 = b'\x13',
    DeviceControl4 = b'\x14',
    NegativeAcknowledge = b'\x15',
    SynchronousIdle = b'\x16',
    EndOfTransmissionBlock = b'\x17',
    Cancel = b'\x18',
    EndOfMedium = b'\x19',
    Substitute = b'\x1A',
    Escape = b'\x1B',
    FileSeparator = b'\x1C',
    GroupSeparator = b'\x1D',
    RecordSeparator = b'\x1E',
    UnitSeparator = b'\x1F',
    Space = b' ',
    ExclamationMark = b'!',
    DoubleQuote = b'"',
    Hash = b'#',
    Dollar = b'$',
    Percent = b'%',
    Ampersand = b'&',
    SingleQuote = b'\'',
    LeftParenthesis = b'(',
    RightParenthesis = b')',
    Asterisk = b'*',
    Plus = b'+',
    Comma = b',',
    Minus = b'-',
    Period = b'.',
    Slash = b'/',
    Digit0 = b'0',
    Digit1 = b'1',
    Digit2 = b'2',
    Digit3 = b'3',
    Digit4 = b'4',
    Digit5 = b'5',
    Digit6 = b'6',
    Digit7 = b'7',
    Digit8 = b'8',
    Digit9 = b'9',
    Colon = b':',
    Semicolon = b';',
    LessThan = b'<',
    Equal = b'=',
    GreaterThan = b'>',
    QuestionMark = b'?',
    AtSymbol = b'@',
    UppercaseA = b'A',
    UppercaseB = b'B',
    UppercaseC = b'C',
    UppercaseD = b'D',
    UppercaseE = b'E',
    UppercaseF = b'F',
    UppercaseG = b'G',
    UppercaseH = b'H',
    UppercaseI = b'I',
    UppercaseJ = b'J',
    UppercaseK = b'K',
    UppercaseL = b'L',
    UppercaseM = b'M',
    UppercaseN = b'N',
    UppercaseO = b'O',
    UppercaseP = b'P',
    UppercaseQ = b'Q',
    UppercaseR = b'R',
    UppercaseS = b'S',
    UppercaseT = b'T',
    UppercaseU = b'U',
    UppercaseV = b'V',
    UppercaseW = b'W',
    UppercaseX = b'X',
    UppercaseY = b'Y',
    UppercaseZ = b'Z',
    LeftBracket = b'[',
    Backslash = b'\\',
    RightBracket = b']',
    Caret = b'^',
    Underscore = b'_',
    GraveAccent = b'`',
    LowercaseA = b'a',
    LowercaseB = b'b',
    LowercaseC = b'c',
    LowercaseD = b'd',
    LowercaseE = b'e',
    LowercaseF = b'f',
    LowercaseG = b'g',
    LowercaseH = b'h',
    LowercaseI = b'i',
    LowercaseJ = b'j',
    LowercaseK = b'k',
    LowercaseL = b'l',
    LowercaseM = b'm',
    LowercaseN = b'n',
    LowercaseO = b'o',
    LowercaseP = b'p',
    LowercaseQ = b'q',
    LowercaseR = b'r',
    LowercaseS = b's',
    LowercaseT = b't',
    LowercaseU = b'u',
    LowercaseV = b'v',
    LowercaseW = b'w',
    LowercaseX = b'x',
    LowercaseY = b'y',
    LowercaseZ = b'z',
    LeftBrace = b'{',
    VerticalBar = b'|',
    RightBrace = b'}',
    Tilde = b'~',
    Delete = b'\x7F', // Código ASCII de DELETE
}
impl AsciiChar {
    pub fn to_byte(&self) -> u8 {
        *self as u8
    }
}
