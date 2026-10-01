pub struct Chunk<'a> {
    pub four_cc: [u8; 4],
    pub size: u32,
    pub payload: &'a [u8],
}

pub fn read_chunk(buf: &[u8]) -> Vec<Chunk<'_>> {
    let len = buf.len();
    let mut pos = 0;
    let mut chunk;
    let mut chunks = Vec::new();
    let mut four_cc;
    let mut size;
    let mut payload;
    loop {
        if pos >= len {
            break;
        }
        four_cc = buf[pos..pos + 4].try_into().unwrap();
        size = u32::from_le_bytes(buf[pos + 4..pos + 8].try_into().unwrap());
        size = size + (size & 1);
        pos += 8;
        payload = &buf[pos..pos + size as usize];
        pos += size as usize;
        chunk = Chunk {
            four_cc,
            size,
            payload,
        };
        chunks.push(chunk);
    }
    chunks
}

pub fn read_wav(buf: &[u8]) {
    assert_eq!(&buf[..4], b"RIFF");
    let _len = u32::from_le_bytes(buf[4..8].try_into().unwrap());

    assert_eq!(&buf[8..12], b"WAVE");
    let chunks = read_chunk(&buf[12..]);
    for chunk in chunks {
        let data = chunk.payload;
        let four_cc = str::from_utf8(&chunk.four_cc).unwrap_or("<invalid>");
        println!("chunk(\"{}\"),size:{}", four_cc, chunk.size);
        match &chunk.four_cc {
            b"fmt " => {
                let audio_format = u16::from_le_bytes(data[0..2].try_into().unwrap());
                let channels = u16::from_le_bytes(data[2..4].try_into().unwrap());
                let sample_rate = u32::from_le_bytes(data[4..8].try_into().unwrap());
                let data_rate = u32::from_le_bytes(data[8..12].try_into().unwrap());
                let block_algin = u16::from_le_bytes(data[12..14].try_into().unwrap());
                let bits_per_sample = u16::from_le_bytes(data[14..16].try_into().unwrap());
                let str = match audio_format {
                    1 => "PCM",
                    3 => "IEEE float",
                    _ => unreachable!(),
                };
                println!("  audio format:{str}");
                println!("  channels:{channels} ");
                println!("  sample_rate:{sample_rate}");
                println!("  data_rate:{data_rate}(block_algin * sample_rate)");
                println!("  block_algin:{block_algin}(bits_per_sample * channels / 8)");
                println!("  bits_per_sample:{bits_per_sample}");
            }
            b"data" => {}
            _ => {}
        }
    }
}

#[test]
fn main() {
    let buf = include_bytes!("../1.wav");
    println!("len={}", buf.len());
    read_wav(buf);
}
