const MD2_BLOCK_SIZE: usize = 16;
const PI_SUBST: [u8; 256] = [
    41, 46, 67, 201, 162, 216, 124, 1, 61, 54, 84, 161, 236, 240, 6, 19, 98, 167, 5, 243, 192, 199,
    115, 140, 152, 147, 43, 217, 188, 76, 130, 202, 30, 155, 87, 60, 253, 212, 224, 22, 103, 66,
    111, 24, 138, 23, 229, 18, 190, 78, 196, 214, 218, 158, 222, 73, 160, 251, 245, 142, 187, 47,
    238, 122, 169, 104, 121, 145, 21, 178, 7, 63, 148, 194, 16, 137, 11, 34, 95, 33, 128, 127, 93,
    154, 90, 144, 50, 39, 53, 62, 204, 231, 191, 247, 151, 3, 255, 25, 48, 179, 72, 165, 181, 209,
    215, 94, 146, 42, 172, 86, 170, 198, 79, 184, 56, 210, 150, 164, 125, 182, 118, 252, 107, 226,
    156, 116, 4, 241, 69, 157, 112, 89, 100, 113, 135, 32, 134, 91, 207, 101, 230, 45, 168, 2, 27,
    96, 37, 173, 174, 176, 185, 246, 28, 70, 97, 105, 52, 64, 126, 15, 85, 71, 163, 35, 221, 81,
    175, 58, 195, 92, 249, 206, 186, 197, 234, 38, 44, 83, 13, 110, 133, 40, 132, 9, 211, 223, 205,
    244, 65, 129, 77, 82, 106, 220, 55, 200, 108, 193, 171, 250, 36, 225, 123, 8, 12, 189, 177, 74,
    120, 136, 149, 139, 227, 99, 232, 109, 233, 203, 213, 254, 59, 0, 29, 57, 242, 239, 183, 14,
    102, 88, 208, 228, 166, 119, 114, 248, 235, 117, 75, 10, 49, 68, 80, 180, 143, 237, 31, 26,
    219, 153, 141, 51, 159, 17, 131, 20,
];

// Append Padding Bytes
fn padding(input: &[u8]) -> Vec<u8> {
    let mut input_vec = input.to_vec();
    let pad_size = MD2_BLOCK_SIZE - input.len() % MD2_BLOCK_SIZE;
    let mut pad = vec![pad_size as u8; pad_size];
    input_vec.append(&mut pad);
    input_vec
}

fn append_checksum(input: &mut Vec<u8>) -> &mut Vec<u8> {
    let len = input.len();
    // Clear checksum
    let mut cs: Vec<u8> = std::iter::repeat(0u8).take(16).collect();
    assert_eq!(16, cs.len());
    let mut last = 0;
    for i in 0..len / MD2_BLOCK_SIZE {
        for j in 0..MD2_BLOCK_SIZE {
            let c = input[i * MD2_BLOCK_SIZE + j];
            let val = PI_SUBST[(c ^ last) as usize];
            cs[j] = val;
            last = cs[j];
        }
    }
    input.append(&mut cs);
    input
}

pub fn digest(input: &[u8]) -> [u8; 16] {
    // Step 1. Append Padding Bytes
    let mut input = padding(input);

    // Step 2. Append Checksum
    let input_mut: &mut Vec<u8> = append_checksum(&mut input);

    // Step 3. Initialize MD Buffer
    let mut message_digest: Vec<u8> = std::iter::repeat(0u8).take(48).collect();
    assert_eq!(48, message_digest.len());

    // Step 4. Process Message in 16-Byte Blocks
    let blocks = input_mut.len() / MD2_BLOCK_SIZE;
    for i in 0..blocks {
        for j in 0..16 {
            message_digest[16 + j] = input_mut[i * 16 + j];
            message_digest[32 + j] = message_digest[j] ^ input_mut[i * 16 + j];
        }
        let mut t = 0u8;
        for j in 0..18 {
            for k in 0..48 {
                t = message_digest[k] ^ PI_SUBST[t as usize];
                message_digest[k] = t as u8;
            }
            // this is basically "Set t to (t+j) modulo 256".
            t = t.wrapping_add(j);
        }
    }

    // Step 5. Output
    let mut result = [0u8; 16];
    for i in 0..16 {
        result[i] = message_digest[i];
    }
    result
}

// The MD2 implementation from https://www.rfc-editor.org/rfc/rfc1319
#[cfg(test)]
mod test {
    use super::digest;

    fn format(hash: &[u8; 16]) -> String {
        let hex = hash
            .iter()
            .fold(String::new(), |a, &b| format!("{}{:02x}", a, b));
        hex
    }

    #[test]
    fn test() {
        assert_eq!(
            "8350e5a3e24c153df2275c9f80692773",
            format(&digest("".as_bytes()))
        );
        assert_eq!(
            "32ec01ec4a6dac72c0ab96fb34c0b5d1",
            format(&digest("a".as_bytes()))
        );
        assert_eq!(
            "da853b0d3f88d99b30283a69e6ded6bb",
            format(&digest("abc".as_bytes()))
        );
        assert_eq!(
            "ab4f496bfb2a530b219ff33031fe06b0",
            format(&digest("message digest".as_bytes()))
        );
    }
}
