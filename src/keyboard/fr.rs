pub static KMAP: [&str; 69] = [
    "NULL",
    "escape",
    "&",
    "é",
    "\"",
    "'",
    "(",
    "§",
    "è",
    "!",
    "ç",
    "à",
    ")",
    "-",
    "DEL",
    "TAB",
    "a",
    "z",
    "e",
    "r",
    "t",
    "y",
    "u",
    "i",
    "o",
    "p",
    "^",
    "$",
    "ENTER",
    "CTRL",
    "q",
    "s",
    "d",
    "f",
    "g",
    "h",
    "j",
    "k",
    "l",
    "m",
    "ù",
    "<",
    "SHIFT",
    "<",
    "w",
    "x",
    "c",
    "v",
    "b",
    "n",
    ",",
    ";",
    ":",
    "=",
    "RSHIFT",
    "",
    "OPTION",
    " ",
    "SHIFT_LOCK",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
];

pub static KMAP_SHIFT: [&str; 69] = [
    "NULL",
    "escape",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    "°",
    "_",
    "DEL",
    "TAB",
    "A",
    "Z",
    "E",
    "R",
    "T",
    "Y",
    "U",
    "I",
    "O",
    "P",
    "¨",
    "*",
    "ENTER",
    "CTRL",
    "Q",
    "S",
    "D",
    "F",
    "G",
    "H",
    "J",
    "K",
    "L",
    "M",
    "%",
    "£",
    "SHIFT",
    ">",
    "W",
    "X",
    "C",
    "V",
    "B",
    "N",
    "?",
    ".",
    "/",
    "+",
    "RSHIFT",
    "",
    "",
    " ",
    "SHIFT_LOCK",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
];

pub enum Kvalue {
    Null,
    Esc,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _0,
    Symbole0,
    Symbole1,
    Del,
    Tab,
    A,
    Z,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    Symbole2,
    Symbole3,
    Enter,
    Ctrl,
    Q,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    M,
    Symbole4,
    Symbole5,
    LShift,
    Symbole6,
    W,
    X,
    C,
    V,
    B,
    N,
    Symbole7,
    Symbole8,
    Symbole9,
    Symbole10,
    Rshift,
    Symbole11,
    Option,
    Space,
    ShiftLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
}
