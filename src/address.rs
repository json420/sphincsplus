use crate::params::*;
use crate::utils::*;

pub const SPX_ADDR_TYPE_WOTS: usize = 0;
pub const SPX_ADDR_TYPE_WOTSPK: usize = 1;
pub const SPX_ADDR_TYPE_HASHTREE: usize = 2;
pub const SPX_ADDR_TYPE_FORSTREE: usize = 3;
pub const SPX_ADDR_TYPE_FORSPK: usize = 4;
pub const SPX_ADDR_TYPE_WOTSPRF: usize = 5;
pub const SPX_ADDR_TYPE_FORSPRF: usize = 6;

// Bitshift value into u32 array, offset is in bytes
// Replaces the uint8_t addr cast in C reference implementation
fn set_addr(addr: &mut[u32], offset: usize, value: u32) 
{
  let set = value << (offset % 4 * 8);
  addr[offset / 4 ] = set; //TODO: Check 
}

fn get_addr(addr: &[u32], offset: usize) -> u32 {
  addr[offset / 4] >> offset % 4 * 8
}

// fn set_addr64(addr: &mut[u32], offset: usize, value: u64) 
// {
//   let set = value << (offset % 4 * 8);
//   addr[offset / 4 ] += set; //TODO: Check 
// }

/*
 * Specify which level of Merkle tree (the "layer") we're working on
 */
pub fn set_layer_addr(addr: &mut [u32], layer: u32)
{ 
  set_addr(addr, SPX_OFFSET_LAYER, layer)
}

/*
 * Specify which Merkle tree within the level (the "tree address") we're working on
 */
pub fn set_tree_addr(addr: &mut [u32], tree: u64)
{
// if (SPX_TREE_HEIGHT * (SPX_D - 1)) > 64 {
//   // compile_error!("Subtree addressing is currently limited to at most 2^64 trees");  
// }
  // let mut buf = [0u8; 32];
  // for i in 0..8 {
  //   buf[..i*4].copy_from_slice(&addr[i].to_ne_bytes());
  // }
  let be64 = tree.to_be_bytes();
  let mut tmp_addr = address_to_bytes(&addr);
  tmp_addr[SPX_OFFSET_TREE..SPX_OFFSET_TREE+8].copy_from_slice(&be64);
  bytes_to_address(addr, &tmp_addr);

    // for i in 0..8 {
    //   addr[i] = u32::from_ne_bytes(buf[i*4..(i+1)*4].try_into().expect("bytes into u32"));
    // }
}

/*
 * Specify the reason we'll use this address structure for, that is, what
 * hash will we compute with it.  This is used so that unrelated types of
 * hashes don't accidentally get the same address structure.  The type will be
 * one of the SPX_ADDR_TYPE constants
 */
pub fn set_type(addr: &mut [u32], addr_type: u32)
{
  let mut tmp_addr = address_to_bytes(&addr);
  tmp_addr[SPX_OFFSET_TYPE] = addr_type as u8;
  bytes_to_address(addr, &tmp_addr);
}

/*
 * Copy the layer and tree fields of the address structure.  This is used
 * when we're doing multiple types of hashes within the same Merkle tree
 */
pub fn copy_subtree_addr(out: &mut [u32], input: &mut [u32])
{
    out[..4].copy_from_slice(&input[..4]);
}

/* These functions are used for OTS addresses. */

/*
 * Specify which Merkle leaf we're working on; that is, which OTS keypair
 * we're talking about.
 */
pub fn set_keypair_addr(addr: &mut [u32], keypair: u32)
{
  let mut tmp_addr = address_to_bytes(&addr);
  /* We have > 256 OTS at the bottom of the Merkle tree; to specify */
  /* which one, we'd need to express it input two bytes */
  if SPX_FULL_HEIGHT/SPX_D > 8 {
    tmp_addr[SPX_OFFSET_KP_ADDR2] = (keypair >> 8) as u8; //TODO: Check
    
  }
  tmp_addr[SPX_OFFSET_KP_ADDR1] = keypair as u8;
  bytes_to_address(addr, &tmp_addr);
}

/*
 * Copy the layer, tree and keypair fields of the address structure.  This is
 * used when we're doing multiple things within the same OTS keypair
 */
pub fn copy_keypair_addr(out: &mut [u32], input: &mut [u32])
{ 
  // memcpy: SPX_OFFSET_TREE + 8 bytes
  out[..(SPX_OFFSET_TREE+8)/4].copy_from_slice(&input[..(SPX_OFFSET_TREE+8)/4]);
  if SPX_FULL_HEIGHT/SPX_D > 8 {
    let value = get_addr(input, SPX_OFFSET_KP_ADDR2);
    set_addr(out, SPX_OFFSET_KP_ADDR2, value)
  }
   let value = get_addr(input, SPX_OFFSET_KP_ADDR1);
   set_addr(out, SPX_OFFSET_KP_ADDR1, value)
    // out[SPX_OFFSET_KP_ADDR1] = input[SPX_OFFSET_KP_ADDR1];
}

/*
 * Specify which Merkle chain within the OTS we're working with
 * (the chain address)
 */
pub fn set_chain_addr(addr: &mut [u32], chain: u32)
{
  let mut tmp_addr = address_to_bytes(&addr);
  tmp_addr[SPX_OFFSET_CHAIN_ADDR] = chain as u8;
  bytes_to_address(addr, &tmp_addr);
}

/*
 * Specify where in the Merkle chain we are
* (the hash address)
 */
pub fn set_hash_addr(addr: &mut [u32], hash: u32)
{
  set_addr(addr, SPX_OFFSET_HASH_ADDR, hash);
  // let mut tmp_addr = address_to_bytes(&addr);
  // tmp_addr[SPX_OFFSET_HASH_ADDR] = hash as u8;
  // bytes_to_address(addr, &tmp_addr);
}

/* These functions are used for all hash tree addresses (including FORS). */

/*
 * Specify the height of the node in the Merkle/FORS tree we are in
 * (the tree height)
 */
pub fn set_tree_height(addr: &mut [u32], tree_height: u32)
{
  let mut tmp_addr = address_to_bytes(&addr);
  tmp_addr[SPX_OFFSET_TREE_HGT] = tree_height as u8;
  bytes_to_address(addr, &tmp_addr);
}

/*
 * Specify the distance from the left edge of the node in the Merkle/FORS tree
 * (the tree index)
 */
pub fn set_tree_index(addr: &mut [u32], tree_index: u32)
{
  let mut tmp_addr = address_to_bytes(&addr);
  u32_to_bytes(&mut tmp_addr[SPX_OFFSET_TREE_INDEX..], tree_index);
  bytes_to_address(addr, &tmp_addr);
}

fn bytes_to_address(addr: &mut[u32], bytes: &[u8])
{
  for i in 0..8 {
    let mut addr_i = [0u8; 4];
    addr_i.copy_from_slice(&bytes[i*4..i*4+4]);
    addr[i] = u32::from_ne_bytes(addr_i);
  }
}

pub fn address_to_bytes(addr: &[u32]) -> [u8; 32] 
{
  let mut tmp_addr = [0u8; 32];
  for i in 0..8 {
    tmp_addr[i*4..i*4+4].copy_from_slice(&addr[i].to_ne_bytes());
  }
  tmp_addr
}