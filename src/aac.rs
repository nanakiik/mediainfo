/// 14496-3
use crate::bit_read::BitReader;

pub fn read_adts(buf: &[u8]) {
    let mut stream = BitReader::new(&buf);
    println!("Main(Main profile)");
    println!("LC(Low Complexity profile)");
    println!("SSR(Scalable Sampling Rate profile)");
    println!("LTP(Long Term Prediction profile)");
    let mut i = 0;
    let mut pos = 0;
    let len = stream.data.len();
    while pos < len {
        // adts_fixed_header
        let syncword: u16 = stream.read_bits(12);
        assert_eq!(syncword, 0xFFF);
        let id = stream.read_bit() != 0;
        let layer: u8 = stream.read_bits(2);
        assert_eq!(layer, 0);
        let protection_absent = stream.read_bit() == 0;
        let profile_object_type: u8 = stream.read_bits(2);
        let sampling_frequency_index: u8 = stream.read_bits(4);
        let private_bit = stream.read_bit();
        assert_eq!(private_bit, 0);
        let channel_configuration: u8 = stream.read_bits(3);
        let _original_copy = stream.read_bit() != 0;
        let _home = stream.read_bit() != 0;

        // adts_variable_header
        let _copyright_identification_bit = stream.read_bit() != 0;
        let _copyright_identification_start = stream.read_bit() != 0;

        let aac_frame_length: u16 = stream.read_bits(13);
        let _adts_buffer_fullness: u16 = stream.read_bits(11);
        let number_of_raw_data_blocks: u8 = stream.read_bits(2);

        let mpeg_identifier = if id { "MPEG-2" } else { "MPEG-4" };

        let profile = match profile_object_type {
            0 => "Main",
            1 => "LC",
            2 => "SSR",
            3 => "LTP",
            _ => unreachable!("overflow"),
        };
        let sample_rate = match sampling_frequency_index {
            0x0 => 96000,
            0x1 => 88200,
            0x2 => 64000,
            0x3 => 48000,
            0x4 => 44100,
            0x5 => 32000,
            0x6 => 24000,
            0x7 => 22050,
            0x8 => 16000,
            0x9 => 12000,
            0xa => 11025,
            0xb => 8000,
            0xc => 7350,
            0xd | 0xe => 0, // reserved
            0xf => 0,       // escape value
            _ => unreachable!("overflow"),
        };
        let channel = match channel_configuration {
            c @ 0..7 => c,
            7 => 8,
            _ => unreachable!("reserved"),
        };

        print!("frame{i:<5} len={aac_frame_length:<4}");
        print!(" id={mpeg_identifier} profile={profile:<4}");
        print!(" sample_rate={sample_rate} channel={channel}");
        println!(" blocks={number_of_raw_data_blocks} has_crc={protection_absent}");
        stream.byte_pos = pos + aac_frame_length as usize;
        stream.bit_offset = 0;
        pos += aac_frame_length as usize;
        i += 1;
    }
}

#[test]
fn main() {
    let buf = include_bytes!("../1.aac");
    println!("len={}", buf.len());
    read_adts(buf);
}
