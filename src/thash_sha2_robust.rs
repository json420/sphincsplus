use crate::thash::*;
use crate::address::*;
use crate::params::*;
use crate::utils::*;
use crate::sha2::*;

#if SPX_SHA512
pub fn thash_512(out: &mut[u8], input: &[u8], inblocks: u32
           ctx: &SpxCtx, addr: &mut [u32; 8]);
#endif

/**
 * Takes an array of inblocks concatenated arrays of SPX_N bytes.
 */
pub fn thash(out: &mut[u8], input: &[u8], inblocks: u32
           ctx: &SpxCtx, addr: &mut [u32; 8])
{
#if SPX_SHA512
    if (inblocks > 1) {
	thash_512(out, input, inblocks, ctx, addr);
        return;
    }
#endif
    let mut outbuf = [0u8; SPX_SHA256_OUTPUT_BYTES];
    SPX_VLA(uint8_t, bitmask, inblocks * SPX_N);
    SPX_VLA(uint8_t, buf, SPX_N + SPX_SHA256_OUTPUT_BYTES + inblocks*SPX_N);
    let mut sha2_state = [0u8; 40]
;
    memcpy(buf, ctx.pub_seed, SPX_N);
    &buf[SPX_N..].copy_from_slice(&addr[..SPX_SHA256_ADDR_BYTES]);
    mgf1_256(bitmask, inblocks * SPX_N, buf, SPX_N + SPX_SHA256_ADDR_BYTES);

    /* Retrieve precomputed state containing pub_seed */
    memcpy(sha2_state, ctx.state_seeded, 40 * sizeof(uint8_t));

    for (i = 0; i < inblocks * SPX_N; i++) {
        buf[SPX_N + SPX_SHA256_ADDR_BYTES + i] = input[i] ^ bitmask[i];
    }

    sha256_inc_finalize(outbuf, sha2_state, buf + SPX_N,
                        SPX_SHA256_ADDR_BYTES + inblocks*SPX_N);
    &out[..SPX_N].copy_from_slice(&outbuf[..SPX_N]);
}

#if SPX_SHA512
pub fn thash_512(out: &mut[u8], input: &[u8], inblocks: u32
           ctx: &SpxCtx, addr: &mut [u32; 8])
{
    let mut outbuf = [0u8; SPX_SHA512_OUTPUT_BYTES];
    SPX_VLA(uint8_t, bitmask, inblocks * SPX_N);
    SPX_VLA(uint8_t, buf, SPX_N + SPX_SHA256_ADDR_BYTES + inblocks*SPX_N);
    let mut sha2_state = [0u8; 72]

    memcpy(buf, ctx.pub_seed, SPX_N);
    &buf[SPX_N..].copy_from_slice(&addr[..SPX_SHA256_ADDR_BYTES]);
    mgf1_512(bitmask, inblocks * SPX_N, buf, SPX_N + SPX_SHA256_ADDR_BYTES);

    /* Retrieve precomputed state containing pub_seed */
    memcpy(sha2_state, ctx.state_seeded_512, 72 * sizeof(uint8_t));

    for (i = 0; i < inblocks * SPX_N; i++) {
        buf[SPX_N + SPX_SHA256_ADDR_BYTES + i] = input[i] ^ bitmask[i];
    }

    sha512_inc_finalize(outbuf, sha2_state, buf + SPX_N,
                        SPX_SHA256_ADDR_BYTES + inblocks*SPX_N);
    &out[..SPX_N].copy_from_slice(&outbuf[..SPX_N]);
}
#endif
