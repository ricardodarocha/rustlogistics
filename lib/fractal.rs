//! FRACTAL Framework Color Terminal Adjustment Layout
//! Criado por Ricardo da Rocha – versão Rust 2025

use std::fmt;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

// =======================================================
// Estruturas básicas de cor e paleta
// =======================================================
#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn ansi_bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn ansi_fg(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Paleta {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub text: Color,
}

impl Paleta {
    pub fn new(primary: Color, secondary: Color, surface: Color, text: Color) -> Self {
        Self { primary, secondary, surface, text }
    }
}

// =======================================================
// Constantes ANSI
// =======================================================
pub const RESET: &str     = "\x1b[0m";
pub const TITLE: &str     = "\x1b[1m";
pub const NEGRITO: &str   = "\x1b[2m";
pub const ITALICO: &str   = "\x1b[3m";
pub const SUBLINHA: &str  = "\x1b[4m";
pub const LIMPA: &str     = "\x1b[2J";

pub fn limpar_tela() {
    print!("{LIMPA}");
    let _ = io::stdout().flush();
}

pub fn cursorxy(x: u16, y: u16) {
    print!("\x1b[{};{}H", y, x);
}

pub fn n() {
    println!("{RESET}");
}

// =======================================================
// Controle de tema global
// =======================================================
static ANSI_ENABLED: AtomicBool = AtomicBool::new(false);
static mut GLOBAL_COLOR: Option<Paleta> = None;

pub fn enable_ansi() {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Console::{
            GetConsoleMode, SetConsoleMode, GetStdHandle,
            ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE
        };
        unsafe {
            let h_out = GetStdHandle(STD_OUTPUT_HANDLE);
            if h_out == 0 { return; }
            let mut mode: u32 = 0;
            if GetConsoleMode(h_out, &mut mode) == 0 { return; }
            SetConsoleMode(h_out, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
    ANSI_ENABLED.store(true, Ordering::SeqCst);
}

pub fn use_global_color(p: Paleta) -> Paleta {
    enable_ansi();
    unsafe { GLOBAL_COLOR = Some(p); }
    p
}

pub fn global_color() -> Option<Paleta> {
    unsafe { GLOBAL_COLOR }
}

// =======================================================
// Paletas padrão
// =======================================================
pub const TEXT_DISABLED: Color = Color::new(55, 71, 79);
pub const TEXT: Color          = Color::new(97, 90, 53);

pub const WHITE: Color         = Color::new(255, 255, 255);
pub const LIGHT: Color         = Color::new(245, 245, 245);
pub const GRAY: Color          = Color::new(33, 33, 33);
pub const BLACK: Color         = Color::new(0, 0, 0);

pub const PALETA_SKY: Paleta = Paleta {
    primary:   Color::new(102, 217, 239),
    secondary: Color::new(174, 129, 255),
    surface:   Color::new(161, 239, 228),
    text:      TEXT,
};

pub const PALETA_MATERIAL: Paleta = Paleta {
    primary:   Color::new(255, 238, 140),
    secondary: Color::new(219, 204, 121),
    surface:   Color::new(138, 128, 76),
    text:      TEXT,
};

// =======================================================
// Funções utilitárias de layout
// =======================================================
pub fn offset(p: Paleta, width: usize, fillchar: char) {
    print!("{}", p.surface.ansi_bg());
    for _ in 0..width {
        print!("{}", fillchar);
    }
    print!("{RESET}");
}

pub fn paragrafo(p: Paleta, message: &str, width: usize) {
    let len = message.len().min(width);
    let pad_left = (width - len) / 2;
    let pad_right = width - len - pad_left;

    print!("{}{}", p.surface.ansi_bg(), p.text.ansi_fg());
    for _ in 0..pad_left {
        print!(" ");
    }
    print!("{}", &message[..len]);
    for _ in 0..pad_right {
        print!(" ");
    }
    println!("{RESET}");
}

pub fn painel(p: Paleta, x: u16, y: u16, width: usize, height: usize) {
    for i in 0..height {
        cursorxy(x, y + i as u16);
        offset(p, width, ' ');
        println!("{RESET}");
    }
}

/// Imprime uma linha zebrada alternando cores primária/secundária
pub fn print_zebra_full(index: usize, p: Paleta, message: &str, width: usize, dark_bg: Color, margin_left: usize) {
    let bg = if index % 2 == 0 { p.primary } else { p.secondary };
    let len = message.len().min(width);

    print!("{}", dark_bg.ansi_bg());
    for _ in 0..margin_left {
        print!(" ");
    }

    print!("{}{}", bg.ansi_bg(), p.text.ansi_fg());
    print!("{}", &message[..len]);
    for _ in 0..(width - len) {
        print!(" ");
    }

    print!("{}", dark_bg.ansi_bg());
    for _ in 0..margin_left {
        print!(" ");
    }

    println!("{RESET}");
}

// =======================================================
// Funções auxiliares
// =======================================================
pub fn pause() {
    #[cfg(windows)]
    {
        std::process::Command::new("cmd").arg("/C").arg("pause").status().ok();
    }
    #[cfg(not(windows))]
    {
        use std::io::{stdin, Read};
        let mut s = String::new();
        println!("Pressione ENTER para continuar...");
        let _ = stdin().read_line(&mut s);
    }
}

pub fn cls() {
    #[cfg(windows)]
    {
        std::process::Command::new("cmd").arg("/C").arg("cls").status().ok();
    }
    #[cfg(not(windows))]
    {
        limpar_tela();
    }
}
