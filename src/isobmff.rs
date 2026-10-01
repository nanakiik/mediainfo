pub struct Box<'a> {
    pub size: u32,
    pub boxtype: [u8; 4],
    pub data: &'a [u8],
}
pub fn read_box(buf: &[u8]) -> Vec<Box<'_>> {
    let len = buf.len();
    let mut pos = 0;
    let mut boxs = Vec::new();
    loop {
        if pos >= len {
            break;
        }
        let size = u32::from_be_bytes(buf[pos..pos + 4].try_into().unwrap());
        let boxtype = buf[pos + 4..pos + 8].try_into().unwrap();
        if size == 0 {}
        let data = if size == 0 {
            &[]
        } else {
            &buf[pos + 8..pos + size as usize]
        };
        boxs.push(Box {
            size,
            boxtype,
            data,
        });
        pos += size as usize;
    }
    boxs
}
