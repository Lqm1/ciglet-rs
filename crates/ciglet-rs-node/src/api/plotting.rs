use crate::{
    array::Array2,
    plotting::{Color, Commands},
};
use napi::Error as BindingError;
#[derive(Clone, Copy)]
#[napi_derive::napi]
pub enum PlotColor {
    Blue = 0,
    Red = 1,
    Green = 2,
    Black = 3,
    Cyan = 4,
    Yellow = 5,
}
fn color(code: u32) -> Result<Color, BindingError> {
    match code {
        0 => Ok(Color::Blue),
        1 => Ok(Color::Red),
        2 => Ok(Color::Green),
        3 => Ok(Color::Black),
        4 => Ok(Color::Cyan),
        5 => Ok(Color::Yellow),
        _ => Err(napi::Error::from_reason("invalid plot color")),
    }
}
pub fn plot_color_from_character(value: &str) -> Result<u32, BindingError> {
    let mut characters = value.chars();
    let character = characters
        .next()
        .filter(|_| characters.next().is_none())
        .ok_or_else(|| napi::Error::from_reason("plot color requires one character"))?;
    Ok(match Color::from(character) {
        Color::Blue => 0,
        Color::Red => 1,
        Color::Green => 2,
        Color::Black => 3,
        Color::Cyan => 4,
        Color::Yellow => 5,
    })
}
/// Complete native command stream, with caller-editable byte storage.
pub struct PlotCommands {
    commands: Commands<Vec<u8>>,
}
impl PlotCommands {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            commands: Commands::new(bytes),
        }
    }
    pub fn bytes(&mut self) -> Vec<u8> {
        self.commands.writer_mut().clone()
    }
    pub fn set_bytes(&mut self, bytes: Vec<u8>) {
        *self.commands.writer_mut() = bytes;
    }
    pub fn append(&mut self, bytes: &[u8]) -> Result<(), BindingError> {
        let writer = self.commands.writer_mut();
        writer
            .try_reserve(bytes.len())
            .map_err(|_| napi::Error::from_reason("plot stream allocation failed"))?;
        writer.extend_from_slice(bytes);
        Ok(())
    }
    pub fn into_inner(self) -> Vec<u8> {
        self.commands.into_inner()
    }
    pub fn cloned(&mut self) -> Self {
        Self::new(self.commands.writer_mut().clone())
    }
    pub fn line_f32(
        &mut self,
        x: Option<Vec<f32>>,
        y: &[f32],
        code: u32,
    ) -> Result<(), BindingError> {
        self.commands
            .line(x.as_deref(), y, color(code)?)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
    pub fn line_f64(
        &mut self,
        x: Option<Vec<f64>>,
        y: &[f64],
        code: u32,
    ) -> Result<(), BindingError> {
        self.commands
            .line(x.as_deref(), y, color(code)?)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
    pub fn image_f32(
        &mut self,
        rows: usize,
        columns: usize,
        values: Vec<f32>,
    ) -> Result<(), BindingError> {
        let values = Array2::from_row_major(rows, columns, values)
            .map_err(|e| napi::Error::from_reason(e.0))?;
        self.commands
            .image(&values)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
    pub fn image_f64(
        &mut self,
        rows: usize,
        columns: usize,
        values: Vec<f64>,
    ) -> Result<(), BindingError> {
        let values = Array2::from_row_major(rows, columns, values)
            .map_err(|e| napi::Error::from_reason(e.0))?;
        self.commands
            .image(&values)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}
