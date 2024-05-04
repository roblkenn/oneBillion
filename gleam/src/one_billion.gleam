import gleam/io
import file_streams/read_text_stream

pub fn main() {
  let assert Ok(text_stream) = read_text_stream.open("../measurements.data")
  let assert Ok(line) = read_text_stream.read_line(text_stream)
  io.print(line)
}
